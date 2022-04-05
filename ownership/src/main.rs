
fn main() {
    let s = String::from("str me plz");
    println!("{}", &&s);
    println!("{}", &s);
    println!("{}", s);

    println!("{}", **(&&s));

    let str_slice: &str = &s[1..];
    println!("{}", str_slice);

    
    let mut s1 = String::from("Test ");
    {
        let s2 = "me?";
        s1.push_str(s2);
    }
    println!("{}", s1);
}
