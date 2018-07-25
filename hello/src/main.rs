use std::thread;

fn main(){
let mut c = vec![];

for i in 0..10{
c.push(thread::spawn(move || { println!("thread {}", i)}));
}

for j in c {
j.join();
}

}
