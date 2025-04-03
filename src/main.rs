fn main(){
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
}