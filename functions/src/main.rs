fn main() {
    println!("playing with functions");
    another_function(89);
    labeled_params(70, '3', 3.14);
    statement_test_1(15);

    println!("display a expresion function value => {}", return_char());
}

// simple param
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn labeled_params(label_1: i32, label_2: char, label_3: f64) {
    println!("printing first label => {}", label_1);
    println!("printing second label => {}", label_2);
    println!("printing third label => {}", label_3);
}

fn statement_test_1(y: i32) {
    // let x = let y = 15; won`t work
    let x = {
        println!(" y => {}", y);
        y
    };
    println!("from y param {}, x => {}", y, x);
    let x = {
        let y = 225;
        println!("from redefined y {}, x => {}", y, x);
        y
    };

    println!("function global score y {}, x => {}", y, x);
}

// function with return data

fn return_char() -> char {
    'a'
}
