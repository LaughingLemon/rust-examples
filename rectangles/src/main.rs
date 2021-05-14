fn main() {
    let rect1 = Rectangle{ width: 50, height: 200 };
    let rect2 = Rectangle{ width: 15, height: 100 };
    let rect3 = Rectangle{ width: 200, height: 150 };

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_rectangle() {
        let rect1 = Rectangle{ width: 50, height: 200 };
        let rect2 = Rectangle{ width: 15, height: 100 };

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn cant_hold_rectangle() {
        let rect1 = Rectangle{ width: 50, height: 200 };
        let rect3 = Rectangle{ width: 200, height: 150 };

        assert!(!rect1.can_hold(&rect3));
    }
}
