use std::marker::{Send, Sync};

fn main() {
    println!("Hello, world!");
    
    // Rc는 스레드 안정성을 지불하고 싶지 않은 한정적인 상황
    // 으로 Send 트레이트가 구현되지 않음

    // sync는 멀티스레드 세이프하다는것
    // send인 경우 sync이다
    // mutex는 sync이다

    // send sync는 자동으로 구현된다.
    // marker 안에 trait은 구현할 메소드도 없다
    // 수동구현은 안전하지 않다
    // 수동구현은 unsafe 코드가 필요하다
    // 더 많은 정보는 러스트노미콘 참고
    //
    // pub unsafe auto trait Send {
    //     // empty.
    // }
    // pub unsafe auto trait Sync {
    //     // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
    //     // lands in beta, and it has been extended to check whether a closure is
    //     // anywhere in the requirement chain, extend it as such (#48534):
    //     // ```
    //     // on(
    //     //     closure,
    //     //     note="`{Self}` cannot be shared safely, consider marking the closure `move`"
    //     // ),
    //     // ```
    
    //     // Empty
    // }

    // 동시성 처리는 언어의 일부가 아님
    // 크레이트를 찾아보자
}
