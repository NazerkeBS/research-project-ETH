extern crate timely;

use std::collections::HashMap;

use timely::Data;
use timely::dataflow::{Stream, Scope};
use timely::dataflow::operators::{Inspect, Map, ToStream};
use timely::dataflow::operators::generic::operator::Operator;
use timely::dataflow::channels::pact::Pipeline;


trait Sum {
    fn sum(&self) -> Self;
}

impl<S: Scope> Sum for Stream<S, u64> {
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

fn main() {
	timely::example(|scope| {
		(0u64..100000).to_stream(scope)
            .sum()
            .map(|x| (x, x*2))
            .inspect_batch(|t, x| println!("t={:?} / d={:?}", t, x));
	})
}
