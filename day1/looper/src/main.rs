fn main() {
    loop_to_10();
    array_loop();
}

fn loop_to_10() {
    // -------------
    // using loop {}

    // let mut n = 0;
    // loop {
    //     n += 1;
    //     println!("Hello {}", n);
    //     if n >= 10 {
    //         return;
    //     }
    // }

    // -----------
    // using for

    // for n in 0..10 {
    //     println!("Hello {}", n);
    // }
}

fn array_loop() {
    // using Vec:new()

    // let mut v = Vec::new();
    // v.push(4);
    // v.push(7);
    // v.push(9);

    // using vec macro
    let v = vec![4, 7, 9, 10];

    for n in &v {
        // using break to stop the loop
        // this will loop until it found 9, so 4 and 7 will be printed out

        // if n == 9 {
        //     break;
        // }

        // using continue to skip to curent iteration
        // this will skip all even number

        if n % 2 == 0 {
            continue;
        }

        println!("{}", n);
    }

    // breaking nested loop, using label

    'outer: for i in 0..10 {
        for n in &v {
            if i + n == 9 {
                break 'outer;
            }

            println!("{}", n);
        }
    }
}
