fn main() {
    // declare a variable

    let _x = 5; // always put a semicolon at the end of the line
    // all variables are inmutable by defeault
    //x = 6; // this will cause an error because x is inmutable

    /*
    if you want to change the value of a variable, you need to make it with the kwyword 'mut'
     */
    let mut y = 6;

    println!("Hello, world! {}", _x); // this functions requires a string as a arguments to print in the console

    println!("variable mutable before: {}", y);
    y += 5;
    println!("variable mutable after: {}", y);

    /*
        constants: the difference between a constant and variables
        is that,  constat you have define the type of the constant
        and you have to assing a value declare a constant,
        declare one constant is with the keyword 'const''
        another thing is only can be a value in concrete,
        not a result of a caculation
    */

    const PI: f64 = 3.1416; // recomended to use uppercase letters for constants

    println!("the value of pi is: {}", PI);

    // difference
    let x = 5; // this is a variable
    let x = 'y'; // this is a same variable but with a different value

    println!("the value of x is: {}", x);
}
