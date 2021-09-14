// 선언적 매크로
#[macro_export]
macro_rules! vec {
    // 이게 하나의 패턴 가지가 된다.
    ( $ ($x:expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("hi");
            )*
            temp_vec
        }
    };
}

// attribute-like macro
// #[route(GET, "/")]
// fn index(){

// }

fn main() {
    println!("Hello, world!");
    let v: Vec<u32> = vec![1, 2, 3];

    // 함수형 매크로는 절차적으로 코드를 분석할 수 있으므로
    // match 같은 패턴 매칭만 사용하는 선언형 매크로보다 구문 분석에 적합하다.
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
}
