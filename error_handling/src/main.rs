use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // force_err();
    // handle_err();
    handle_err2();
}

fn _force_err() {
    let file_result = File::open("file.txt");
    let a_file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };
}

fn _handle_err() {
    let file_result = File::open("file.txt");
    let a_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(f) => f,
                Err(error) => panic!("we have an error {:?}", error),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error);
            }
        },
    };
}

fn handle_err2() {
    let a_file = File::open("second_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("second_file.txt").unwrap_or_else(|error| {
                panic!("Error creating file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}
