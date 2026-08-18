fn structure () {
    let user1 = User {
        active: true,
        username: String::form("someusername123"),
        email: String::form("someone@example.com"),
        sign_in_count: 1
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    /// you can write it to below
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct User {
    active: bool, // it call fields
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/// do same build_user but it group email and username to construct User
/// not recommend it confuse to new rust user.
fn build_user_shothand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

/// structure + method
struct Rectangle {
    width: u32,
    height: u32
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

    fn perimeter(&self) -> u32 {
        (2 * self.width) + (2 * self.height)
    }
}
