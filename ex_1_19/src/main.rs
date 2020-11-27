use std::io::{self, BufRead};

fn int_overflow(a:i32, b:i32) -> i32 {
    a + b
}

fn get_roman_number(_val: i64) -> &'static str {
    let ret = "";
    return ret
}

#[test]
pub(crate) fn test_get_roman_number() {
    assert!(get_roman_number(0) == "");
    //assert!(get_roman_number(1) == "I");
}


fn main() {
    println!("Number to add:");    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

    // read in b

    println!("Ergebnis: {}",int_overflow(std::i32::MAX, 0));
}
