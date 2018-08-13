extern crate timely;

use std::collections::HashMap;

use timely::Data;
use timely::dataflow::{Stream,Scope};
use timely::dataflow::operators::{Inspect, ToStream, Delay};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::Operator;
use timely::progress::timestamp::RootTimestamp;
use std::ops::Add;
use std::ops::Mul;
use std::cmp::Ord;
use std::ops::Div;

trait Aggregate<S: Scope, T> where T: Ord + Clone + Copy{
	fn max(&self) -> Stream<S, T>;
	fn min(&self) -> Stream<S, T>;	
}

trait TakeWhile <D: Data>{
	fn take_while <L: Fn(&D) -> bool + 'static> (&self, predicate: L) -> Self;
}

trait Sum<S: Scope, T> where T: Add<Output=T> + Copy + Clone + Default {
	fn sum(&self) -> Stream<S,T>;
}
/*
trait Product<S: Scope, T> where T: Mul<Output=T> + Copy + Clone + Default {
	fn product(&self) -> Stream<S,T>;
}
*/

trait Mode<S: Scope> {
	fn mode(&self) -> Stream <S,(u64, u64)>;
}

trait Average <S: Scope, T> where T: Add<Output=T> + Copy + Clone + Default  + Div<Output=f64>{
  	fn avg (&self) -> Stream<S, f64>;
}

impl <S: Scope> Mode<S> for Stream<S, u64>{
	fn mode(&self) -> Stream <S,(u64, u64)>{
		self.unary(Pipeline, "Mode", |_cap, _info| move |input, output|{		
				while let Some((time,data)) = input.next(){
					let mut session = output.session(&time);
					let mut map = HashMap::new();
					for datum in data.iter(){
						let count = map.entry(datum).or_insert(0);
						*count += 1;
					}
                    
                    let mut max_cnt = 0;
                    let mut value = 0; 
					for (c, v) in map {
						if &max_cnt < c {
							max_cnt = *c;
							value = v;
						}
					}
					let tuple = (value, max_cnt);
					session.give(tuple);
				}
			
		})
	}	
}

impl <S: Scope, T: Data> Sum<S, T> for Stream<S, T> where T: Add<Output=T> + Copy + Clone + Default{
    fn sum(&self) -> Stream<S,T>{
        let mut stash = HashMap::new();
        self.unary_notify(Pipeline, "Sum", vec![], move |input, output, notificator|{
            input.for_each(|time, data|{
                let sum = stash.entry(time.time().clone()).or_insert(Default::default());
                    for datum in data.iter() {
                        *sum = *sum + *datum;
                    }
                notificator.notify_at(time.delayed(&time));        
            });

            notificator.for_each(|time,_,_| {
                if let Some(sum) = stash.remove(&time) {
                    output.session(&time).give(sum);
                }
            });
        })
    }
}
/*
impl <S: Scope, T: Data> Product<S, T> for Stream<S, T> where T: Mul<Output=T> + Copy + Clone + Default{
	fn product(&self) -> Stream<S,T>{
		self.unary(Pipeline, "Product", |_cap, _info| move |input, output|{
			input.for_each(|time,data|{
				let mut prod = 1;
				for datum in data.iter(){
					prod = prod * (*datum);	
				}
				output.session(&time).give(prod);
			});
		})
	}
}
*/

impl <S: Scope, D: Data> TakeWhile<D> for Stream<S, D>{
	fn take_while<L: Fn(&D) -> bool + 'static> (&self, predicate: L) -> Stream<S, D>{
		self.unary(Pipeline, "TakeWhile", |_cap, _info| move|input, output|{
			while let Some((time, data)) = input.next(){
			    for datum in data.drain(..){ // extract values
					if predicate(&datum){
						output.session(&time).give(datum);
					}else {
						break;
					}
				}	
			}

		})
	}
}

impl <S: Scope, T: Data> Aggregate<S,T> for Stream<S, T> where T: Ord + Clone + Copy{
	fn max(&self) -> Stream<S,T> {
		self.unary(Pipeline, "Maximum", |_cap, _info| move |input, output|{
				while let Some((time, data)) = input.next(){
					let mut session = output.session(&time);
					let mut largest = data[0];
					for d in data.drain(..){
						if d > largest {
							largest = d;
						} 
					}
					session.give(largest);
				}			
		})
	}

	fn min(&self) -> Stream<S,T>{
		self.unary(Pipeline, "Minimum", |_cap, _info| move |input, output|{
				while let Some((time, data)) = input.next(){
					let mut session = output.session(&time);
					let mut smallest = data[0];
					for d in data.drain(..){
						if smallest > d {
							smallest = d;
						}
					}
					session.give(smallest);
				}
			
		})
	}

}

impl <S: Scope, T: Data> Average <S, T> for Stream <S, T> where T: Add<Output = T> + Copy + Clone + Default + Div<Output=f64>{
	fn avg (&self) -> Stream<S, f64> {
		self.unary(Pipeline, "Average", |_cap, _info| move |input, output|{
			input.for_each(|time, data|{
				let mut sum : T = Default::default();
				let mut cnt: f64 = 0.0;
				for datum in data.drain(..){
					sum = sum + datum;
					cnt += 1.0;
				}
				let avg : f64 = f64::from(sum) / cnt;		
				output.session(&time).give(avg);
			});		 
		
		})
	}
}

fn main(){

	//sum up the stream of integers
	timely::example(|scope|{
		(0u64..100).to_stream(scope)
		.delay(|x,_t| RootTimestamp::new(*x /10))
		.sum()
		.inspect_batch(|t,x| println!("time ={:?} sum ={:?}", t, x));
	});
    
    //find the maximum
	timely::example(|scope|{
		(25..100).to_stream(scope)
		.max()
		.inspect_batch(|t,x| println!("time ={:?} maximum ={:?}", t,x ));
	});

	//find the minimum
	timely::example(|scope|{
		(3..10).to_stream(scope)
		.min()
		.inspect_batch(|t,x| println!("time = {:?}, minimum = {:?}", t, x));
	}); 
   
   	// mode of a vector 
 	timely::example(|scope|{
   		let v = vec![1,2,2,2,3,4,4];
   		v.to_stream(scope)
   		.mode()
   		.inspect(|x| println!("{:?}", x ));
   });

 	//avg
 	timely::example(|scope| {
 		(20..1000).to_stream(scope)
 				.avg()
 				.inspect(|x| println!("avg = {:?}", x));
 	});

 	//product
 	timely::example(|scope|{
		(1..10).to_stream(scope)
			   //.product()
			   .inspect(|x| println!("{:?}", x));
	});

	//take until predicate holds
	timely::example(|scope|{
		(1..10).to_stream(scope)
			.take_while(|x| x < &4)
			.inspect(|x| println!("{:?}", x));
	});

}