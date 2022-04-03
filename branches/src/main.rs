fn main() {
    // One zero apart
    let condition = if 0.00000000000000000000000000000000000000000000000000000000000000000000000001 == 0.0000000000000000000000000000000000000000000000000000000000000000000000001 { true } else { false };

    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    for character in "اهلا وسهلا!".chars() {
        print!("{} ", character); // Doesn't work, variable size chars        
    }
}
