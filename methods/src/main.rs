#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() && rect1.height() {
        println!(
            "The rectangle has been initialized with nonzero width and height; it is {} for width and {} for height",
            rect1.width, rect1.height
        );
    }

    println!("Can rect1 hold rect2? It is {}!", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? It is {}!", rect2.can_hold(&rect3));

    let sqr = Rectangle::square(10);
    if sqr.is_square() {
        println!("\nThe rectangle");
        dbg!(sqr);
        println!("is a square!");
    }
}
