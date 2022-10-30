pub fn control() {
    // if_statement();
    // while_loop();
    // for_loop();
    match_statement();
}

fn if_statement() {
    let temp = 5;
    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold!");
    } else {
        println!("temperature is okay");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" }; // if expression
    println!("Today is {}", day);
}

fn while_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x={}", x);
    }

    let mut y = 1;
    loop {
        // while true
        y *= 2;
        println!("y={}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    // [1 to 11) end exclusive
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        // if x == 8 {
        //     break;
        // }
        println!("x={}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 36;
    let country = match country_code {
        36 => "hungary",
        40 => "romania",
        61 => "thailand",
        1...999 => "unknown", // [1 to 999] both inclusive
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country);
}
