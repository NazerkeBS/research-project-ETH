extern crate timely;
#[macro_use] extern crate abomonation_derive;
extern crate abomonation;

use timely::dataflow::operators::*;
use timely::dataflow::operators::aggregation::Aggregate;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_str(s: &String) -> u64 {
	let mut h = DefaultHasher::new();
	s.hash(&mut h);
	h.finish()
}

#[derive(Debug,Clone, Abomonation)]
struct Order{
	name: String,
	value: u32
}

#[derive(Debug)]
struct Ag {
	sum: u32,
    total_count: u32

}

impl Default for Ag {
	fn default() -> Ag {
		return Ag {
			sum: 0,
			total_count: 0
		};
	}
	
}

fn main() {
    
    timely::example(|scope| {
    	let v = vec![Order{name: "eth".to_string(), value: 14}, Order{name: "eth".to_string(), value: 26},Order{name: "amazon".to_string(), value: 10}];
        v.to_stream(scope)
        		.map(|order| (order.name, order.value))
        	    .aggregate(
        	   		|_name, val, agg: &mut Ag| { agg.sum += val; agg.total_count += 1; }, 
               		|name, agg: Ag| (name, agg.sum as f32 / agg.total_count as f32), 
               		|name| hash_str(name))
                .inspect(|x| println!("seen: {:?}", x));
    });
}
