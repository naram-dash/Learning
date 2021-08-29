fn main() {
    // 1. Vector
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4, 5];

        let third = &v[2];
        println!("third {}", third);

        // let no_exist = &v[100];
        let no_exist = v.get(100);
    }
    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);

        for i in &v {
            println!("{}", i);
        }

        //
        for i in &mut v {
            // 역참조 연산자
            // mut 참조로 가져오더라도 역참조로 가서 바꿔야함
            *i += 50;
        }
    }
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("blue")),
        ];
    }

    // 2. String
    // Vector 대부분의 작업을 지원함
    {
        let mut s = String::new();

        // &str
        let data = "문자열 초기값";

        // String
        let s = data.to_string();

        let s = "문자열 초기값".to_string();
    }

    {
        let s = String::from("문자열 초기값");

        // utf-8 지원
    }
    {
        let mut s = String::from("foo");
        let s2 = "bar";
        // s2는 &str immutable refrence로 전달된다.
        s.push_str(s2);
        s.push_str(s2);

        println!("{}", s);
        println!("{}", s2);

        s.push('l');
        println!("{}", s);
    }
    {
        let s1 = String::from("hello, ");
        let s2 = String::from("world");

        // 소유권이 + 연산자로 넘어가면서 s1의 소유권이 결과적으로 사라짐
        // s2는 참조만 넘겨서 살아남음
        // 내부적으로는 add 메소드를 호출함
        let s3 = s1 + &s2;

        // println!("s1 {}", s1);
        // println!("s3 {}", s3);
        println!("s3 {}", &s3);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s = s1 + "-" + &s2 + "-" + &s3;
        let s = format!("{}-{}-{}", s1, s2, s3);
        // sprintf 같은거
        // format을 사용하면 인자로 넘긴 모든 값의 소유권이 그대로 유지된다. (+) 와 다른 점

        println!("{}", s1);
        println!("{}", s2);

        println!("{}", &s)
    }

    {
        let s1 = String::from("hello");
        // let h = s1[0];

        // string은 Vec<u8>을 한번 감싼 타입
        // u8 => 8bit unsigned
        // 즉슨 문자 인코딩에 따라 끊어야하는 시점이 다를수 있다는 것이다.

        let len = String::from("hola").len();
        // 4byte => 영문자 글자당 1byte
        println!("len: {}", len);
        let len = String::from("안녕하세요").len();
        // 15byte => 유니코드 타입 글자당 3byte
        println!("len: {}", len);
    }

    {
        let hello = "안녕하세요";
        let s = &hello[0..3];

        println!("{}", s);

        for c in hello.chars() {
            println!("{}", c);
        }
        for c in hello.bytes() {
            println!("{}", c);
        }
    }

    // 3. HashMap
    use std::collections::HashMap;
    {
        let mut score = HashMap::new();

        score.insert(String::from("블루"), 10);
        score.insert(String::from("예로"), 50);
    }
    {
        let teams = vec![String::from("블루"), String::from("옐로")];
        let init_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    }

    // copy vs drop
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("블룰");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // 소유권이 이동
        // println!("{}", field_name);
    }
    // get
    {
        let teams = vec![String::from("블루"), String::from("옐로")];
        let init_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

        let team_name = String::from("블루파랑");
        // 키를 참조로 받는다. 굳이 소유권 줄필요 없긴 함
        let score = scores.get(&team_name);

        match score {
            Some(v) => println!("{}", v),
            None => println!("not found"),
        };

        for (key, value) in scores {
            println!("{}: {}", key, value);
        }
    }

    // edit
    {
        //insert로 값 덮어쓰기
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"), 10);
        scores.insert(String::from("블루"), 50);

        println!("{:?}", scores);
    }

    {
        //entry = 값이 있는지 확인
        // or_insert =
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"), 10);

        // 기본값 느낌
        scores.entry(String::from("옐로")).or_insert(5);
        scores.entry(String::from("블루")).or_insert(50);

        println!("{:?}", scores);
    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
            // 역참조로 들어가서 값 변경
        }

        println!("{:?}", map);
    }
}
