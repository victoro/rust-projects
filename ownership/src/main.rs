fn main() {
    // scoped_test();
    // string_test();
    let s = String::from("A string");
    pass_reference(&s); //ownership is not passed

    pass_string(s); //takes ownership
                    // println!(" trying to use s after passing it => {}", s); //will throw error

    let x = 5;
    pass_integer(x); // x makes use of Copy
    println!("trying to use integer after passing it {}", x); //will not throw error
}

fn pass_reference(s: &String) -> usize {
    println!("refenrence s lenght {}", s.len());
    s.len()
}

fn pass_string(x: String) {
    println!("received string {}", x);
}

fn pass_integer(x: u32) {
    println!(" received integer {}", x);
}

fn _string_test() {
    let s1 = String::from("hwllo");
    let s2 = s1;
    let s3 = s2.clone(); //expensive on memory
                         // println!("print initial string => {}", s1); //will raise error
    println!("print moved string => {}", s2);
    println!("print s2 => {} and cloned s2 from s3 => {}", s2, s3);
}

/// .
fn _scoped_test() {
    let mut s = String::from("Hello");
    s.push_str(" papa");
    let var_1 = 1;
    let var_2 = var_1;
    println!("print initial var 1 => {} ", var_1);
    println!("print var 2 which contains var_1 data => {}", var_2);
    {
        let var_3 = var_1;
        let var_1 = 2;
        s.push_str(" mama");
        println!("print scoped redefined var 1 =>  {}", var_1);
        println!("print scoped var 3 =>  {}", var_3);
        let var_3 = var_1;
        println!("print scoped var 3 after redefined var 1 =>  {}", var_3);
    }
    println!("print var 1 after soped defined var 1 =>  {}", var_1);
    println!("print mut string => {}", s);
}
