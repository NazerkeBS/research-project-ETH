use std::collections::HashMap;

//Option
fn divide(num: f64, den: f64) -> Option<f64>{
	if den == 0.0 {
		None
	}else {
		Some(num/den)
	}
}

fn main(){

 let mut map = HashMap::new();
 map.insert("one", 1);
 map.insert("two", 2);
 map.insert("three", 3);

 //Entry API for setting, updating, removing keys & values
 //insert a key four with value 4
map.entry("four").or_insert(4);



//update all values
for (_,v) in &mut map {
	*v = *v + 1;
}
for (k, v) in &map {
	println!("key= {:?}, value={:?}",k,v);
}

let mut letters = HashMap::new();
//(char,frequency) in sentence
for ch in "a short treatise on fungi".chars() {
    let counter = letters.entry(ch).or_insert(0);
    *counter += 1;
}
for (k,v) in letters{
	println!("key= {:?} value= {:?}",k, v );
}

let num = 3.4;
let den = 0.0;
let result = divide(num,den);
println!("{:?}", result);


println!("{:?}",map.len() );


}