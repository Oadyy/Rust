fn function() {
    /// Rust function use fn prefix and name conventional style and variable names are snake case follow this
    /// 
    simple_function();

    function_with_parameter("hi", 5);

    _ = function_recive_return(1,2);

    expression();
}

fn simple_function() {
    println!("simple_function called");
}

fn function_with_parameter(param1: &str, param2: u32) {
    println!("function_with_parameter called argument: param1 = {param1}, param2 = {param2}");
}

fn function_recive_return(p1: u32, p2: u32) -> u32 {
    println!("function_recive_return called argument: p1 = {p1}, p2 = {p2}");

    return p1 + p2;

    /// Rust function return have little indie you can write below for return value
    /// p1 + p2 and don't have semicolon
}

fn expression() {
    let express = {
        let x = 3;
        x + 1 // when will return value don't have semicolon
    };

    let first_name: &str = "first";
    let last_name: &str = "last";
    let full_name = {
        "{first_name} {last_name}" // when will return value don't have semicolon
    };
}