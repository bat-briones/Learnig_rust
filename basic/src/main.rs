fn main() {
    /*
    functions: the definition of a new function is fn
    the parameters of the function is a topic that we will see in the next section, but for now we can create a function without parameters
    the return type of the function is a topic that we will see in the next section, but for now we can create a function without return type
        */
    hello_world(); // this call the function and print "Hello, world!" in the terminals

    sum(5); // this call the function and print "The sum is: 10" in the terminals

    let i = 26;
    let j = {
        let k = i + 1;
        k + 1
    };
    println!("j: {}", j); // this print "j: 28" in the terminals
    let sum = sum_with_return(5); // this call the function and return 10, then we can print it
    println!("sum with return: {}", sum); // this print "sum with return:

    if j > 27 {
        println!("j is greater than 27");
    } else {
        println!("j is less than or equal to 27");
    }
    let mut counter = 0;
    loop {
        println!("This is an infinite loop");
        counter += 1;
        if counter == 5 {
            break; // this break the loop when counter is equal to 5
        }
    }

    while counter < 10 {
        println!("This is a while loop");
        counter += 1;
    }

    let my_loop = loop {
        println!("This is a loop with a return value");
        if counter == 15 {
            break counter * counter; // this break the loop and return the value of counter
        }
        counter += 1;
    };
    println!("my loop: {}", my_loop); // this print "my loop: 225" in the terminals

    let list: [i32; 5] = [1, 2, 3, 4, 5]; // this is an array of integers with size 5

    for element in list.iter() {
        println!("element: {}", element); // this print "element: 1", "element: 2", "element: 3", "element: 4", "element: 5" in the terminals
    }

    for number in 1..5 {
        println!("number without 5 : {}", number); // this print "number: 1", "number: 2", "number: 3", "number: 4", "number: 5" in the terminals
    }

    for number in 1..=5 {
        println!("number: {}", number); // this print "number: 1", "number: 2", "number: 3", "number: 4", "number: 5" in the terminals
    }

    for element in list.iter().rev() {
        println!("element in reverse: {}", element); // this print "element in reverse: 5", "element in reverse: 4", "element in reverse: 3", "element in reverse: 2", "element in reverse: 1" in the terminals
    }
}

fn hello_world() {
    println!("Hello, world!");
}

fn sum(a: i32) {
    println!("The sum is: {}", a + a);
}

// with return type
fn sum_with_return(a: i32) -> i32 {
    a + a // normaly the last line of the function is the return value, but you can use the return keyword if you want to return early
}
