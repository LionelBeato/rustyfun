fn loop_fun() {
    // this will loop forever
    loop {
        println!("again!");
    }

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let x = 1;

    while x == 1 {
        println!("hi!");
    }

    for number in (1..4) {
        println!("{}", number);
    }
}
