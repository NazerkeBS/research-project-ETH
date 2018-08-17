extern crate timely;
extern crate differential_dataflow;
use differential_dataflow::input::Input;

fn main() {

    timely::execute_from_args(std::env::args(), |worker|{
        
        let (mut handle, _probe) = worker.dataflow::<(),_,_>(|scope|{

            let (handle, data) = scope.new_collection_from(10..14); 
            //let (handle, data) = scope.new_collection();

            let probe = data.map(|x| x*2)
                            .inspect(|x| println!("{:?}", x ))
                            .probe();
            (handle, probe)
        });

        handle.insert(2);// adding +1
        handle.insert(3);//addding +1
        handle.insert(2);
        handle.remove(3); // removal -1
 

    }).unwrap();

    timely::example(|scope| {
        let data = scope.new_collection_from(1 .. 10).1;
        let odds = data.filter(|x| x % 2 == 1);
        let evens = data.filter(|x| x % 2 == 0);

        odds.negate()
            .concat(&data)
            .assert_eq(&evens);
    });
}
