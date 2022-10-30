pub fn functions() {
    let x = 42;
    print_value(x);
    let mut z = x;
    increase(&mut z);
    print_value(z);
    print_value(product(3, 4));

    let myline = Line {
        start: Point { x: 3.0, y: 4.0 },
        end: Point { x: 5.0, y: 10.0 },
    };
    println!("{}", myline.len());

    closures();

    higher_order();

    traits();
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    let z = x * y;
    z
}

fn print_value(x: i32) {
    println!("value={}", x);
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx * dx + dy * dy).sqrt()
    }
}

fn say_hello() {
    println!("hello")
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };
    println!("{}", plus_one(1));
    let two = 2;
    let plus_two = |x| {
        let mut z = x;
        z += two;
        z
    };
    println!("{}", plus_two(1));

    let plus_three_local = |mut x: i32| x += 3;
    let f = 12;
    plus_three_local(f);
    println!("{}", f);

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("{}", f);
}

fn higher_order() {
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq
        }
    }

    println!("sum={}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("sum2={}", sum2);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn traits() {
    let c = Cat { name: "Misty" };
    c.talk();
    let h: Human = Animal::create("John");
    h.talk();

    let a = vec![1, 2, 3];
    println!("sum={}", a.sum());
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += x;
        }
        result
    }
}

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}
