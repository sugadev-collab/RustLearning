fn describe_number(n: u32) {
    match n {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Not listed."),
    }
}

fn main() {
    for i in 0..5_u32 {
        describe_number(i);
    }
}
