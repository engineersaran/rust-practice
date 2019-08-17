fn main() {
    println!("Hello, world!");
    // If condition
    let number = 6;

    if number != 0 {
        println!("number was something other than zero");
    }

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 3 or 2");
    }
    // if can be used expression and result can be stored in variable
    let result = if number % 4 == 0 {
        "number is divisible by 4"
    } else if number % 3 == 0 {
        "number is divisible by 3"
    } else if number % 2 == 0 {
        "number is divisible by 2"
    } else {
        "number is not divisible by 4 or 3 or 2"
    };
    println!("The result of If expression: {}", result);
}
