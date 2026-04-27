fn main() {
    /*
    types of variables:
    scalar types: rust has 4 types of scalar types: integer,
    floating-point, boolean and character
    this types are the tradicional types of variables in programming languages
      */

    //integers types
    // rust has different types of integers: signed and unsigned, with different sizes
    // the size of the integer is in bits
    // unsigned range: 0 .. 2^n - 1 (n is the size in bits)
    // signed range: -2^(n-1) .. 2^(n-1) - 1 (n is the size in bits)

    /*
    this have a matematical explication

    how work ?
    one bit is the minimum unit of information in a computer, it can be 0 or 1 (2 posibilities)
    if you create a variable with the minum size of 8 bits, you have 8 bits to represent a number,
    so you have 2^8 posibilities (256 posibilities), in a few words, you can save from 0 to 255 because the
    programation lenguage start to count from 0
    if the same posibilities are for signed integers, you have 2^8 posibilities but you need to save the negative numbers,
    so you have 128 posibilities for negative numbers and 127 posibilities for positive numbers, because the programation
    lenguage start to count from 0, so you can save from -128 to 127

    But why did this happen?
    why not is from -255 to ?
    how the computer know if the number is negative if dont exist a symbol

    well, this is my explication JAJJAJAJ

    TYPES TO SAVE POSITIVE NUMBERS AND NEGATIVE NUMBERS?
    error, if you have different types of integers, then you
    need to change all hardware to create a new type of integer

    why -128 to 127?
    because the computer use the first bit to save the sign of the number,
    if the first bit is 0, the number is positive, if the first bit is 1, the number is negative,
    weel basically this is beacause the first value say, if im on then you have a debt with me
    how much? 128, then all number after me plus value in the debt
     */
    let x: i8 = 127; // signed integer with 8 bits
    // if you say, well, this number is in your limit, what happen if you add 1 to this number?
    println!("x: {}", x); // this print -128 because of the overflow

    //x = x + 1; // this trigger an error in terminal
    //println!("x: {}", x); // this print -128 because of the overflow

    /*
    floating-point types (with comma):

        */

    let decimal: f32 = 3.14;
    let decimal2 = 3.14;
    println!("decimal: {}", decimal);
    println!("decimal2: {}", decimal2);

    /*
    BOOLEAN TYPE:
    the boolean type is saved in a byte

     */

    let boolean: bool = true; // in python is True
    let boolean2: bool = false; // in python is False
    println!("boolean: {}", boolean);
    println!("boolean2: {}", boolean2);

    // character type:
    let character: char = 'a'; // in singles quotes
    let character2: char = 'b'; // in singles quotes
    println!("character: {}", character);
    println!("character2: {}", character2);

    // compound types:
    // tuple type: a tuple is a group of values with different types, the values are separated by commas and the tuple is enclosed in parentheses
    let tuple: (i32, f32, char) = (1, 3.14, 'a');
    let (a, b, c) = tuple; // this is called destructuring, you can assign the values of the tuple to different variables
    println!("tuple: {:?}", tuple); // this print the tuple with debug format
    println!("a: {}, b: {}, c: {}", a, b, c);

    let first_value = tuple.0; // this is the way to access the values of the tuple, you can use the index of the value, starting from 0
    println!("first value: {}", first_value);

    // its important say the tuple need to be choise your size, and your types, because if you want to change the size of the tuple,
    // you need to change all the code that use this tuple

    // array types: all values need to be of the same type, the values are separated by commas and the array is enclosed in square brackets
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // [type; size]
    println!("array: {:?}", array); // this print the array with debug format 

    let firts_value_array = array[0]; // this is the way to access the values of the array, you can use the index of the value, starting from 0

    println!("first value of array: {}", firts_value_array);
    let [_, second, third, fourth, fifth] = array; // destructuring of the array
    println!("second value of array: {}", second);
    println!("third value of array: {}", third);
    println!("fourth value of array: {}", fourth);
    println!("fifth value of array: {}", fifth);
    println!("second value of array: {}", second);
    println!("third value of array: {}", third);
    println!("fourth value of array: {}", fourth);

    /*vector types: this is the list, the main difference with another tyypes is your size,
    the vector is bassically a list with dynamic size
    */
    let mut basic_vector = vec![1, 2, 3, 4, 5]; // this is a vector of integers
    println!("basic vector: {:?}", basic_vector); // this print the vector with debug format
    let first_value_vector = basic_vector[0]; // this is the way to access the values of the vector, you can use the index of the value, starting from 0
    println!("first value of vector: {}", first_value_vector);

    basic_vector.push(2);
    println!("basic vector after push: {:?}", basic_vector); // this print the vector with debug format
    dbg!(&basic_vector); // this print the vector with debug format and the line number where this code is located

    basic_vector.pop(); // this remove the last value of the vector and return it, if the vector is empty, return None
    print!("{:?}", basic_vector); // this print the vector with debug format and the line number where this code is located
}
