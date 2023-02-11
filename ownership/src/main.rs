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

    'tyrone: loop {
        loop {
            break 'tyrone;
        }
        print!("Unreachable, will the smart compiler figure it out? :)");
    }

    let s = String::from("Small step for man");
    print_len(&s);

    let word = get_first_word(&s);

    println!("{word}");
}

fn print_len(some_string: &String) {
    println!("Length: {}", (*some_string).len());
    println!("Length: {}", some_string.len());
}

fn get_first_word(s: &String) -> &str {
    // My solution for a function that gets the first word
    match s.chars().position(|c| c == ' ') {
        Some(x) => &s[..x],
        None => s
    }
}
