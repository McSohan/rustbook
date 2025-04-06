fn main() {
    let x = 5;
    println!("Value of variable x is {x}");
    let x = 7; // This is allowed
    //x = 7; // but this is not allowed - because x is immutable
    println!("Value of variable x is {x}");
    let mut y = 5;
    println!("Value of variable y is {y}");
    y = 8;
    println!("Value of variable y is {y}");
}

/*
mut cannot be used with constants 
the type of the value must be annotated 
const is used to declare them and can be used in any scope - even global (unlike let which is function scope).
Can only be set to something that can be computed at compile time, actually, more specifically - 
    it can only be something that can be a constant expression 
    usually ALL_CAPS
Mutating the type of a variable is not allowed.
Rust is statically typed- it must know the data types of all variables at compile time.

Number literals that can be of multiple types allow a type suffix- 58u16 is an example.
Numbers can also use _ as a visual separator anywhere.
0x, 0o, 0b --> these representations are also allowed.
Integer overflows are checked in debug mode , but not in release mode at compile time. 
f32, f64, bool (true, false), char (this is unicode in rust- 4 bytes).

Compund types in rust :
    tuple -- elements can have different types
        let tup: (i32, u32, f32) = (-2, 2, 2.2);
        tup.0 --> accesses the 0th element of the tuple
        let (x, y, z) = tup; // this is destructuring
    array -- Unlike a tuple, every element of an array must have the same type and fixed length.
        let a: [i32; 5] = [1, 2, 3, 4, 5];
    These are on the stack 
    let a = [3; 5];
        initializes the array with 5 elements of value 3.
    a[0] --> to index into the array 
    


Also : 
    In Rust, variables are not initialized by default, meaning you must explicitly provide 
    an initial value when declaring them. The compiler will enforce this, preventing you from 
    using uninitialized variables.
*/
