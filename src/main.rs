use rand::Rng;
use std::io::{self};



fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("please input you guess");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_)=> continue,
        };

        println!("you guessed: {}", guess);

        if guess > secret_number {
            println!("your guess is high");
        }else if guess < secret_number {
            println!("your guess is low");
        }else {
            println!("you win");
            break;
        }
    }




}

// let x: i32 = 4;
// if x > 5 {
//     println!("x is greater than 5")
// } else if x<2{
//     println!("x is less than 2")
// }else{
//     println!("x")
// }

// //unsigned integer
// let unsigned: u8 = 10;

// //signed integer
// let signed: u8 = 33;

// //float used for decimal point
// let float: f32 = 33.5;

// //char
// let letter: char = 'a';

// //bool
// let is_true: bool = true;

// //array
// let arr: [i32; 5] = [-1, 2, 3, 4, 5];

// //tuples can store different types of data
// let age: (i32, f64, u8) = (500, 6.4, 1);

// let z = age.2;

// println!("unsigned_interger{}", unsigned);
// println!("signed_interger {}", signed);
// println!("float {}", float);
// println!("char {}", letter);
// println!("istrue {}", is_true);
// println!(" {}", arr[0]);
// println!(" {}", arr[0]);
// println!(" {}", z);

// another_function(8, 'n');

// fn another_function(z:i32, alphabet: char) {
//     println!("another function:{},alphabet:{}", z,alphabet);
// }
