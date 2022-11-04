fn main() {
    // let x = vec!["carlo".to_string()];
    // let y = x;
    // let z = y;
    // println!("{:?}", z);

    //clone
    // let x = vec!["carlo".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // // copy
    // let x = 1;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    let s = String::from("takes");
    takes_ownership(s);

    let val = 1;
    make_copy(val);
}

fn takes_ownership(s: String) {
    let string = s;
    println!("{}", string)
}

fn make_copy(one: i32) {
    let val1 = one;
    println!("{}", val1);
}

fn take_and_give(str2: String) -> String {
    str2
}
