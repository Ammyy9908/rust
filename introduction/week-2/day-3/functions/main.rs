fn main() {
    print_hello();

    let x = 5;
    let y = 10;

    let sum = add(x, y);

    println!("The sum of {} and {} is: {}", x, y, sum);

    let a = 5;
    let b = 10;

    let result = return_value(a, b);

    println!("The result of {} and {} is: {}", a, b, result);
}

fn print_hello() {
    println!("Hello, World!");
}

//parameters

fn add(x: i32, y: i32) -> i32 {
    x + y
}

//statements and expressions

fn return_value(x: i32, y: i32) -> i32 {
    let z = {
        let a = x + y;
        a + 10
    };

    z
}
