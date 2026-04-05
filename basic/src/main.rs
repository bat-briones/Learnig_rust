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
}
