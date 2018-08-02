extern crate timely;


use timely::dataflow::operators::*;
use timely::dataflow::InputHandle;

fn main(){

	//print 0..9 in timely 
	timely::example(|scope|{
    	(0..10).to_stream(scope)
    	       .inspect(|x| println!("{:?}", x)); 
	});
     
     println!("\n");

	//print even numbers in timely
	timely::example(|scope|{
    	(0..100).to_stream(scope)
    			.filter(|x| x % 2 == 0)
    			.inspect(|x| println!("{:?}", x ));
	});
     
      println!("\n");

	//print numbers which are multiple of 3 and 5 in timely
	timely::example(|scope|{
		(0..100).to_stream(scope)
				.filter(|x| x % 3 == 0 && x % 5 == 0)
				.inspect(|x| println!("{:?}", x));
	});

	//compute cube of each number in timely
	timely::example(|scope|{
    	(0..100).to_stream(scope)
    			.map(|x| (x*x*x))
    			.inspect(|x| println!("{:?}", x));
	});

	//read from command arguments & print
    timely::execute_from_args(std::env::args(), |worker|{
    	let index = worker.index();
    	let mut input = InputHandle::new();
    	let probe = worker.dataflow(|scope|
    		scope.input_from(&mut input)
    			 .inspect(move |x| println!("worker {:?} : {:?}", index, x))
    			 .probe()
    		);

    	//introduce data
    	for i in 0..4{
    		input.send(i);
    		input.advance_to(i+1);
    	}
    }).unwrap();

}