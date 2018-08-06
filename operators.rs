extern crate timely;
extern crate rand;

use timely::*;
use timely::dataflow::Scope;
use timely::dataflow::scopes::*;
use timely::dataflow::operators::{ToStream, Filter,Probe, Input, Inspect, Map, Concat, Concatenate,LoopVariable, Accumulate, Partition, Enter, Leave, ConnectLoop};
use timely::dataflow::operators::input::Handle;
use timely::progress::timestamp::RootTimestamp;
use timely::dataflow::operators::probe::Handle as probee;
use timely::dataflow::operators::Broadcast;
use rand::{Rng,SeedableRng, StdRng};

fn main(){
    
timely::example(|scope|{
		let stream1 = (0..10).to_stream(scope);

		let stream2 = (0..10).to_stream(scope)
			   .filter( |x| *x % 2  == 1)
			   .map_in_place(|x| *x *= 100)
			   .map(|x| x/2)
			   .flat_map(|x| (1..2))
			   .inspect(|x| println!("{:?}", x));

    //merge the contents of two streams
	stream1.concat(&stream2).inspect(|x| println!("{:?}", x));
	});
 	
 	timely::example(|scope|{
 		let streams = vec![(0..10).to_stream(scope),(10..20).to_stream(scope),(20..30).to_stream(scope)];
        
        //merge the contents of multiple streams
 		scope.concatenate(streams).inspect( |x| println!("{:?}", x));
 	});

 	timely::example(|scope|{
 		let stream = vec![1,2,3,4,4,4,2,3,3,2,2].to_stream(scope);

 	});
    println!();

    timely::example(|scope|{
    	let streams = (0..10).to_stream(scope)
    	 					 .partition(3,|x| (x % 2, x));
    	streams[0].inspect(|x| println!("{:?}", x));
    	streams[1].inspect(|x| println!("{:?}", x));
    	streams[2].inspect(|x| println!("{:?}", x));
    });


    timely::example( |scope|{
    	let stream = (0..10).to_stream(scope);

    	let result = scope.scoped::<usize, _, _>(|subscope|{
        	stream.enter(subscope) //extends timestamp with new timestamp
        		  .inspect_batch(|t,xs| println!("timestamp: {:?}, value: {:?}", t, xs))
        		  .leave(); // strips off the new timestamp
    	});
    });

    timely::example(|scope| {

        //create a loop cycles 20 times
        let (handle, stream) = scope.loop_variable(20,1);

       
        //circulate numbers. Collatz stepping each time
        (2..5).to_stream(scope)
               .concat(&stream)
               .map(|x| if x % 2 == 0 {x / 2} else { 3 * x + 1})
               .inspect(|x| println!("{:?}", x ))
               .filter(|x| *x != 1)
               .connect_loop(handle);
    });

   timely::example( |scope| {
       
        let input = (1..10).to_stream(scope);

        let result = scope.scoped::<usize, _, _>(|subscope|{
            let (handle, stream) = subscope.loop_variable(100, 1); 

            input.enter(subscope)
            .concat(&stream)
                 .map(|x| if x % 2 == 0 {x / 2} else {3 * x  + 1})
                 .inspect(|x| println!("{:?}", x ))
                 .filter(|x| *x != 1)
                 .connect_loop(handle);

        });
   });
   

//construct and execute a timely dataflow
timely::execute(Configuration::Thread, |worker|{
    //computation
    let mut probe = Handle::new();
    let mut input = worker.dataflow(|scope|{
    let (input, stream) = scope.new_input();
        stream.probe_with(&mut probe)
              .inspect_batch(|t,xs| println!("{:?} \t {:?}", t, xs ));
              input  //inspect_batch(timestamp, content of input)
    });


    //introduce input, advance computation
    for i in 0..10 {
        input.send(i);
        input.advance_to(i+1);
        //worker.step_while(probe.less_than(input.time())); 
                                                             // step_while() -> bool 
                                                             //probe.less_than(timestamp) -> bool
    }
}).unwrap();


// now() -> returns time zone in seconds {tv_sec, tv_nsec} 
let timer = std::time::Instant::now();
println!("{:?}",timer);


timely::execute(Configuration::Thread, |worker|{

    let mut input = Handle::new();
    worker.dataflow(|scope|{
        scope.input_from(&mut input) 
             .inspect(|x| println!(" {:?}", x));      
    });

    // introduce input
    for i in 1..10{
        input.send(i);
        input.advance_to(i + 1);
        worker.step();
    }

});

let seed: &[_] = &[1,2,3,4];
let mut rng: StdRng = SeedableRng::from_seed(seed);
println!("{:?}", rng.gen::<u64>()); //gen => return a random value of type Rand


timely::execute_from_args(std::env::args().skip(1), move |worker|{
    let index = worker.index(); // index of worker
    let peers = worker.peers(); // number of workers

    let mut input = worker.dataflow::<u64,_,_>(|scope| {
        let (input,stream) = scope.new_input();

        stream
            .broadcast() //broadcasts records to all workers
            .inspect(move |x| println!("worker: {} ->  {:?}", index, x ));
        
        input
    });

    for i in 0u64..10{
        if(i as usize) % peers == index {
            input.send(i);
        }
        input.advance_to(i + 1);
        worker.step();
    }
}).unwrap();

}