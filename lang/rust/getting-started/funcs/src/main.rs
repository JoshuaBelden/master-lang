fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum: i32 = add(7, 8);
    println!("The sum is {sum}");

    let mut counter: i32 = 0;
    let total: i32 = loop {
        println!("The index is {counter}");
        if counter == 5 {
            break counter;
        }
        counter += 1;
    };

    println!("The total is {total}");
}
