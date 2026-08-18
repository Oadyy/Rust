fn variables_and_mutablility() {
    declare_variables();

    mutable_immutable();

    /// declare constant
    const HOURS_IN_DAY: u32 = 24;

    shadowing();
}

fn declare_variables() {
    // auto assign data type
    let auto_data_type = 5;
    let mut most_input = String::new();

    /// manual assign data type
    // integer
    let signed_int_8: i8 = 5;
    let signed_int_32: i32 = 5;
    let signed_int_64: i64 = 5;
    let signed_int_128: i128 = 5;
    let signed_int_arc_depend: isize = 5; // depend code are run on architecture x32, x64

    let unsigned_int_8: u8 = 5;
    let unsigned_int_32: u32 = 5;
    let unsigned_int_64: u64 = 5;
    let unsigned_int_128: u128 = 5;
    let unsigned_int_arc_depend: usize = 5; // depend code are run on architecture x32, x64

    /// Fun fact: If you declare a 8-bit unsigend integer variable and it overflows.
    /// in Rust it won't show an runtime error, but it will occurs "Nuclear Gandhi" issue lol.
    // char
    let char: char = 'c';

    // string
    let string: &str = "String";

    // float point
    let float_32: f32 = 3.2;
    let float_64: f64 = 6.4;

    // boolean
    let boolean: bool = true;
}

fn mutable_immutable() {
    /// in rust variable by defaul are immutable.
    let im: u32 = 5; // you can change value any more

    /// if you want use variable can mutable you must add mut.
    let mut m: u32 = 5;
    m = m + 1; // you can re-assign data as same data type
}

fn shadowing() {
    /// Shadowing in rust is hard understand,
    /// it allow you declare a new variable (Change data type or re-assign) with same name.
    /// it effect in same scope
    let x = 5;
    let x = x + 1; // shadowing x will change 5 -> 6
    {
        let x = x * 2; // shadowing x will change 6 -> 12 but x is effect only inner scope!!!
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
