struct Rectangle {
 x: u32,
 y: u32,
}

struct Circle {
 r : f64,
}

trait Shape {
  fn area(&self) -> u32;
}
impl Shape for Rectangle {
    fn area(&self) -> u32{
        self.x * self.y
    }
}

impl Shape for Circle {
   fn area(&self) -> u32{
    (self.r*3.14*self.r) as u32
   }
}


fn main() {
    let c = Circle{r: 240.2};
    let rect = Rectangle{x: 5, y: 40};
    println!("Circle area: {} \n Rectangle area: {}", c.area(),rect.area());


}
