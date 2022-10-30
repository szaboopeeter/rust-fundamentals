use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 2*8 bytes for the 2 f64
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 byte pointer on stack (+ 16 bytes on heap)
    
    let p3 = *p2; // dereference the boxed value
    println!("{}", p3.x); // 8 byte pointer on stack (+ 16 bytes on heap)
}
