fn main() {
    println!("Hello, world!");

    {
        use std::net::IpAddr;

        let home = "127.0.0.1"
                .parse::<IpAddr>()
                .expect("hardcoded ip address should be valid");
    }
}
