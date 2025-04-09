// import libraries
use std::io;
// search for crates: crates.io
// add to Cargo.toml
use rand::random;
use rand::prelude::*;
use std::env;
use std::fs;
use std::io::prelude::*;

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

    // Challenge: removing leading and ending spaces from a string without using .trim()
    let message = String::from("    Hello, world!  ");
    println!("string is \"{}\"", message);
    let result = remove_space(&message);
    println!("result is \"{}\"", result);

    // // interact with user
    // let mut buffer = String::new();
    // println!("Enter a message:");
    // io::stdin()
    //     .read_line(&mut buffer)
    //     .expect("Failed to read line");
    // println!("You entered: {}", buffer);

    // // parse the input
    // let number = buffer.trim().parse::<i32>().expect("Failed to parse number");
    // println!("number + 1 is {}", number + 1);

    // let number = random::<f64>();
    // println!("random number is {}", number);

    // // challenge: number guessing game
    // let secret_number = rand::rng().random_range(1..=100);
    // println!("I'm thinking of a number between 1 and 100...");
    // println!("Guess the number: ");
    // loop {
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let guess: i32 = guess.trim().parse().expect("Please enter a number");

    //     if guess < secret_number {
    //         println!("Too small!");
    //     } else if guess > secret_number {
    //         println!("Too big!");
    //     } else {
    //         println!("You guessed it!");
    //         break;
    //     }
    // }

    // // argument parsing
    // if env::args().len() <= 2 {
    //     println!("Program requires at least 2 arguments");
    // }else{
    //     for (index, argument) in env::args().enumerate() {
    //         println!("Argument {}: {}", index, argument);
    //     }
    //     let arg2 = env::args().nth(2).unwrap();
    //     println!("Argument 2: {}", arg2);
    // }

    // read file
    let contents = fs::read_to_string("planets.txt").unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }

    // write file
    let mut hello_message = String::new();
    hello_message.push_str("Hello World!\n");
    let _ = fs::write("hello_message.txt", hello_message);

    let mut my_file = fs::OpenOptions::new().append(true)
        .open("hello_message.txt").unwrap();
    let _ = my_file.write(b"Hello from Earth!\n"); // remember the b prefix! 

    // Struct
    // Stuct data is by default stored on the stack
    // If a stuct also has a String, it will be stored on the heap with the ptr, len, and cap on the stack
    let mut vehicle = Shuttle {
        name: String::from("Space Shuttle"),
        crew_size: 7,
        propellant: 100.0
    };
    println!("vehicle name is {}", vehicle.name);
    vehicle.name = String::from("SpaceX Falcon 9");
    println!("Vehicle is {:?}", vehicle);

    // Struct update
    let vehicle2 = Shuttle {
        name: String::from("SpaceX Falcon Heavy"),
        ..vehicle // This will copy the rest of the fields from vehicle
    };
    vehicle.crew_size = 6; // This will not affect vehicle2
    println!("Vehicle is {:?}", vehicle);
    println!("Vehicle is {:?}", vehicle2);

    let vehicle3 = Shuttle {
        crew_size: 10,
        ..vehicle.clone()
        // Since now we have #[derive(Clone)], 
        // we can use this to copy the strings in the struct instead of own the strings
    };
    println!("Vehicle is {:?}", vehicle);
    println!("Vehicle is {:?}", vehicle3);

    // Struct methods
    let vehicle_name = vehicle.get_name();
    println!("Vehicle name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(10.0);
    println!("propellant is {} after adding fuel", vehicle.propellant);

    // Associated functions
    let vehicle4 = Shuttle::new("SpaceX Starship");
    println!("Vehicle is {:?}", vehicle4);

    // Tuple struct
    let red = Colour(255, 0, 0);
    println!("First value is {}", red.0);

    let point = Point(10, 20, 30);
    println!("Y value is {}", get_y(point));

    // Challenge: define a struct to represent a rectangle
    let mut my_rectangle = Rectangle::new(4.0, 3.0);
    println!("Rectangle is {:?}", my_rectangle);
    println!("Area is {}", my_rectangle.area());
    my_rectangle.scale(2.0);
    println!("New rectangle is {:?}", my_rectangle);
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

fn remove_space(s: &str) -> &str {
    let mut start = 0;
    for (index, item) in s.chars().enumerate() {
        if item != ' ' {
            start = index;
            break;
        }
    }

    let mut end = 0;
    for(index, item) in s.chars().rev().enumerate() {
        if item != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}

#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

struct Colour(u8, u8, u8); // RGB

struct Point(u8, u8, u8); // X, Y, Z

fn get_y(p: Point) -> u8 {
    p.1
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}