use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1,101);

    loop{
        println!("请猜测一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取行动失败");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_num){
            Ordering::Less => {
                println!("太小了");
            },
            Ordering::Greater => {
                println!("太大了");
            },
            Ordering::Equal => {
                println!("获胜！");
                break;
            },
        }
    }
}
