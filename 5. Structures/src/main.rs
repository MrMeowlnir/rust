#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    // Методы принимают &self в качестве аргумента
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    // Ассоциированные функции не принимают &self в качестве аргумента
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect);

    println!("The area of {rect:?} is {}", rect.area());

    let rect1 = Rectangle{
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

    let sq = Rectangle::square(50);

    println!("{rect1:?} can hold {rect2:?}? {}", rect1.can_hold(&rect2));
    println!("{rect1:?} can hold {rect3:?}? {}", rect1.can_hold(&rect3));
    println!("{rect1:?} can hold {sq:?}? {}", rect1.can_hold(&sq));
}


