#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut x = 34;
    let y = x;
    x += 5;

    println!("y= {}, x = {}", y, x);

    let mut p1 = Person {
        name: "Dan".to_string(),
        age: 36,
    };

    let p2 = p1.clone();

    p1.name.push_str(" Andrei");

    println!("p1 = {:?}, p2 = {:?}", p1, p2);

    let mut p1 = Point::new(3, 4);
    let p2 = p1;

    p1.x += 3;

    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}
