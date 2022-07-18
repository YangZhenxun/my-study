use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    loop{
        let n1 = rand::thread_rng().gen_range(1..=5);
        let n2 = rand::thread_rng().gen_range(1..=5);
        let maths = n1 + n2;
        let mut a = String::new();
        println!("{n1}+{n2}=");
        io::stdin()
            .read_line(&mut a)
            .expect("读取失败");

        let a:  u32 = match a.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match a.cmp(&maths) {
            Ordering::Equal => println!("你答对了!"),
            Ordering::Greater => {
                println!("你答错了!");
                println!("正确答案是{maths}!");
            },
            Ordering::Less => {
                println!("你答错了!");
                println!("正确答案是{maths}!");
            },
        }
    }
}
