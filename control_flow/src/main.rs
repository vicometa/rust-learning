fn main() {
    another_function(5);
    print_labeled_measurement(3600, 'm');
    exp_test();
    println!("The value of five() is: {}", five());

    let x = 5;
    let y = increment(x);
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // ########################################################

    test_if();
    test_if_else(5);
    test_let_if(5 % 2 == 0);

    // ########################################################

    test_loop();
    test_loop_marker();
    test_while();
    test_for();
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];
    //let mut index = 0;

    //while index < 5 {
    //    println!("the value is {}", a[index]);
    //    index += 1;
    //}

    for element in &a {
        println!("the value is: {element}");
    }

    for i in (1..=10).rev() {
        println!("{i}!");
    }
    println!("Explosion!!!");
}

fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn test_loop_marker() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            } else if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn test_loop() {
    let mut counter = 0;

    let result = loop {
        println!("Counter: {counter}");

        if counter == 10 {
            break counter * 2;
        }

        counter += 1;
    };

    println!("The result is: {result}");
}

// ##########################################################

fn test_let_if(condition: bool) {
    let result = if condition { 5 } else { 6 };
    // let result: i32 = if condition { 5 } else { "six" }; // This will cause a compile-time error due to mismatched types

    println!("The value of result is: {result}");
}

fn test_if_else(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn test_if() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// #########################################################

fn increment(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn exp_test() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}
