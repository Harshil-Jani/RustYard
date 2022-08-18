pub fn run() {
    let mut count = 0;

    // Infinite Loop
    loop {
        count += 1;
        println!("Number : {}", count);

        if count == 20 {
            break;
        }
    }

    // While Loop
    while count <= 100 {
        if count % 15 == 0 {
            println!("3 & 5 : {}", count);
        } else if count % 5 == 0 {
            println!("5 : {}", count);
        } else {
            println!("Random");
        }
        count += 1;
    }

    // For Range
    for x in 0..100{
        print!("{} ",x);
    }
}
