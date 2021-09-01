use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("인수 구문분석 동안 에러 ㅂ발생: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
