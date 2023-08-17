
fn main() {
    println!("Hello, world!");
    
    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);

        let team_name = String::from("blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        
        println!("{} team score => {}", &team_name, score);
    }

    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);

        for (key, value) in &scores {
            println!("{key}, {value}");
        }
    }

    // data copied or ownership moved when data added
    {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // compile time error <= ownership moved
        // println!("{field_name}: {field_value}");
    }

    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);

        // if emtpy function start with or_ prefix
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    {
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
