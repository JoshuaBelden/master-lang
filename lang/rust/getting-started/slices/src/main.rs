// fn fragile_first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("The first word is: {}", word);

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &a[2..=4];
    println!("{:?}", slice);
    assert_eq!(slice, [2, 3, 4]);
}
