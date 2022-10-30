pub fn lifetime() {
    let v = vec![1, 2, 3, 4];
    let v2 = v;
    // println!("{:?}", v); // ==> cannot, v is invalidated
    let foo = |v: Vec<i32>| ();
    foo(v2);
    // println!("{:?}", v2); // ==> cannot, v2 is invalidated
    // for primitives (values on stack), or implementors of i32 Copy trait it works, it performs a copy
    let u = 1;
    let u2 = u;
    println!("{:?}", u);

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };

    let v3 = vec![1, 2, 3, 4];
    let v4 = print_vector(v3);
}

pub fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
    };
    let v = vec![3, 2, 1];
    print_vector(&v);
    println!("{:?}", v); // still usable
    let mut a = 40;
    let b = &mut a;
    *b *= 2;
    a *= 2;
    let c = &a;
    // *c += 1;
    println!("c={}", c);
    println!("a={}", a);
}
