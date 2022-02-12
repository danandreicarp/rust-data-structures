#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    let mut v = Vec::with_capacity(100);
    v.push("hello".to_string());
    v.push("goodbye".to_string());

    for i in 0..105 {
        v.push(i.to_string());
    }

    println!("v.len = {}, v.capacity = {}", v.len(), v.capacity());

    println!("Hello, {:?}!", ll);

    let s = " hello ";
    let p = s.trim();

    println!("s= {}, p = '{}'", s, p);

    let s = " hello ".to_string();
    let mut p = s.trim().to_string();

    p.push_str(" dear");

    println!("s = {}, p = '{}'", s, p);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = '{}'", ffstr);

    println!("chosen: '{}'", choose_str(1));
}

fn string_find_f<'a>(s: &'a str) -> &'a str {
    // for (n, x) in s.chars().enumerate() {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
