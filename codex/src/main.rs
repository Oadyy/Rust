use ferris_says::say;
//use std::env;
use std::io::{self, BufWriter, Write};
use std::process::Command;

fn main() {
    let stdout = io::stdout();
    let message = String::from("Rust Codex 1.96.0");

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, 100, &mut writer).unwrap();

    // flush the buffer immediately to force correct order
    writer.flush().unwrap();

    const MENU_LEN: usize = 6;
    let codex: [&str; MENU_LEN] = [
        "Variables & Mutability",
        "Compound Data Types",
        "Function",
        "Control Flow",
        "Ownership and Reference",
        "Structure"
    ];
    let mut index = 0;
    while index < MENU_LEN {
        let bullet = index + 1;
        let header = codex[index];
        print!("{bullet}. {header}\n");
        index += 1;
    }

    print!("\nSelect index: ");
    _ = io::stdout().flush();

    loop {
        let mut index_str = String::new();

        io::stdin()
            .read_line(&mut index_str)
            .expect("Failed to read line");

        let i: u32 = match index_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error parse input");
                continue;
            }
        };

        exe_index(i);
    }
}

fn exe_index(index: u32) {
    let i_arg = index.to_string();
    Command::new("sh")
        .arg("./scripts/code_file.sh")
        .arg(i_arg)
        .spawn()
        .expect("code_file.sh command failed to start!");
}

// fn current_dir() -> std::io::Result<()> {
//     let path = env::current_dir()?;
//     println!("The current directory is {}", path.display());
//     Ok(())
// }
