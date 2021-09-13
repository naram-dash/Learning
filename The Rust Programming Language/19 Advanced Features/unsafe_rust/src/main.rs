use std::slice;

fn main() {
    {
        // raw pointer without unsafe
        // make raw pointer from reference
        let mut num = 5;
        // immutable raw pointer
        let r1 = &num as *const i32;
        // mutable raw pointer
        let r2 = &mut num as *mut i32;

        // dereference of raw pointer is unsafe and requires unsafe function or block
        // println!("{}", *r1);
    }

    // very unsafe code
    {
        let address = 0x012345usize;
        let r = address as *const i32;

        unsafe {
            // println!("{}", *r); // errror
            // error: process didn't exit successfully:
            // `target\debug\unsafe_rust.exe`
            // (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
        }
    }

    // with unsafe
    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 = {}", *r1);
            println!("r2 = {}", *r2);
            *r2 = 3;
            println!("r1 = {}", *r1);
            println!("r2 = {}", *r2);
        }
    }

    {
        unsafe fn dangerous() {}

        // must be inside unsafe
        // dangerous();

        unsafe {
            dangerous();
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    {
        let address = 0x012345usize;
        let r = address as *mut i32;

        let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    }

    unsafe {
        println!("by Clang, abs of -3 is {}", abs(-3));
    }

    {
        println!("word: {}", HELLO_WORLD);
    }
}

// usize
// The size of this primitive is how many bytes it takes to reference any location in memory.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            // isize
            // The size of this primitive is how many bytes it takes to reference any location in memory.
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("hello from rust");
}

static HELLO_WORLD: &str = "hello world!";

static mut COUNTER: u32 = 0;

// 정적 변수의 변경은 unsafe에서 해야한다.
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 안전하지 않은 트레이트
unsafe trait foo {}

// 안전하지 않은 트레이트 구현
unsafe impl foo for i32 {}
