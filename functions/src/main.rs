fn main() {
    print_hello_world();
    print_integer_x(5);
    print_labeled_measurement(5, 'h');
    expression_evaluation();
    let x = five();
    println!("The value of x is: {x}");
}

fn print_hello_world() {
    println!("Hello, world!");
    
}

// In function signatures, you must declare the type of each parameter
fn print_integer_x(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.
*/

// The block {} is an expression here which evaluates to the value 4
fn expression_evaluation() {
    let y = {
        let x = 3;
        x + 1 // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    }; //Expressions do not include ending semicolons

    println!("The value of y is: {y}");
}

// we must declare the type of the return value after an arrow (->)
fn five() -> i32 {
    5
}

