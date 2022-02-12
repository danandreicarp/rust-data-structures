fn main() {
    println!("factorial of 30 is {}", factorial(30));
    println!("factorial_tail_rec of 30 is {}", factorial_tail_rec(30, 1));
}

fn factorial(n: u128) -> u128 {
    println!("factorial {}", n);
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

fn factorial_tail_rec(n: u128, r: u128) -> u128 {
    println!("factorial {} and {}", n, r);
    if n <= 1 {
        return r;
    }
    return factorial_tail_rec(n - 1, n * r);
}
