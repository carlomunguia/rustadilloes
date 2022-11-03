fn main() {
    // let x = vec!["carlo".to_string()];
    // let y = x;
    // let z = y;
    // println!("{:?}", z);

    let x = vec!["carlo".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
