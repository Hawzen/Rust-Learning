fn main() {
    let mut surprise = Some(String::from("Hello"));
    // println!("{}", surprise.unwrap());
    let new_ref = surprise.insert(String::from("Hello2"));
    println!("{}", new_ref);
}
