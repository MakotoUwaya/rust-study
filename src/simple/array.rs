#[allow(dead_code)]
pub fn print_month(index: usize) {
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

#[allow(dead_code)]
pub fn using_vec() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);
}
