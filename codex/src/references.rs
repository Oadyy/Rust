fn references () {
    // normal ref
    let x = 32;
    let ref_x = &x;
    pass_ref_to_function(ref_x);

    // mut ref
    let mut str = String::form("Hello");
    pass_ref_mul_to_function(&str);

    // slices
    let s = String::from("hello kitty");
    let hello = &s[0..5]; // &ref[start_index.. end_index +1]
    let kitty = &s[6..11]; // &ref[start_index.. end_index +1]
    /// &ref[start_index.. end_index +1]
    /// shoterm code &ref[0..5] -> &ref[..5] if start index = 0
    /// then &ref[6..end_index + 1] -> &ref[6..] if end index is len
    /// and combind it &ref[0..len] -> &ref[..] 
}

fn pass_ref_to_function(i32: &i) {
    println!("ref i = {&i}");
}

fn pass_ref_mul_to_function(String: &str) {
    &str.push_str(" World");
}