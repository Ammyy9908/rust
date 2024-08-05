fn main() {
    let x = 5;
    let y = 10;
    let z = x + y;
    println!("The value of z is: {}", z);
    let x = 15;
    let z = x + y;
    println!("The value of z is: {}", z);
    let x = 20;
    let z = x + y;
    println!("The value of z is: {}", z);

    //If we trying to update the x we will got an error
    //verify to uncomment the next line
    //x = 25;

    //but if we add mut keyword to variable then we can update the value of that variable

    let mut p = 10;
    println!("The value of p is: {}", p);
    p = 15;
    println!("The value of p is: {}", p);

    //Constants

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    //Shadowing

    let a = 5;
    let a = a + 1;

    let a = a * 2;
    {
        let a = a * 2;
        println!("The value of a is: {}", a);
    }

    println!("The value of a is: {}", a);
}
