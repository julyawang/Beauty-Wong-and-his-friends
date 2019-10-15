use std::io;
fn main() {
    println!("输入数字为：");
    let num = wait_for_number();
    println!("进制数为：");
    let hex = wait_for_number();
    assert!(hex > 1);
    let mut result = func(num, hex);
    let length = result.len();
    for i in 0..length {
        print!("{}", result[length - 1 - i]);
    }
}





pub fn wait_for_number() -> u64 {
    let input = wait_for_input();
    input.parse::<u64>().unwrap()
}

pub fn wait_for_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

pub fn func(mut num: u64, hex: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut quotient= 1;
    while quotient > 0 {
        quotient = num / hex;
        let remainder = num % hex;
        num = quotient;
        result.push(remainder);
    }
    result
}

pub fn num_transfer(num: u64, hex: u64) -> u64 {
    let mut remainder = 0;
    if num <= hex {
        num
    } else {
        remainder = &num % &hex;
        remainder + 10 * num_transfer(num/hex, hex)
    }
}