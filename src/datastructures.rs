use std::mem;

pub fn ds() {
    // structs();
    // enums();
    // option_of_t();
    // arrays();
    vectors();
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structs() {
    let p = Point { x: 3.0, y: 4.5 };
    let p2 = Point { x: 6.5, y: 1.5 };

    println!("point p is at ({}, {})", p.x, p.y);

    let myLine = Line { start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // Tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn enums() {
    // let c: Color = Color::Red;
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::CmykColor { black: 255, .. } => println!("black"), // technically same as above
        _ => println!("unknown"),
    }
}

fn option_of_t() {
    // Option<T>

    let x = 3.0;
    let mut y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };
    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by 0"),
    }

    if let Some(z) = result {
        println!("z = {}", z);
    }

    while let Some(z) = if y != 0.0 { Some(x / y) } else { None } {
        println!("z={}", z);
        y -= 1.0;
    }
}

fn arrays() {
    let mut a/*: [i32; 5]*/ = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 42;
    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("the array was modified");
    }

    let b = [1u16; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}]={}", i, j, mtx[i][j]);
            }
        }
    }
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(9);
    a[0] = 1993;
    // let idx:i32 = 0;
    // a[idx] = 2; // => error
    let idx: usize = 1;
    a[idx] = 3;
    println!("a = {:?}", a);
    match a.get(666) {
        Some(x) => println!("a[666]={}", x),
        None => println!("No element at index 666"),
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(42);
    println!("{:?}", a);
    let last_elem = a.pop();
    println!("{:?}", a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
