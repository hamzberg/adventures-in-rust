fn main() {
    println!("Hello, world!");

    another_function();

    function_with_parameters(2);

    print_label_measurement(12, 'f');

    expression();

    let z = the_number_five();

    println!("The value of z is: {z}.");

    let a = plus_one(5);

    println!("The value of a is: {a}");
}

fn another_function() {
    println!("Greetings from another_function()!");
}

fn function_with_parameters(x: i64) {
    println!("The value of x is: {x}.");
}

fn print_label_measurement(value: i64, unit_label: char) {
    println!("The measurement is: {value}{unit_label}.");
}

fn expression() {
    let y = {
        let x = 3;
        //"If you add a semicolon to the end of an expression,
        // you turn it into a statement,
        // and it will then not return a value."
        x + 1
    };

    println!("The value of y is: {y}.");
}

fn the_number_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
