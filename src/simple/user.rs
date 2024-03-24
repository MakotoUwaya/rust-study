#[allow(dead_code)]
pub fn using_user() {
    let user1 = build_user(String::from("hoge h"), String::from("email@test.example"));
    print_user_info(&user1);

    let user2 = User {
        username: String::from("aaaa bbbbb"),
        email: String::from("saasdf@asdfkljasdf.example"),
        ..user1
    };
    print_user_info(&user2);
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
