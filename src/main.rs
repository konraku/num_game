use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数あてゲームだよ、お兄ちゃん！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("好きな数字を入力してね！");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("ふぇぇ...エラーだよぅ");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("入力した数字は{}だね！", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎるよ、お兄ちゃん！"),
            Ordering::Greater => println!("大きすぎるよ、お兄ちゃん！"),
            Ordering::Equal => {
                println!("ピンポーン！正解だよ！");
                break;
            }
        }
    }
}
