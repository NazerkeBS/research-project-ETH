extern crate timely;

use std::collections::HashMap;

use timely::Data;
use timely::dataflow::{Stream,Scope};
use timely::dataflow::operators::{Inspect, ToStream, Delay};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::Operator;
use timely::progress::timestamp::RootTimestamp;

trait Aggregate {
	fn max(&self) -> Self;
	fn min(&self) -> Self;
	fn product(&self) -> Self;
}

trait Sum{
	fn sum(&self) -> Self;
}

trait Mode<S: Scope> {
	fn mode(&self) -> Stream <S,(u64, u64)>;
}

trait Average <S: Scope>{
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

impl <S : Scope> Sum for Stream<S, u64>{
	fn sum(&self) -> Stream<S, u64> {
        let mut stash = HashMap::new();
        self.unary_notify(Pipeline, "Sum", vec![], move |input, output, notificator|{
            input.for_each(|time, data|{
                let sum = stash.entry(time.time().clone()).or_insert(0);
                    for datum in data.iter() {
                        *sum += datum;
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


impl <S: Scope> Aggregate for Stream<S, u64>{
	fn max(&self) -> Stream<S,u64> {
		self.unary(Pipeline, "Maximum", |_cap, _info| move |input, output|{
				while let Some((time, data)) = input.next(){
					let mut session = output.session(&time);
					let mut largest = data[0];
					for d in data.iter(){
						if d > &largest {
							largest = *d;
						} 
					}
					session.give(largest);
				}			
		})
	}

	fn min(&self) -> Stream<S,u64>{
		self.unary(Pipeline, "Minimum", |_cap, _info| move |input, output|{
				while let Some((time, data)) = input.next(){
					let mut session = output.session(&time);
					let mut smallest = data[0];
					for d in data.iter(){
						if &smallest > d {
							smallest = *d;
						}
					}
					session.give(smallest);
				}
			
		})
	}

	fn product(&self) -> Stream <S, u64>{
		self.unary(Pipeline, "Product", |_cap, _info| move |input, output|{
			input.for_each(|time,data|{
				let mut prod = 1;
				for datum in data.iter(){
					prod *= datum;	
				}
				output.session(&time).give(prod);
			});
		})
	}	

}


impl <S: Scope> Average <S> for Stream <S, u64>{
	fn avg (&self) -> Stream<S, f64> {
		self.unary(Pipeline, "Average", |_cap, _info| move |input, output|{
			input.for_each(|time, data|{
				let mut sum = 0;
				let mut cnt = 0;
				for datum in data.iter(){
					sum += datum;
					cnt += 1;
				}
				let avg = (sum as f64) / (cnt as f64);		
				output.session(&time).give(avg);
			});		 
		
		})
	}
}

fn main(){

	//sum up the stream of integers
	timely::example(|scope|{
		(0u64..100).to_stream(scope)
		.delay(|x,t| RootTimestamp::new(*x /10))
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
			   .product()
			   .inspect(|x| println!("{:?}", x));
	});

}