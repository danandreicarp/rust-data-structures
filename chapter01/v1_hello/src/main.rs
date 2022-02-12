#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age = {} has {} children",
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "dan".to_string(),
        age: 36,
        children: 1,
        fave_color: Color::Red("bordeaux".to_string()),
    };

    let c = Color::Red("hello".to_string());

    match c {
        Color::Red(s) => println!("It's red {}", s),
        Color::Green => println!("It's green"),
        Color::Blue => println!("It's blue"),
    }

    // println!("Hello, people, from {}", p.print());
    println!("Hello, people, from {:?}", p);
}
