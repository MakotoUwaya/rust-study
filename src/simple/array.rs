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
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let first = vec[0];
    println!("first: {:?}", first);
    println!("{:?}", vec);
    vec.push(4);
    println!("first: {:?}", vec);
    for i in &vec {
        println!("four_information: {}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i += 50;
        // println!("{}", i);
    }
    println!("{:?}", v);
}
