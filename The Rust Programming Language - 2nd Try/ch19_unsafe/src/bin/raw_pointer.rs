use std::slice;

// 이 언어에 있는 함수를 가져온다?
extern "C" {
    fn abs(input: i32) -> i32;
}

// 러스트 함수를 외부로 노출할때는 함수마다 extern을 붙인다.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("just called a rust function from c");
}

fn main() {
    {
        let mut num = 5;
    
        // 같은 메모리 위치에 대해서 서로 다른 원시 포인터 두개가 생성됨
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
    
        // 주소값
        let address = 0x012345usize;
        // 주소값을 참조로 강제변환
        let r = address as *const i32;
    
        // 원시 포인터를 역참조할때만 unsafe
        unsafe {
            println!("r1 is {}", *r1);
            println!("r2 is {}", *r2);

            // 에러남
            // println!("r is {}", *r);
        }
    }

    {
        // unsafe fn은 unsafe block에서 부를수 있다.
        // compile time error
        // dangerous();

        // unsafe {
        //     dangerous();
        // }
    }

    {
        let mut v = vec![1,2,3,4,5,6];
        let r = &mut v[..];

        let (a,b) = r.split_at_mut(3);

        // 이 함수는 내부 구현에 unsafe를 사용한다.
        // pub const fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
        //     assert!(mid <= self.len());
        //     // SAFETY: `[ptr; mid]` and `[mid; len]` are inside `self`, which
        //     // fulfills the requirements of `from_raw_parts_mut`.
        //     unsafe { self.split_at_mut_unchecked(mid) }
        // }

        assert_eq!(a, &mut [1,2,3]);
        assert_eq!(b, &mut [4,5,6]);
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    {
        // 불변 전역변수는 safe이다.
        println!("name is {}", HELLO_WORLD);
    }

    {
        add_to_count(3);

        // 변경가능한 전역 변수는 변경, 사용 모두 unsafe 이다
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    {
        let i32num = 23;
        i32num.say();
    }
}

static HELLO_WORLD: &str = "Hello world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 변경가능한 전역 변수는 변경, 사용 모두 unsafe 이다
    // 데이터 경합이 발생하기 쉽기 때문
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {
    // 이안에서는 따로 unsafe 지정 안해도 unsafe 기능 사용 가능
}

fn split_at_mut(values: &mut[i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // compile time error
    // second mutable borrow occurs here
    // (&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

trait Carense2 {
    fn say(&self) {
        println!("carense2!");
    }
}

impl Carense2 for i32 {
}