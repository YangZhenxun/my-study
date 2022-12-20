#include <iostream>
#include <random>
#include <ctype.h>
#include <string>
using namespace std;

int main(){
    default_random_engine e;
    uniform_int_distribution<unsigned> u(1, 10);
    while (true){
        int num1 = u(e);
        int num2 = u(e);
        cout << num1 << "+" << num2 << "=";
        try{
            int input1;
            cin >> input1;
            if (not cin.good()){
                throw input1;
            }
            int math = num1 + num2;
            if (math == input1){
                cout << "你答对了！" << endl;
            } else {
                cout << "你错了！" << endl;
                cout << "正确答案是：" << math << endl;
            }
        }
        catch(...){
            cout << "请输入数字！" << endl;
        }
    }
}

