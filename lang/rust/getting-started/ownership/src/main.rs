fn main() {
    let mut s1 = String::from("This is a string, yeah!");
    s1.push_str(" So why not more?");
    println!("{s1}");

    let s2 = &s1;
    //s2.push_str("compiler error"); // If we borrowed s1 as a & then we can write to it.
    println!("s1:{s1}, s2:{s2}"); // s1 is borrowed so it can't be used
    println!("s2:{s2}");

    // let s2 = s1.clone();
    // println!("s1:{s1}, s2:{s2}") // Optional use clone to avoid borrowing issues but at the sake of performance


}
