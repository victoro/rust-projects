#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            width: 6,
            height: 14,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn contains(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle::new();
    println!("area of rectangle with new data {}", &rect1.area());
    let rect2 = Rectangle {
        width: 5,
        height: 3,
    };
    println!("area of rectangle with redefined data {}", &rect2.area());

    let rect3 = Rectangle { width: 10, ..rect2 };
    println!(
        "area of rectangle with updates struct data {}",
        &rect3.area()
    );

    let scale: u32 = 4;
    let rect4 = Rectangle {
        width: dbg!(&rect3.width * scale),
        ..rect3
    };

    dbg!(&rect1);
    dbg!(&rect2);
    dbg!(&rect3);
    dbg!(&rect4);
    println!("Does rect1 contains rect 2 ? {}", rect1.contains(&rect2));
    println!("Does rect1 contains rect 4 ? {}", rect1.contains(&rect4));
    println!("Area of rect 4 => {}", &rect4.area());

    let rect_square = Rectangle::square(5);
    dbg!(&rect_square);
    println!(
        "Does rect1 contains rect square? {}",
        rect1.contains(&rect_square)
    );
}
