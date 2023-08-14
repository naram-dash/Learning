fn main() {
    println!("Hello, world!");

    {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    }
    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
    }

    {
        let black = Color(0,0,0);
        let origin = Point(0,0,0);

        let white = Color { 0: 1, 1: 1, 2: 1, ..black};
        // error
        // let dodo = Color { ..origin};
    }

    {
        let subject = AlwaysEqual;
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

// missing lifetime specifier
struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
