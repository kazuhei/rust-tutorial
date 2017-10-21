extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字を当ててみて！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("予想値を入力してください。");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("行の読み取りに失敗しました。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます！"),
            Ordering::Greater => println!("大きすぎます！"),
            Ordering::Equal =>  {
                println!("あなたの勝ちです。");
                break;
            },
        }
    }
}
