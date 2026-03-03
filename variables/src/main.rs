fn demonstrate_constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}\n");
}

fn demonstrate_mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n");
}

fn demonstrate_shadowing() {
    let y = 5;

    println!("The value of y is: {y}");

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}\n");
}

fn demonstrate_integer_types() {
    let uint8: u8 = 255;
    let int8: i8 = -128;
    let uint16: u16 = 65535;
    let int16: i16 = -32768;
    let uint32: u32 = 4_294_967_295;
    let int32: i32 = -2_147_483_648;
    let uint64: u64 = 18_446_744_073_709_551_615;
    let int64: i64 = -9_223_372_036_854_775_808;
    let uint128: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let int128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;

    let uint_size: usize = usize::MAX;
    let int_size: isize = isize::MIN;

    println!(
        "unsigned 8-bit integer: {uint8}\n\
             signed 8-bit integer: {int8}\n\
             unsigned 16-bit integer: {uint16}\n\
             signed 16-bit integer: {int16}\n\
             unsigned 32-bit integer: {uint32}\n\
             signed 32-bit integer: {int32}\n\
             unsigned 64-bit integer: {uint64}\n\
             signed 64-bit integer: {int64}\n\
             unsigned 128-bit integer: {uint128}\n\
             signed 128-bit integer: {int128}\n\
             unsigned size integer: {uint_size}\n\
             signed size integer: {int_size}\n"
    );

    // let mut overflow: u8 = 255;
    // overflow += 1; // error: attempt to add with overflow, but in release mode it will wrap around to 0
    // println!("The value of overflowed i8 var is: {overflow}");
}

fn demonstrate_numeric_literals() {
    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_1010_0101_0000;
    let byte = b'A';

    println!(
        "Decimal: {dec}\n\
             Hexadecimal: {hex}\n\
             Octal: {oct}\n\
             Binary: {bin}\n\
             Byte (ASCII): {byte}"
    );
}

fn demonstrate_floating_point() {
    let f = 2.5; // f64 by default
    let g: f32 = 3.14; // f32

    println!("The value of f is: {f}\nThe value of g is: {g}");
}

fn demonstrate_calculator() {
    println!("{:?}", calculator(10.5, 3.5));
}

fn calculator(first: f64, second: f64) -> (f64, f64, f64, f64) {
    let sum = first + second;
    let difference = first - second;
    let product = first * second;
    let quotient = first / second;

    (sum, difference, product, quotient)
}

fn demonstrate_boolean_types() {
    let t = true;
    let f: bool = false;

    println!("The value of t is: {t}\nThe value of f is: {f}");
}

fn demonstrate_char_type() {
    let c = 'c';
    let z: char = 'Z';
    let emoji: char = '😀';

    println!("The value of c is: {c}\nThe value of z is: {z}\nThe value of emoji is: {emoji}");
}

fn demonstrate_tuple() {
    let tup: (i32, f64, u8) = (500, 6.28, 127);
    let (x, y, z) = tup;

    println!("The value of tup is: {tup:?}");
    println!("{x} = {}", tup.0);
    println!("{y} = {}", tup.1);
    println!("{z} = {}", tup.2);
}

fn demonstrate_array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = ["hello"; 5];
    println!("a: {a:?}");
    println!("b: {b:?}");

    //for i in 0..a.len() {
    //    println!("a[{i}] = {}", a[i]);
    //}
    for j in b {
        println!("b element: {j}");
    }
}

fn main() {
    demonstrate_constants();
    demonstrate_mutability();
    demonstrate_shadowing();
    demonstrate_integer_types();
    demonstrate_numeric_literals();
    demonstrate_floating_point();
    demonstrate_calculator();
    demonstrate_boolean_types();
    demonstrate_char_type();
    demonstrate_tuple();
    demonstrate_array();
}
