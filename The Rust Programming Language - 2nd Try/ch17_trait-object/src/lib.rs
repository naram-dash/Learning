pub trait Draw {
    fn draw(&self);
}

// 특성개체는 제네릭과는 다르다
// 제네릭은 각 타입별로 실제 스트럭처와 메소드가 생기고(그래서 다른 타입끼리 하나의 벡터안에 안들어감)
pub struct Screen {
    // the size for values of type `(dyn Draw + 'static)` cannot be known at compilation time
    // the trait `Sized` is not implemented for `(dyn Draw + 'static)
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|component| {
            component.draw();
        }) 
    }
}

