#include "testrand.h"

int testrandom(){
    //Creat a random devcie.
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution < > dis(1, 11);
    while (true) {
        //Creat random numbers.
        int i1 = dis(gen);
        int i2 = dis(gen);
        std::cout << i1 << "+" << i2 << "=";
        int input;
        std::cin >> input;
        if (!std::cin.good()) {
            std::cout << "请输入数字！" << std::endl;
        }else {
            int math = i1 + i2;
            if (input == math) {
                std::cout << "你答对了！" << std::endl;
            }else {
                std::cout << "你答错了！\n正确答案是：" << math << std::endl;
            }
        }
        //清除数据
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
    }
    return 0;
}
