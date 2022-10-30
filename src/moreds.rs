pub fn ds() {
    // slices();
    // strings();
    // tuples();
    // pattern_matching();
    generics();
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    println!("{}", sum(&data[1..4]));
    modify(&mut data);
    println!("{:?}", data)
}
fn sum(slice: &[i32]) -> i32 {
    let mut s = 0;
    for i in 0..slice.len() {
        s += slice[i];
    }
    return s;
}
fn modify(slice: &mut [i32]) {
    for i in 0..slice.len() {
        slice[i] = 42;
    }
}

fn strings() {
    let s: &'static str = "hello there!"; // static-ly allocated. &str = string slice; sequence of utf-8 characters

    for c in s.chars().rev() {
        println!("{}", c);
    }
    // let h = s[0]; // ==> error
    if let Some(first_char) = s.chars().nth(0) {
        println!("first character is {}", first_char);
    }

    // heap-string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{:?}", letters);

    // &str <> string
    let u: &str = &letters;

    // concatenations
    // String + str
    let z = letters + "abc";
    let mut abc = String::from("hello world") + "!";
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    let (a, b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a, b);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=14 => "lots of", // z becomes a reference to the case // `..=` is the new version of `...`
        _ if (x % 2 == 0) => "a few couples of ",
        _ => "some",
    }
}

struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

fn generics() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: "hel", y: "lo" };
    // let c = Line { start: a, end: b }; // ==> no
    let c = Line { start: a, end: Point { x: 1, y: 2 } };
}
