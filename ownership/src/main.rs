
fn main() {
    let s = String::from("str me plz");
    println!("{}", &&s);
    println!("{}", &s);
    println!("{}", s);

    println!("{}", **(&&s));

    let str_slice: &str = &s[1..];
    println!("{}", str_slice)
}
