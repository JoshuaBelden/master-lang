struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Point(i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("jcbelden@procaresoftware.com"),
        String::from("jcbelden"),
    );

    let user2 = User {
        email: String::from("joshuabelden@hotmail.com"),
        ..user1
    };
    user1.email = String::from("jcbelden@procaresoftware.com");
    println!("User1 email: {}", user1.email);
    println!("User2 email: {}", user2.email);

    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };
    // println!("Rect 1 is {:?}", rect1);
    // dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let container_rect = Rectangle {
        width: 10,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 5,
        height: 10,
    };

    println!("Can rect1 fit in containerRect? {}", container_rect.can_hold(&rect1)); // Should be false
    println!("Can rect3 fit in containerRect? {}", container_rect.can_hold(&rect3)); // Should be true

    let square = Rectangle::square(10);
    println!("Square is {:?}", square);

}
