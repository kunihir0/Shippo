use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut shipping_info = HashMap::new();
    let prompts = ["Real name", "Street address", "Apt # (leave blank if not needed)", "City", "State", "Zip code"];

    println!("Please enter your shipping information:");

    for prompt in prompts.iter() {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        shipping_info.insert(prompt.to_string(), input.trim().to_string());
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("Your shipping information:");
    println!("{}", shipping_info["Real name"]);
    println!("{}", shipping_info["Street address"]);
    if !shipping_info["Apt # (leave blank if not needed)"].is_empty() {
        println!("APT #{}", shipping_info["Apt # (leave blank if not needed)"]);
    }
    println!("{} {} {}", shipping_info["City"], shipping_info["State"], shipping_info["Zip code"]);
}
