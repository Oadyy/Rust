fn data_collection() {
    Vectors();

    Strings();


}

fn Vectors() {
    // vector (other language is list)
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    let mut m_v = Vec::new(); // by default vector store i32
    m_v.push(5);
    m_v.push(6);
    m_v.push(6);
    m_v.push(7);

    // get pointer element
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // save get pointer element to Option type, if index out of range will return None
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // get value from vector (it work data type i32 have impelemt copy)
    let serven: i32 = m_v[3];

    // pop element
    let poped_element = v.pop();

} // <- all vector and element with in will freed after end the scope

fn Strings() {
    let mut s = String::new(); // default UTF-8 string

    /// fadamental string
    let s: &str = "Hello, world!"; // it ref string on stack, it is immutable
    let heap_s = s.to_string(); // it copy string to heap and bind to heap_s
    let ref_s = heap_s.as_str(); // it copy string heap_s, add allocate to stack then bind to ref_s

    let ss: String = String::from("Hello, world!"); // so String::from() != literal string event it same immutable

    // string by default is UTF-8, so it can store any language character
    let hello = String::from("Hello");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("こんにちは");
    let hello = String::from("สวัสดี");

    // add string
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str() append string slice to String
    s.push('!'); // push() append char to String

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    /// The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, 
    /// has to do with the signature of the method that’s called when we use the + operator. 
    /// The + operator uses the add method, whose signature looks something like this:
    /// fn add(self, s: &str) -> String { ... }
    
    /// So if you want to keep s1 valid after the addition, you can use the format! macro instead:
    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    /// rust not have string index alternative way is slice string by byte index,
    /// but it is not safe because rust string is UTF-8, byte range 1-4 bytes, so
    /// if you want slice й you will know latter before slice take bytes to refer start byte of й
    /// and you must know binary of й convert into byte for make sure you slice correct byte range
    /// now i don't know it have best practice to get character fron string.
    let hello = "Здравствуйте";
    // letter З decimal code => 1047
    // binary => 10000
    // utf-8 => 11010000 10010111
    let s = &hello[0..2]; // so it take 2 byte to store 3
    print!("\n{hello} -> {s}\n");

    // letter д decimal code => 1072
    // binary => 10000100000
    // utf-8 => 11010000 10100000
    let s = &hello[2..4]; // so it take 2 byte to store д start from byte 2 to 4
    print!("\n{hello} -> {s}\n");

    /// it so hard to slice long lenght string so i have solution
    let long_str =
        "welcome to rust codex, please select index to read more about rust programming language";
    let chars: Vec<char> = long_str.chars().collect();
    let char_i: char = chars[37];
    print!("\n{char_i}\n");
}

fn HashMaps() {
    /// create a new HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /// if you double insert same key it will replace value instaed
    scores.insert(String::from("Blue"), 25);
    /// if you want to insert value only if key not exist you can use entry() and or_insert()
    let blue_score = scores.entry(String::from("Blue")).or_insert(50); // or_insert() return mutable reference to value of key, so you can change value by dereference
    *blue_score += 1; // dereference to change value of key

    /// get value from HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    /// for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}