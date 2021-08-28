struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");

    {
        let mut user1 = User {
            // let user1 = User {
            email: String::from("someon@t.com"),
            username: String::from("juho"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("whywhy@naver.com");

        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anothername"),
            ..user1
        };

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        // let origin = Point { ..black };
    }
}
