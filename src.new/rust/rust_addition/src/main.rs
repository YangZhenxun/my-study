use rand::Rng;
use  std::io;

fn main() {
    loop {
        let mut rng = rand::thread_rng();
        let i1 = rng.gen_range(1..11);
        let i2 = rng.gen_range(1..11);
        println!("{i1}+{i2}=");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("无法读取！");
        let input: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let math = i1 + i2;
        if math == input{
            println!("你答对了！");
        }
        else{
            println!("你答错了！");
            println!("正确答案是{math}");
        }
    }
}
