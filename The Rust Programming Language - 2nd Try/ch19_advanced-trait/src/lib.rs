use std::iter::Iterator;

struct Counter {

}

// associated type을 사용하면 Counter 타입에 대해서 Item 타입을 여러개 만들 수 없다
// impl Iterator for Counter {
    // type Item = u32;
    // fn next(&mut self) -> Option<Self::Item> {
        
    // }
// }
