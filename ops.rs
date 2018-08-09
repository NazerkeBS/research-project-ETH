extern crate timely;

use std::collections::HashMap;


use timely::dataflow::{Stream,Scope};
use timely::dataflow::operators::{Inspect, ToStream};
use timely::dataflow::operators::generic::operator::Operator;
use timely::dataflow::channels::pact::Pipeline;

trait Aggregate {
	fn sum(&self) -> Self;
	fn max(&self) -> Self;
	fn min(&self) -> Self;
	fn mode(&self) -> Self;
}

impl <S: Scope> Aggregate for Stream<S, u64>{
	
	fn sum(&self) -> Stream<S, u64>{
		self.unary(Pipeline, "sum", |_default_cap, _info|{
			move |input, output|{
				while let Some((time,data)) = input.next() {
					//a single timestamp
					let mut session = output.session(&time);
					let mut sum = 0;
					for d in data.iter(){
						sum += d;
					}
					session.give(sum);
				}	
			}
		})
	}

	fn max(&self) -> Stream<S,u64> {
		self.unary(Pipeline, "maximum", |_default_cap, _info|{
			move |input, output|{
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
			}
		})
	}

	fn min(&self) -> Stream<S,u64>{
		self.unary(Pipeline, "minimum", |_default_cap, _info|{
			move |input, output|{
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
			}
		})
	}
	
	fn mode(&self) -> Stream<S,(u64)>{
		self.unary(Pipeline, "mode", |_default_cap, _info|{
			move |input,output|{
				while let Some((time,data)) = input.next(){
					let mut session = output.session(&time);
					let mut map = HashMap::new();
					for d in data.iter(){
						let count = map.entry(d).or_insert(0);
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
					session.give(value);

				}
			}
		})
	}

}

trait Average {
  	fn avg(&self) -> Self;
}

impl <S: Scope> Average for Stream<S, u64>{
	fn avg(&self) -> Stream <S, u64> {
		self.unary(Pipeline, "average", |_default_cap, _info|{
			move |input, output| {
				while let Some((time,data)) = input.next(){
					let mut session = output.session(&time);
					let mut sum = 0;
					let mut cnt  = 0;
					for d in data.iter(){
						sum += d;
						cnt += 1;
					}

					session.give(sum/ cnt);
				}
			}
		})
	}
}

fn main(){

	//sum up the stream of integers
	timely::example(|scope|{
		(0u64..10).to_stream(scope)
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

}