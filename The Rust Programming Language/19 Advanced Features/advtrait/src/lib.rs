// // using associated types and type placeholder
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// // using generic
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T> {
//         // 생략
//     }
// }

// // 장점
// // 제네릭을 쓰면 구현할 때 마다 타입을 명시해야함
// // 제네릭을 쓰면, 제네릭 타입 매개변수를 지원하는 타입마다 교체해서 여러번 구현해야함
// // 즉슨, 연관 타입을 쓰면 여러 타입에 대해서 여러번 구현할 필요가 없어짐

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         // 생략
//     }
// }
