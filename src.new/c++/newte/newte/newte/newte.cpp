#include <iostream>
#include <random>
#include <string>

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
		std::cout << "*               ��ӭ��¼��������             *";
		std::cout << "*                                            *";
		std::cout << "*                                            *";
		std::cout << "**********************************************";
	}
	static void FunctionView()
	{
		std::cout << "**********************************************" << std::endl;
		std::cout << "*           ������1��     ��ѯ��2��          *" << std::endl;
		std::cout << "*           ȡ�3��     ��4��          *" << std::endl;
		std::cout << "*           ת�ˣ�5��     �����루6)         *" << std::endl;
		std::cout << "*           ������7��     ������8��          *" << std::endl;
		std::cout << "*           ������9��     ������10��         *" << std::endl;
		std::cout << "*                  �˳���0��                 *" << std::endl;
		std::cout << "**********************************************" << std::endl;
	}
	static int Check()
	{
		std::string input_Admin;
		std::cout << "���������Ա�˻���";
		std::getline(std::cin, input_Admin);
		if (admin == input_Admin)
		{
			std::cout << "�˺��������";
			return -1;
		}
		std::string input_Passwd;
		std::cout << "���������Ա���룺";
		std::getline(std::cin, input_Admin);
		if (passwd == input_Passwd)
		{
			std::cout << "�����������";
			return -1;
		}
		std::cout << "�����ɹ��������......";
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
