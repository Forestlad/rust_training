// use std::collections::VecDeque;

fn fun(x: &mut i32) {
    *x += 1;
    print!("{}", x);
}

fn main() {
    let mut a = Box::new(5);
    println!("{:p}", &*a);
    let b = a;
    a = Box::new(7);
    println!("{:p}, {:p}", &*a, &*b);
}
