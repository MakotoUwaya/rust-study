fn main() {
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
    print_month(11);

    let user1 = build_user(String::from("hoge h"), String::from("email@test.example"));
    print_user_info(&user1);

    let user2 = User {
        username: String::from("aaaa bbbbb"),
        email: String::from("saasdf@asdfkljasdf.example"),
        ..user1
    };
    print_user_info(&user2);

    calc_rectangle();

    let ip_addr_v4 = IpAddrKind::V4;
    let ip_addr_v6 = IpAddrKind::V6;
    println!("ip_address_v4: {:?}", ip_addr_v4);
    println!("ip_address_v6: {:?}", ip_addr_v6);
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("std: ip_address_v4: {}", localhost_v4);
    println!("std: ip_address_v6: {}", localhost_v6);
}

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn print_month(index: usize) {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
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
    println!(
        "username: {}, email:{} ({}) - {}",
        user.username, user.email, user.active, user.sign_in_count
    );
}

/// ## Rectangle size props
/// param
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn calc_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Create square: {:?}", Rectangle::square(25));
}

#[derive(Debug)]
#[deprecated(since = "0.1.0", note = "use std::net::{IpAddr, Ipv4Addr, Ipv6Addr}")]
enum IpAddrKind {
    V4,
    V6,
}
