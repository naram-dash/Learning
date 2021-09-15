use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // wait on tcp address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // incoming을 통해 연결에 대한 "시도"를 순회한다.
    for stream in listener.incoming() {
        // for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.excute(|| {
            // 연결하기
            handle_connection(stream);
        });
    }
}

// Rust          C/C++
// a: &T     == const T* const a; // can't mutate either
// mut a: &T     == const T* a;       // can't mutate what is pointed to
// [포인터] ====> [실제 값]
// [mutable] ====> [immutable]
//     a: &mut T == T* const a;       // can't mutate pointer
// mut a: &mut T == T* a;             // can mutate both

// 가변 매개변수
// + 소유권
// 인자가 let이 아니라 let mut 으로 넘어왔다고 생각하면 됨
// TcpStream이라고 써서 소유권이 함수로 MOVE됬으니 참조관계는 생각하지 않아도 됨.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // cannot borrow `stream` as mutable, as it is not declared as mutable
    // fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    stream.read(&mut buffer).unwrap();
    // println!("request: {}", String::from_utf8_lossy(&buffer));

    // &[u8, n] literal
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(2));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // make sending
    stream.write(response.as_bytes()).unwrap();
    // wait sending end
    stream.flush().unwrap();
}
