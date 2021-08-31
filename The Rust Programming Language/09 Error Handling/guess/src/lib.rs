pub struct Guess {
    // 필드가 비공개라서, 직접 Guess 구조체를 만들 수 없다.
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("must be inside");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value()
    }
}
