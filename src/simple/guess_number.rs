#[allow(dead_code)]
pub fn guess_number() {
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    let data = "user";

    let guess: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not number");
            return;
        }
    };
    println!("{} da, {}", guess, data);
}
