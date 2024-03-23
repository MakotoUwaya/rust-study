fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);     // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    let data = "user";

    let guess: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not number");
            return;
        },
    };
    println!("{} da, {}", guess, data);
    print_month(11);

    let user1 = build_user(String::from("hoge h"), String::from("email@test.example"));
    print_user_info(&user1);

    let user2 = User {
        username: String::from("aaaa bbbbb"),
        email: String::from("saasdf@asdfkljasdf.example"),
        ..user1
    };
    print_user_info(&user2);
}

fn print_month(index: usize) {
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}", months[index])

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn print_user_info(user: &User) {
    println!("username: {},
     email:{} ({}) - {}", user.username, user.email, user.active, user.sign_in_count);
}