#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let width1 = 30;
    let length1 = 50;

    let rect1 = (50, 30);
    let rect2 = Rectangle { length: 50, width: 30 };
    let rect3 = Rectangle { length: 40, width: 10 };
    let rect4 = Rectangle { length: 45, width: 60 };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(length1, width1)
        // area2(rect1)
        // area3(&rect2)
        rect2.area()
    );
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let sq = Rectangle::square(3);
    println!("Square: {:#?}", sq);
    println!("Square.area: {}", sq.area());
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}