use std::mem;
mod control;
mod datastructures;
mod functions;
mod lifetime;
mod moreds;
mod sh;

extern crate rand; // see Cargo.toml
use rand::Rng;

fn main() {
    println!("Hello, world!");
    // numbers();
    // scopes();
    // operators();
    // constants();
    // sh::stack_and_heap();
    // control::control();
    // datastructures::ds();
    // moreds::ds();
    // functions::functions();
    lifetime::lifetime();
    lifetime::borrowing();
    let mut rng = rand::thread_rng();
    let b: i32 = rng.gen();
    println!("{}", b);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
// #[should_panic]
// #[ignore]
// #[cfg(tests)]
fn test_sum() {
    assert_eq!(3, sum(1, 2))
}

const MEANING_OF_LIFE: u8 = 42; // no fixed address, always inlined
static Z: i32 = 123;
static mut Z_MUT: i32 = 456;
fn constants() {
    println!("MEANING_OF_LIFE={}", MEANING_OF_LIFE);
    println!("Z={}", Z);
    // Z_MUT += 1; // does not compile, unsafe use of mutable static
    unsafe {
        Z_MUT += 1;
        println!("Z_MUT={}", Z_MUT);
    }
}

fn operators() {
    let mut a = 2 + 3 * 4; // -= += *= /= %=
    println!("{}", a);

    println!("remainder of {} / {} = {}", a, 3, (a % 3));
    let a_cubed = i32::pow(a, 3);
    println!(" {}^3 = {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);
    // bitwise: | & ^ (xor) ! (nor) >> <<
    // logical: < > ! <= >= == !=
}

fn scopes() {
    let a = 123;
    let a = 456; // shadowing

    {
        let b = 456;
        let a = 555; // will go out of scope
        println!("inner a={}", a);
    }

    println!("a={}", a);
    // println!("b={}", b); // not accessible
}

fn numbers() {
    let a: u8 = 123; // unsigned 8 bits
    println!("a = {}", a);
    // a = 456; ==> error, cannot be mutated

    let mut b: i8 = 0; // mutable, signed 8 bits
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);
    let mut c = 123456789; // 32 bit, signed
    println!("c = {}, size={} bytes", c, mem::size_of_val(&c));
    c = -1; // 32 bit, signed
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("d = {}, size={} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 64 bits, f64 (there's also f32)
    println!("e = {}, size={} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size={} bytes", g, mem::size_of_val(&g));
    let f = 4 > 0;
    println!("f = {}, size={} bytes", f, mem::size_of_val(&f));
}
