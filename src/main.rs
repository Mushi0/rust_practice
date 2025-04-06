fn main() {
    println!("Hello, world!");

    let mut x: i8 = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is changed to {}", x);

    let a = 10;
    let b = 3.0;
    let c = a as f64 / b;
    println!("10 / 3 is {}", c);

    let d = 300;
    println!("casting {} to i8 is {}", d, d as u8);
    let e = -300;
    println!("casting {} to u32 is {}", e, e as u32);

    let is_pi = true;
    let f = if is_pi {3.14} else {3.0};
    println!("f is {}", f);

    let mut count = 0;
    let result = loop{
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("result is {}", result);

    let numbers = [1, 2, 3, 4];
    for number in numbers.iter(){
        println!("number is {}", number);
    }

    // Important: ownership in Rust! 
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!{"inner_planet is {}", inner_planet};
        outer_planet = inner_planet;
        // Cannot use inner_planet here because it is moved to outer_planet! 
    }
    println!("outer_planet is {}", outer_planet);

    // But we can use .clone() to duplicate the data
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        println!{"inner_planet is {}", inner_planet};
        inner_planet.clear(); // This will not affect outer_planet
    }
    println!("outer_planet is {}", outer_planet);

    // Strings live in heap memory
    // Integers live in stack memory because they are fixed size
    // Assigning an integer to another variable is itself a copy
    let outer_planet: i32;
    {
        let inner_planet = 10;
        outer_planet = inner_planet;
        println!{"inner_planet is {}", inner_planet};
    }
    println!("outer_planet is {}", outer_planet);
    
    // Ownership also works in parameters
    let rocket_fuel = String::from("RP-1");
    process_fuel(rocket_fuel.clone()); // Have to use .clone() to pass the value
    println!("rocket_fuel is {}", rocket_fuel);
    let rocket_fuel = process_fuel2(rocket_fuel); // Or by returning a value
    println!("rocket_fuel is {}", rocket_fuel);
    process_fuel3(&rocket_fuel); // Or by passing a reference (borrowing)
    println!("rocket_fuel is {}", rocket_fuel);
    let mut rocket_fuel = String::from("RP-1");
    process_fuel4(&mut rocket_fuel); // Need to specify mutability to change the value
    println!("rocket_fuel is {}", rocket_fuel);

    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);
    let last_word = &message[15..15+5]; // Slice
    println!("last_word is {}", last_word);
    let last_wrod = &message[15..]; // The same slice
    println!("last_word is {}", last_wrod);
    // Remember that length is in bytes, not characters

    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
    // However, a &String reference does not equal to a &str (string slice) reference
    // &String reference has ptr, len, and cap; while &str reference only has ptr and len
    // Therefore, we can convert &String to &str, but not the other way around
    let first_word = get_first_word2(&message);
    println!("first_word is {}", first_word);
    let second_word = get_first_word2(&message[10..]);
    println!("second_word is {}", second_word);
}

fn process_fuel(propellant: String) {
    println!("Processing propellant {}...", propellant);
}

fn process_fuel2(propellant: String) -> String {
    println!("Processing propellant {}...", propellant);
    propellant
}

fn process_fuel3(propellant: &String) {
    println!("Processing propellant {}...", propellant);
}

fn process_fuel4(propellant: &mut String) {
    // Whwn using a mutable reference, cannot create other references to it within a scope
    // This is to prevent data races
    println!("Processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable");
}

// fn process_fuel5() -> &String {
//     // This function demostrate returning of a dangling reference
//     // This is not allowed in Rust
//     let propellant = String::from("RP-1");
//     &propellant // This will not work because propellant will be dropped after the function ends
// }

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s
}

fn get_first_word2(s: &str) -> &str {
    let first_space = s.find(' ').unwrap_or(s.len());
    &s[0..first_space]
}