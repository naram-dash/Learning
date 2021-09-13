// lifetime subtyping
// 하나의 라이프타임이 다른것보다 오래사는 것을 보장

// expected named lifetime parameter
// Context 내부의 값이 Context보다 빨리 사라져서는 안되기 때문에
// 라이프타임 파라미터가 필요
struct Context<'s>(&'s str);
struct Parser<'c, 's> {
    // 앞 뒤로 다 써야함
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    // 라이프타임 생략 규칙에 따라 자동으로 추론되는 타입
    // fn parse<'a>(&'a self) -> Result<(), &'a str> {

    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

// string 슬라이스의 라이프타임
fn parse_context(context: Context) -> Result<(), &str> {
    // 함수가 context의 소유권을 가지고 있는 상태에서
    // 반환 타입이 가지고 있는게 borrow된 값이니,
    // 함수가 끝나는 시점에서 borrow한 값도 의미 없다
    // ERROR

    // Parser와 그안의 context는 인자보다 오래 살아야한다는 이야기
    Parser { context: &context }.parse()
}
