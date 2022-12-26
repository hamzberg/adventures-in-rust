fn main() {
    //Immutable Variable - Default, type is inferred, value can not be changed.
    let x = 5;
    println!("The value of x is: {x}");

    //Mutable Variable - Permits changing of a value in later code.
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The altered  value of y is: {y}");

    // Constant Variable - Usable only within scope, never mutable, type must be annotated.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let a = 5;

    //The variable is redeclared, the value is not changed, but ammended. (Shadowing)
    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is: {a}");

    //rustc assumes f64 for floating point types.
    let b = 2.1;

    let c: f32 = 3.2;

    println!("The value of b is: {} \nThe value of c is: {}", b, c);

    //Addition
    let sum = 12 + 24;
    println!("The sum of 12 + 24 is {sum}.");

    //Subtraction
    let difference = 34.2 - 15.9;
    println!("The difference of 34.2 - 15.9 is {difference}.");

    //Multiplication
    let product = 23 * 12;
    println!("The product of 23 * 12 is {product}.");

    //Division
    let quotient = 23.8 / 10.3;
    let floored = 4 / 5;
    println!("The quotient of 23.8 / 10.3 is {quotient}.");
    println!("The result of 4 / 5 is {floored}.");

    //Remainder
    let remainder = 33 % 2;
    println!("The remainder of 33 % 2 is {remainder}.");

    //Booleans
    let vrai = true;
    println!("This sentence is {vrai}.");

    let faux: bool = false;
    println!("This sentence is {faux}.");

    //Characters
    let d = 'w';
    println!("We have a {d}inner!");

    let m: char = 'k';
    println!("I have found the {m}iller...");

    //Tuple
    let tup: (i64, f32, u16) = (250, 3.2, 12);
    let (f, g, h) = tup;
    println!("{f}, {g}, {h}");
}
