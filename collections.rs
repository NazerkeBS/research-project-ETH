use std::collections::HashMap;

#[derive(Debug)]
struct Point <T, V>{
	x: T,
	y: V,
}

fn largest (list: &[i32]) -> i32 {
	let mut largest = list[0];

	for &elem in list.iter() {
		if largest < elem {
			largest = elem;
		}
	}
	largest
}


fn main(){

	//empty vector holds values of type i32
	let mut v : Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);
    println!("{:?}", v[1]);
    //println!("{:?}", v[11]); // crashing the program
    println!("{:?}",v.get(11) ); // returns None

    //using vec! macro
	let mut v1 = vec![1,2,3,4];
    
    for elem in &mut v1 {
    	*elem += 10;
    }

    for elem in &v1{
    	println!("{:?}", elem);
    }

 let s1 = String::from("tic");
 let s2 = String::from("tac");
 let s3 = String::from("toe");

 //concatenate s1, s2, s3 using format! macro
 let s = format!("{}-{}-{}", s1,s2,s3);
 println!("{:?}", s);

//use '+' for concatenation
let ss = s1  + "-" + &s2 + "-" + &s3; // first is self + strings
println!("{:?}", ss);
//Rust doesn't support string indexing
//println!("{:?}", ss[0]);

//to iterate over the string, use chars() method
for c in ss.chars(){
	println!("{:?}",c );
}

//bytes
for c in ss.bytes(){
	println!("{:?}",c );
}


let mut scores = HashMap::new();

scores.insert("apple".to_string(), 5);
scores.insert("banana".to_string(), 7);

let subjects = vec!["SE".to_string(), "BI".to_string()];
let grades = vec![100,105];

//another way of creating a hashmap
let map1: HashMap< _, _> = subjects.iter().zip(grades.iter()).collect();

for (s,g) in map1 {
	println!("{:?} {:?}",s, g);
}

//it only inserts if the key doesn't have a value
scores.entry(String::from("apple")).or_insert(10);
for s in scores {
	println!("{:?} ",s);
}

let text = String::from("Hello world from Naz world");
let mut map = HashMap::new();

for word in text.split_whitespace(){
	let count = map.entry(word).or_insert(0); // mutable reference to value
	*count += 1; // dereference then increment by 1
}
println!("{:?}", map );


let v = vec![12,3,22,12,88,9];
let max = largest(&v);
println!("largest: {:?}", max);

let both_integer = Point { x: 5, y: 10 };
let integer_and_float = Point{ x: 5, y: 4.0 }; 
println!("{:?}", integer_and_float);
}