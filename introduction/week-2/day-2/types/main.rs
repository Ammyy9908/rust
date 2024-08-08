fn main() {
    //Compound Types

    //Tuple Type

    let s = (1, 2, 3, 4, 5);
    println!("The value of s is: {:?}", s);

    let t: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value of t is: {:?}", t);

    let (u, v, w) = t;

    println!("The value of u is: {}", u);

    println!("The value of v is: {}", v);

    println!("The value of w is: {}", w);

    println!("The value of t is: {:?}", t);

    println!("The value of t.0 is: {}", t.0);

    println!("The value of t.1 is: {}", t.1);

    println!("The value of t.2 is: {}", t.2);

    //Array Type

    let x = [1, 2, 3, 4, 5];

    println!("The value of x is: {:?}", x);

    let y: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of y is: {:?}", y);

    let z = [3; 5];

    println!("The value of z is: {:?}", z);

    println!("The value of z.0 is: {}", z[0]);

    println!("The value of z.1 is: {}", z[1]);

    println!("The value of z.2 is: {}", z[2]);

    //Slice Type

    let a = [1, 2, 3, 4, 5];

    let b = &a[1..3];

    println!("The value of b is: {:?}", b);

    let c = &a[..3];

    println!("The value of c is: {:?}", c);

    let d = &a[1..];

    println!("The value of d is: {:?}", d);

    //String Type

    let e = String::from("Hello, World!");

    println!("The value of e is: {}", e);

    let f = "Hello, World!".to_string();

    println!("The value of f is: {}", f);

    let g = String::from("Hello, ") + "World!";

    println!("The value of g is: {}", g);

    //Reference Type

    let h = 5;

    let i = &h;

    println!("The value of i is: {}", i);

    let j = &h;

    println!("The value of j is: {}", j);

    let k = &h;

    println!("The value of k is: {}", k);

    //Pointer Type

    let l = 5;

    let m = &l as *const i32;

    println!("The value of m is: {:?}", m);

    let n = &l as *const i32;

    println!("The value of n is: {:?}", n);

    let o = &l as *const i32;

    println!("The value of o is: {:?}", o);

    //Type inference

    let p = 5;

    println!("The value of p is: {}", p);

    let q = 5.0;

    println!("The value of q is: {}", q);

    let r = 5i32;

    println!("The value of r is: {}", r);

    //Function Parameters and Return Types

    /**
     *
     * Rust requires explicit type annotations for
     * function parameters and return types to
     * ensure clarity and prevent ambiguity:
     */

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let s = add(5, 5);

    println!("The value of s is: {}", s);
}
