use std::io;
use std::collections::HashMap;

struct Preson{
    name: Box<str>,
    idcard: Box<str>,
    phone: Box<str>,
    card: Box<str>,
}

struct Card{
    cardid: Box<str>,
    cardpasswd: Box<str>,
    cardmoney: u32,
    cardlock: bool,
}

struct Admin{
    adminname: &'static str,
    adminid: &'static str,
}

impl Admin{
    fn new() -> Admin {
        let adminname: &'static str = "YangZhenxun";
        let adminid: &'static str = "Yzx20120413";
        Admin{
            adminname: adminname,
            adminid: adminid,
        }
    }
}

impl Admin{
    fn AdminView(&self){
        println!("**********************************************");
        println!("*                                            *");
        println!("*                                            *");
        println!("*               欢迎登录联盟银行             *");
        println!("*                                            *");
        println!("*                                            *");
        println!("**********************************************");
    }
}

impl Admin{
    fn FuncionView(&self){
        println!("**********************************************");
        println!("*           开户（1）     查询（2）          *");
        println!("*           取款（3）     存款（4）          *");
        println!("*           转账（5）     改密码（6)         *");
        println!("*           锁定（7）     解锁（8）          *");
        println!("*                   退出（0）                *");
        println!("**********************************************");
    }
}

impl Admin{
    fn Check(&self) -> i32 {
        let mut something = String::new();
        println!("请输入管理员账号：");
        io::stdin()
            .read_line(&mut something)
            .expect("读取内容失败！");
        let inputadmin = something.trim();
        if inputadmin != self.adminname{
            println!("账号输入错误！");
            return -1;
        }
        let mut something2 = String::new();
        println!("请输入管理员密码：");
        io::stdin()
           .read_line(&mut something2)
           .expect("读取内容失败！");
        let inputadminid = something2.trim();
        if inputadminid != self.adminid{
            println!("密码输入错误！");
            return -1;
        }
        println!("操作成功，请稍后...");
        0
    }
}

struct ATM{
    allusers: HashMap<_,_>,
}

impl ATM{
    fn CreatUser(&self){
        let mut something = String::new();
        println!("请输入您的姓名：");
        io::stdin()
           .read_line(&mut something)
           .expect("读取内容失败！");
        let name = something.trim();
        let mut something2 = String::new();
        println!("请输入您的身份证号：");
        io::stdin()
          .read_line(&mut something2)
          .expect("读取内容失败！");
        let phone = something2.trim()
        
    }
}

fn main(){
    let mut admin = Admin::new();
    admin.AdminView();
    admin.Check();
}
