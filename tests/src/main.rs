fn main() {
    let tuple: (i64, &str, (f32, f32)) = (32, "dod", (1.0, -1.048));

    match tuple {
        (32, "dod", (1.0, -1.048)) => println!("Yes"),
        _ => println!("No"),
    }
}
