fn main() {
    /*
    !important
    you can use different ways in functions, :: and .
    the main differece for example in string

    when you declare a string with let, rust will check if that variable do not brak the rules
    for that reason rust need to know the value of the variable at compile time
    for that reason exist String::from() and String::new() that are functions that create a string at runtime
    this means that string can change its value

    is
    !important
    say the keyword let nnot is the guility of that reason,
    the guility is the distribution of memory, heap and stack, and the way that rust manage the memory

     */

    {
        let a = "hello world";
    }
    let a = "in another";
    println!("variable out of scope: {}", a);

    /*
    the propierty of the variables is this, only one variable can own a value at a time
    whe you assign a value to a variable, that variable becomes the owner of that value
    if tha value is assigned to another variable, the ownership is moved to the new variable and the old variable is no longer valid
    */
    let value_1 = String::from("hello");
    let value_2 = value_1;
    // println!("value_1: {}", value_1); // this will cause an error
    println!("value_2: {}", value_2);
    /*to fix this error you need to put clone because ypu want clone the variable */
    let value_3 = value_2.clone();
    println!("value_3: {}", value_3);
}
