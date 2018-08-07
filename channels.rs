extern crate timely_communication;

fn main(){

	//give a number of workers as an arugment eg. cargo run --example channels -- -w4 && initialize the computation                                 
	let config = timely_communication::Configuration::from_args(std::env::args()).unwrap();
    
    //create a source of inactive loggers
    let logger = ::std::sync::Arc::new(|_|timely_communication::logging::BufferingLogger::new_inactive());

    // make a guard with configuration and logger and worker thread
    let guards = timely_communication::initialize(config, logger, |mut worker|{
    	println!("worker {:?} of {} started", worker.index(), worker.peers());

    //allocates pair of senders list and one receiver
    let (mut senders, mut receiver, _) = worker.allocate();

    for i in 0..worker.peers(){
    	senders[i].send(format!("hello, {}", i));
    	senders[i].done();
    }

    //no termination suppport
    let mut received = 0;
    while received < worker.peers() {
   		if let Some(message) = receiver.recv(){
   			println!("worker {} received {:?}", worker.index(), message);
   			received += 1;
   		}
    }
    	worker.index();
    });

//computation runs until guards are joined, it is a way of error handling in Rust
if let Ok(guards) = guards{
	for guard in guards.join(){
		println!("result: {:?}", guard);
	}
}
else {
	println!("error in computation");
}



}