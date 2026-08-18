fn control_flow() {
    if_else_condition();

    loop_control();

    while_control();

    for_control();
}

fn if_else_condition() {
    let score = 35;
    // just like another language but it have not ()
    if score < 50 {
        println!("You got grade F");
    } else if score < 55 {
        println!("You got grade D");
    } else if score < 60 {
        println!("You got grade D+");
    } else if score < 65 {
        println!("You got grade C");
    } else if score < 70 {
        println!("You got grade C+");
    } else if score < 75 {
        println!("You got grade B");
    } else if score < 80 {
        println!("You got grade B+");
    } else {
        println!("You got grade A");
    }

    let number = if score < 80 { 1 } else { 0 };
}

fn loop_control() {
    // simple
    loop {
        println!("loop!");
    }

    // loop lable
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
    println!("End count = {count}");
}

fn while_control() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
}

fn for_control() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    /// countdown 4 3 2 1 
    /// (1..5) generate array(1 - n-1)
    /// .rev() reverse element
    for number in (1..5).rev() {
        println!("{number}!");
    }
}
