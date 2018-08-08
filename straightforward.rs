#[derive(Debug)]
struct User{
	name: String,
	age: u32,
	university: String,
	major: String
}

#[derive(Debug)]
struct Product{
	name: String,
	price: u64,
    height: f32,
    length: f32,
    width: f32
}


impl Product {  
	/// &self = instance of the struct: Product
	///	method
	fn volume (&self) -> f32 {
 		self.height * self.length * self.width
	}

	/// method with two parameters
	fn after_discount(&self, discount: f32) -> f32 {
		(self.price  as f32) * discount
	}
}



//struct tuple
struct Color(i32, i32, i32);

//struct tuple
struct Misc(String,u32,bool,f32);

//enums
enum TrafficLight{
	Red,
	Yellow,
	Green
}

use TrafficLight::{Red,Yellow}

fn main(){

	let user1 = User{name:"Lili Julia".to_string(), age: 23, university:"ETH".to_string(), major: "CS".to_string()};
    
    //to modify field of a struct => it should be mutable 
    let mut user2 = User{name:String::from("John Derk"), age: 26,university:String::from("ICL"), major: String::from("BI")};
    

	println!("{:?}", user1);
	println!("{:?}", user2);

	//change user2's major
	user2.major = String::from("Investment & Entrepreneurship");
    println!("After change: {:?}", user2);


    let black = Color(0,0,0);
    println!("{:?}", black.0);	

    let miscel = Misc("Type".to_string(), 23, true, 34.5);
    println!("{:?}", miscel.2);


    let v = vec![Product{name: "smartphone".to_string(), price: 400, height:1.1, length: 2.1, width: 1.4},
                 Product{name: "watch".to_string(), price: 250, height:1.2, length: 2.2, width: 1.56},
                 Product{name: "laptop".to_string(), price: 1900, height:1.3, length: 2.4, width: 1.2},
                 Product{name: "earpiece".to_string(), price: 90, height:1.4, length: 2.15, width: 1.24}
            ];
    let mut sum: u64 = 0;
    for elem in v {
    	sum += elem.price;
    	println!("volume: {:?}", elem.volume());
    	println!("with discount: {:?}", elem.after_discount(0.25));

    }
    println!("Total price: {:?}", sum);

    let red = Red;
    
}