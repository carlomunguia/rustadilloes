fn main() {
    let v = vec![1, 2, 3];

    let third: &i32 = &v[2];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
