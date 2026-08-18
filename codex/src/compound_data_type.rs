fn compound_data_type() {
    /// Rust and other language have same common data type such as (integer, float, boolean, character, string).
    /// how to declare in variables_and_mutability.rs file. So I skiped
    /// The Compound data type it's array, list tuple in rush
    tuple();

    array();
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // mapping x y z
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // point tuple index
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of one is: {one}");
}

fn array() {
    // auto assign
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // full declare
    let i: [u32; 5] = [1, 2, 3, 4, 5];

    // generate declare
    let g: [u32; 5] = [0; 5]; // same g = [0,0,0,0,0]

    // element access
    let jan = months[0];
    let feb = months[1];

    // two dimension tbc
}
