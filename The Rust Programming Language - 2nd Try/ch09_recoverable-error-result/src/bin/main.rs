use std::{fs, fs::File, io, io::{Read, ErrorKind}};

fn main() {
    println!("Hello, world!");

    let greeting_file_result = File::open("hello.txt");
    println!("{:?}", greeting_file_result);

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening the file: {:?}", error)
    // };
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", error)
            },
            other_error => panic!("problem opening the file: {:?}", error),
        }
    };

    // using closure
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem on creating inside closure");
            })
        } else {
            panic!("problem on opening inside closure")
        }
    });


    // force open 
    let greeting_file = File::open("hello.txt").unwrap();

    // force open with panic message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err)
    }
}

// with ? operator
// ? => return content of result
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// with ? operator
// more shorter
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// just using exists function
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
