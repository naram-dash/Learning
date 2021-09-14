use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// 이거 돌릴 땐, 내부 derive procedure macro crate 부터 빌드한 다음
// 그다음에 path를 이용한 디펜던시 지정 후 본 crate run 하기!!
