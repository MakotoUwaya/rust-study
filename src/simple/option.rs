pub fn using_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    println!("some_number.is_some(): {}", some_number.is_some());
    println!("some_number.is_none(): {}", some_number.is_none());
    println!("some_string.is_some(): {}", some_string.is_some());
    println!("some_string.is_none(): {}", some_string.is_none());

    let absent_number: Option<i32> = None;
    println!("absent_number.is_some(): {}", absent_number.is_some());
    println!("absent_number.is_none(): {}", absent_number.is_none());

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or_default();
    println!("{}", sum);
}
