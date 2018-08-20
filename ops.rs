extern crate timely;

use std::collections::HashMap;

use timely::Data;
use timely::dataflow::{Stream,Scope, InputHandle};
use timely::dataflow::operators::{Inspect, ToStream, Delay, Input, Probe};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::Operator;
use timely::progress::timestamp::RootTimestamp;
use std::ops::Add;
use std::ops::Mul;
use std::cmp::Ord;
use std::hash::Hash;
use std::cmp::Eq;

trait Aggregate<S: Scope, T> where T: Ord + Clone + Copy{
	fn max(&self) -> Stream<S, T>;
	fn min(&self) -> Stream<S, T>;	
}

trait TakeWhile <D: Data>{
	fn take_while <L: Fn(&D) -> bool + 'static> (&self, predicate: L) -> Self;
}

trait Sum<S: Scope, T> where T: Add<Output=T> + Copy + Clone {
	fn sum(&self) -> Stream<S,T>;
}

trait Product<S: Scope, T> where T: Mul<Output=T> + Copy + Clone  {
	fn product(&self) -> Stream<S,T>;
}

trait Mode<S: Scope, T: Data> where T: Eq + Hash{
	fn mode(&self) -> Stream <S,(T, u64)>;
}

trait Average <S: Scope, T> where T: Into<f64> {
  	fn avg (&self) -> Stream<S, f64>;
}

impl <S: Scope, T: Data> Mode<S, T> for Stream<S, T> where T: Eq + Hash{
	fn mode(&self) -> Stream <S,(T, u64)>{
		self.unary(Pipeline, "Mode", |_cap, _info| {

		    let mut map = HashMap::new();
			move |input, output|{		
				while let Some((time,data)) = input.next(){
					let mut session = output.session(&time);
					for datum in data.iter(){
						let count = map.entry(datum.clone()).or_insert(0);
						*count += 1;
					}
                    
                    let mut iterator = map.iter();
                    if let Some((mut max_key, mut max_cnt)) = iterator.next() {

						for (v, c) in iterator {
							if max_cnt < c {
								max_cnt = c;
								max_key = v;
							}
						}

						let tuple = (max_key.clone(), max_cnt.clone());
						session.give(tuple);
                    }

                    /*
                    if let Some((key,cnt)) = map.into_iter().max_by_key(|pair| pair.1) {
						session.give((key,cnt));                    	
                    }
                    */

				}
			}
		})
	}	
}

impl <S: Scope, T: Data> Sum<S, T> for Stream<S, T> where T: Add<Output=T> + Copy + Clone{
    fn sum(&self) -> Stream<S,T>{
    	let mut sum : Option<T> = None;
		self.unary(Pipeline, "Sum", |_cap, _info| 
			move |input, output|{
			input.for_each(|time,data|{
				for datum in data.iter(){
					if let Some(x) = sum {
						sum = Some(x + (*datum));
					}
					else {
						sum = Some(*datum);
					} 	
				}
				if let Some(x) = sum {
					output.session(&time).give(x);
				}
			});
		})
    }
}

impl <S: Scope, T: Data> Product<S, T> for Stream<S, T> where T: Mul<Output=T> + Copy + Clone {
	fn product(&self) -> Stream<S,T>{
		let mut prod : Option<T> = None;

		self.unary(Pipeline, "Product", |_cap, _info| move |input, output|{
			input.for_each(|time,data|{
				for datum in data.iter(){
					if let Some(x) = prod {
						prod = Some(x * (*datum));
					}
					else {
						prod = Some(*datum);
					} 	
				}
				if let Some(x) = prod {
					output.session(&time).give(x);
				}
			});
		})
	}
}

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

impl <S: Scope, T: Data> Average <S, T> for Stream <S, T> where T: Into<f64>{
	fn avg (&self) -> Stream<S, f64> {
		self.unary(Pipeline, "Average", |_cap, _info| move |input, output|{
			input.for_each(|time, data|{
				let mut sum : f64 = Default::default();
				let mut cnt: f64 = 0.0;
				for datum in data.drain(..){
					sum = sum + datum.into();
					cnt += 1.0;
				}
				let avg : f64 = sum / cnt;		
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
   		//let v = vec![1,2,2,2,3,4,4];
   		let v = vec![0; 20000];
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
		(1..6).to_stream(scope)
			   .product()
			   .inspect(|x| println!("product = {:?}", x));
	});

	//take until predicate holds
	timely::example(|scope|{
		(1..10).to_stream(scope)
			.take_while(|x| x < &4)
			.inspect(|x| println!("{:?}", x));
	});

	// initializes and runs a timely dataflow.
    timely::execute_from_args(std::env::args(), |worker| {

        let index = worker.index();
        let mut input = InputHandle::new();

        // create a new input, exchange data, and inspect its output
        let probe = worker.dataflow(|scope|
            scope.input_from(&mut input)
                 .mode()
                 .inspect_batch(move |t,xs| println!("worker {}:\thello {:?} @ {:?}", index, xs, t))
                 .probe()
        );

        let timer = std::time::Instant::now();

        // introduce data and watch!
        let v = vec![1,2,2,2,3,5,5];
        for (_round, value) in v.into_iter().enumerate() {
            if index == 0 {
                input.send(value);
            }

            let elapsed = timer.elapsed();
            let elapsed_ns = elapsed.as_secs() * 1000000000 + (elapsed.subsec_nanos() as u64);

            input.advance_to(elapsed_ns);
            while probe.less_than(input.time()) {
                worker.step();
            }
        }
    }).unwrap();

    timely::execute_from_args(std::env::args(), |worker|{
    	let index = worker.index();
    	let mut input = InputHandle::new();

    	let probe = worker.dataflow(|scope|
    		scope.input_from(&mut input)
    			 .sum()
    			 .inspect_batch(move |t, xs| println!("worker: {:?} computation: {:?} time: {:?}",index, xs, t))
    			 .probe()
	    ) ;
		let v = vec![1,2,3,4,5];
	    for (round, value) in v.into_iter().enumerate() {
		    if index == 0 {
		    	input.send(value);
		    }    	
	    	input.advance_to(round + 1);
            while probe.less_than(input.time()) {
                worker.step();
            }
	    }
    }).unwrap();


}