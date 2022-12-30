fn main() {
    //Reviewing if statements.
    first_if_statement();
    second_if_statement();
    third_if_statement();
    fourth_if_statement();

    //Reviewing loop statements.
    //first_loop_statement();
    second_loop_statement();
    third_loop_statement();

    //Reviewing while statements.
    first_while_statement();
    second_while_statement();

    //Reviewing for statements.
    first_for_statement();
    second_for_statement();
}

fn first_if_statement() {
    let number = 3;

    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }
}

fn second_if_statement() {
    let number = 3;

    if number != 0 {
        println!("The number was something other than zero.");
    }
}

fn third_if_statement() {
    let number = 6;

    if number % 4 == 0 {
        println!("The number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2.");
    } else {
        println!("The number is not divisible by 4, 3, or 2.");
    }
}

fn fourth_if_statement() {
    let condition = true;
    //When in a let expression, all values of the if statement must be the same type.
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}.");
}

fn _first_loop_statement() {
    loop {
        println!("Looping again!");
    }
}

fn second_loop_statement() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}.");
}

fn third_loop_statement() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}.");
}

fn first_while_statement() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!");
}

fn second_while_statement() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value of the array at index {index} is: {}", a[index]);

        index += 1;
    }
}

fn first_for_statement() {
    let a = [20, 40, 60, 80, 100];

    for element in a {
        println!("The value is: {element}");
    }
}

fn second_for_statement() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF");
}
