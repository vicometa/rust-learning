struct User {
    // active: bool,
    username: String,
    email: String,
    // sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(usize, usize);

// struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = build_user(String::from("user1"), String::from("mail@example.com"));
    user1.email = String::from("another_one@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.email);
    println!("{} {}", user2.username, user2.email);

    // #############################################################

    let black = Color(0, 0, 0);
    let mut origin = Point(0, 0);
    display(&black, &mut origin);

    // #############################################################

    // let subject = AlwaysEqual;

    // #############################################################

    rectangle();
}

fn rectangle() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn display(color: &Color, point: &mut Point) {
    let mut buf = [
        [
            Color(255, 255, 255),
            Color(255, 255, 255),
            Color(255, 255, 255),
        ],
        [
            Color(255, 255, 255),
            Color(255, 255, 255),
            Color(255, 255, 255),
        ],
        [
            Color(255, 255, 255),
            Color(255, 255, 255),
            Color(255, 255, 255),
        ],
    ];

    for _ in 0..9 {
        buf[point.0][point.1] = Color(color.0, color.1, color.2);
        point.0 += 1;
        if point.0 == 3 {
            point.0 = 0;
            point.1 += 1;
        }
    }

    for row in &buf {
        for cell in row {
            print!("({},{},{}) ", cell.0, cell.1, cell.2);
        }
        println!();
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        // active: true,
        username,
        email,
        // sign_in_count: 1,
    }
}
