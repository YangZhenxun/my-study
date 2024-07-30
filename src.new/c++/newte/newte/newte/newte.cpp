#include <iostream>
#include <random>
#include <string>
#include <fileapi.h>
class Admin
{
public:
	std::string static admin;
	std::string static passwd;

	static void AdminView()
	{
		std::cout << "**********************************************";
		std::cout << "*                                            *";
		std::cout << "*                                            *";
		std::cout << "*               欢迎登录动物银行             *";
		std::cout << "*                                            *";
		std::cout << "*                                            *";
		std::cout << "**********************************************";
	}
	static void FunctionView()
	{
		std::cout << "**********************************************" << std::endl;
		std::cout << "*           开户（1）     查询（2）          *" << std::endl;
		std::cout << "*           取款（3）     存款（4）          *" << std::endl;
		std::cout << "*           转账（5）     改密码（6)         *" << std::endl;
		std::cout << "*           锁定（7）     解锁（8）          *" << std::endl;
		std::cout << "*           补卡（9）     销户（10）         *" << std::endl;
		std::cout << "*                  退出（0）                 *" << std::endl;
		std::cout << "**********************************************" << std::endl;
	}
	static int Check()
	{
		std::string input_Admin;
		std::cout << "请输入管理员账户：";
		std::getline(std::cin, input_Admin);
		if (admin == input_Admin)
		{
			std::cout << "账号输入错误！";
			return -1;
		}
		std::string input_Passwd;
		std::cout << "请输入管理员密码：";
		std::getline(std::cin, input_Admin);
		if (passwd == input_Passwd)
		{
			std::cout << "密码输入错误！";
			return -1;
		}
		std::cout << "操作成功，请操作......";
		return 0;
	}
};
std::string Admin::admin = "YangZhenxun";
std::string Admin::passwd = "Yzx20120413";

class Card
{
private:
	std::string name;
	std::string id_card;
	std::string phone;
	std::string card;
public:
	Card()
	{
		Init();
	}
	static void Init(std::string k_name, std::string k_id_card, std::string k_phone, std::string k_card)
	{

	}
};
