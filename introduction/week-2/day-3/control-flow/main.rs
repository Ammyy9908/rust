fn main() {
    //if expression

    let number = 3;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    //else if

    let number = 6;

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }

    //if in let statement

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    //loops

    //loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    //infinite loop
    // loop {
    //     println!("Hello, World!");
    // }

    //returning values from loops

    let mut ctr = 0;

    let result = loop {
        ctr += 1;

        if ctr == 10 {
            break ctr * 2;
        }
    };

    println!("The result is {result}");

    //conditional loops with while

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //loop through a collection using for

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    //for loop with range

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
