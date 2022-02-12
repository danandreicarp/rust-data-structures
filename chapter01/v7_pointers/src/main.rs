#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    pub fn greet(&self) -> String {
        format!(
            "Hi, my name is {} and I'm {} years old!",
            self.name, self.age
        )
    }

    pub fn age_up(&mut self, n: i32) {
        self.age += n;
    }

    pub fn dropme(self) {}
}

fn main() {
    let mut p = Person::new("Dan Andrei".to_string(), 36);
    p.age_up(3);
    let s = p.greet();
    println!("{}", s);

    let a = get_age(&p);
    println!("person's age is {}", a);

    p.age_up(2);

    // p.dropme(); // won't be able to use p afterwards

    let s2 = p.greet();
    println!("again: {}", s2);
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}
