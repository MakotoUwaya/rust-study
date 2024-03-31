#[allow(dead_code)]
pub fn using_string() {
    let data = "initial contents";

    let s1 = data.to_string();
    println!("{}", s1);

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();
    println!("{}", s2);
    assert_eq!(s1, s2);

    let mut s3 = String::from("initial contents");
    s3.push_str(&s2);
    println!("{}", s3);

    let concat_text = format!("{} {}", s1, s2);
    println!("{}", concat_text);
}
