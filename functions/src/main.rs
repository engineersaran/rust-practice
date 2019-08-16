fn main() {
    println!("Hello, world!");
    function_without_argument();
    functon_with_single_argument(-10);
    functon_with_two_argument(9, 7);
    let x = function_with_return_values();
    println!("function_with_return_values x: {}", x);
    let x = plus_one(10);
    println!("plus_one x: {}", x);
    let x = plus_two(10);
    println!("plus_two x: {}", x);
    // expression language
    let z = {
        let x = 3;
        x + 10 // no semicolon for expression
    };
    println!("expression init z: {}", z);
}

fn function_without_argument() {
    println!("Function without Argument is called");
}

fn functon_with_single_argument(x: i32) {
    println!("functon_with_single_argument x: {}", x);
}

fn functon_with_two_argument(x: i32, y: i32) {
    println!("functon_with_two_argument x: {}", x);
    println!("functon_with_two_argument y: {}", y);
}

fn function_with_return_values() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    //x + 2;
    return x + 2;
}
