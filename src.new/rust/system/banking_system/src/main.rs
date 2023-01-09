use serde::{Deserialize, Serialize};
use std::{io::{self, Read, Write}, collections::HashMap, process, path::Path, fs::*};
use rand::Rng;
pub struct Admin{
    admin : String,
    passwd: i64,
}
impl Admin{
    fn admin_view(&self){
        println!("**********************************************");
        println!("*                                            *");
        println!("*                                            *");
        println!("*               欢迎登录动物银行             *");
        println!("*                                            *");
        println!("*                                            *");
        println!("**********************************************");
    }
    fn func_view(&self){
        println!("**********************************************");
        println!("*           开户（1）     查询（2）          *");
        println!("*           取款（3）     存款（4）          *");
        println!("*           转账（5）     改密码（6)         *");
        println!("*           锁定（7）     解锁（8）          *");
        println!("*           补卡（9）     销户（10）         *");
        println!("*                   退出（0）                *");
        println!("**********************************************");
    }
    fn check(&self) -> bool{
        println!("请输入管理员账号：");
        let mut inputadmin = String::new();
        io::stdin().read_line(&mut inputadmin).expect("读取错误！");
        if inputadmin.trim() != self.admin{
            println!("账号输入有误！");
            return true;
        }
        println!("请输入管理员密码：");
        let mut inputpasswd = String::new();
        io::stdin().read_line(&mut inputpasswd).expect("读取错误！");
        let inpasswd:i64 = match inputpasswd.trim().parse(){
            Ok(num) =>num,
            Err(_) => {
                println!("密码输入错误！");
                return true;
            }
        };
        if inpasswd != self.passwd{
            println!("密码输入有误！");
            return true;
        }
        println!("操作成功，请稍后...");
        return false;
    }
    pub fn new() -> Admin{
        Admin{admin:String::from("1"), passwd:1}
    }
}
#[derive(Serialize,Deserialize)]
pub struct CAP{
    cardid: String,
    cardpasswd: String,
    cardmoney: i64,
    cardlock: bool,
    name: String,
    idCard: String,
    phone: String,
    card: Card,
}


pub struct ATM{
    allusers: CAP,
}
impl ATM{
    fn creat_user(&mut self) -> bool{
        println!("请输入你的姓名：");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("读取失败！");
        println!("请输入你的身份证号：");
        let mut idcard = String::new();
        io::stdin().read_line(&mut idcard).expect("读取失败！");
        println!("请输入你的电话号码：");
        let mut phone = String::new();
        io::stdin().read_line(&mut phone).expect("读取失败！");
        println!("请输入你的预存款金额：");
        let mut premoney = String::new();
        io::stdin().read_line(&mut premoney).expect("读取失败！");
        let intpremoney: i64 = match premoney.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("预存款输入有误，开户失败！");
                return true;
            }
        };
        if intpremoney < 0{
            println!("预存款输入有误，开户失败！");
            return true;
        }
        println!("请输入密码：");
        let mut ownpasswd = String::new();
        io::stdin().read_line(&mut ownpasswd).expect("读取失败！");
        if !self.check_passwd(&ownpasswd){
            println!("密码输入有误，开户失败...");
            return true;
        }
        let cardid = self.creat_cardid();
        let cardn = CAP{&cardid, &ownpasswd, intpremoney};
        let usern = CAP{ name:name, idCard:idcard, phone:phone, card:cardn };
        self.allusers.insert(cardid.clone(), usern);
        println!("开户成功，请牢记卡号：{}",&*cardid);
        true
    }
    fn check_passwd(&self, passwd:&String) -> bool{
        for _i in 1..=3{
            println!("请确认密码：");
            let mut tempasswd = String::new();
            io::stdin().read_line(&mut tempasswd).expect("读取失败！");
            if tempasswd == *passwd{
                return true;
            }
        }
        false
    }
    fn creat_cardid(&self) -> String{
        loop{
            let mut strn = String::new();
            let mut rng = rand::thread_rng();
            for _i in 1..=6{
                let ia1 = rng.gen_range(1..=9).to_string();
                strn += &ia1;
            }
            if !self.allusers.contains_key(&*strn){
                return strn;
            }
        }
    }
    fn search_userinfo(&mut self) -> bool{
        println!("请输入你的卡号：");
        let mut abd = String::new();
        io::stdin().read_line(&mut abd).expect("读取失败！");
        let abe = &*abd.trim();
        match self.allusers.get_mut(abe){
            Some(usr) => {
                if usr.card.cardlock == true{
                    println!("该卡已经锁定！");
                    return true;
                }
                for _i in 1..=3{
                    println!("请确认密码：");
                    let mut tempasswd = String::new();
                    io::stdin().read_line(&mut tempasswd).expect("读取失败！");
                    if tempasswd != usr.card.cardpasswd{
                        println!("密码错误，该卡已经被锁定...");
                        usr.card.cardlock = true;
                        return true;
                    }
                    else{
                        break;
                    }
                }
                println!("账户：{}      余额：{}",usr.card.cardid, usr.card.cardmoney);
                false
            },
            _ => {
                println!("该卡号不存在！查询失败！");
                true
            }
        }
    }
    fn get_money(&mut self) -> bool{
        println!("请输入你的卡号：");
        let mut abd = String::new();
        io::stdin().read_line(&mut abd).expect("读取失败！");
        let abe = &*abd.trim();
        match self.allusers.get_mut(abe){
            Some(usr) => {
                if usr.card.cardlock == true{
                    println!("该卡已经锁定！");
                    return true;
                }
                for _i in 1..=3{
                    println!("请确认密码：");
                    let mut tempasswd = String::new();
                    io::stdin().read_line(&mut tempasswd).expect("读取失败！");
                    if tempasswd != usr.card.cardpasswd{
                        println!("密码错误，该卡已经被锁定...");
                        usr.card.cardlock = true;
                        return true;
                    }
                    else{
                        break;
                    }
                }
                println!("请输入您的取款金额：");
                let mut get_money = String::new();
                io::stdin().read_line(&mut get_money).expect("读取失败！");
                let get_moneyint:i64 = match get_money.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("请输入数字！");
                        return true;
                    }
                };
                let mut now_money = usr.card.cardmoney;
                if get_moneyint > now_money{
                    println!("余额不足，取款失败...");
                    return true;
                }
                now_money -= get_moneyint;
                usr.card.cardmoney = now_money;
                println!("取款成功，您当前余额为：{}",usr.card.cardmoney);
                false
            },
            _ => {
                println!("该卡号不存在！取款失败！");
                true
            }
        }
    }
    fn save_money(&mut self) -> bool{
        println!("请输入你的卡号：");
        let mut abd = String::new();
        io::stdin().read_line(&mut abd).expect("读取失败！");
        let abe = &*abd.trim();
        match self.allusers.get_mut(abe){
            Some(usr) => {
                if usr.card.cardlock == true{
                    println!("该卡已经锁定！");
                    return true;
                }
                for _i in 1..=3{
                    println!("请确认密码：");
                    let mut tempasswd = String::new();
                    io::stdin().read_line(&mut tempasswd).expect("读取失败！");
                    if tempasswd != usr.card.cardpasswd{
                        println!("密码错误，该卡已经被锁定...");
                        usr.card.cardlock = true;
                        return true;
                    }
                    else{
                        break;
                    }
                }
                println!("请输入您的存款金额：");
                let mut save_money = String::new();
                io::stdin().read_line(&mut save_money).expect("读取失败！");
                let save_moneyint:i64 = match save_money.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("请输入数字！");
                        return true;
                    }
                };
                let mut now_money = usr.card.cardmoney;
                now_money += save_moneyint;
                usr.card.cardmoney = now_money;
                println!("存款成功，您当前余额为：{}",usr.card.cardmoney);
                false
            },
            _ => {
                println!("该卡号不存在！存款失败！");
                true
            }
        }
    }
    pub fn new(allusers:HashMap<String, Person>)-> ATM{
        ATM {allusers:allusers}
    }
}
fn main(){
    let ad = Admin::new();
    let allusers = HashMap::new();
    let mut atm = ATM::new(allusers);
    ad.admin_view();
    if ad.check(){
        process::exit(-1);
    }
    let filepath = "userinfo.txt";
    if !Path::new("userinfo.txt").exists(){
        File::create("userinfo.txt").expect("无法创建");
    }
    let size = metadata("userinfo.txt").expect("无法读取！").len();
    if size != 0{
        let mut f = File::open(filepath).expect("读取失败！");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("读取失败！");
    }
    loop{
        ad.func_view();
        println!("请输入您的操作：");
        let mut opt = String::new();
        io::stdin().read_line(&mut opt).expect("读取失败！");
        let opt = opt.trim();
        match opt {
            "1" => {
                if atm.creat_user(){
                    continue;
                }
            },
            "2" => {
                if atm.search_userinfo(){
                    continue;
                }
            },
            "3" => {
                if atm.get_money(){
                    continue;
                }
            },
            "4" => {
                if atm.save_money(){
                    continue;
                }
            },
            "0" => {
                if !ad.check(){
                    let mut f = File::open(filepath).expect("读取失败！");
                    let lbuf = serde_json::to_string(&allusers);
                    f.write_all(allusers.as_bytes()).expect("写入失败！");
                    process::exit(0);
                }
                else{
                    continue;
                }
            },
            _ => {
                continue;
            },
        }
    }
}
