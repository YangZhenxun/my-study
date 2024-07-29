// Tests.cpp : 此文件包含 "main" 函数。程序执行将在此处开始并结束。
//

#include <iostream>
#include <functional>
#include <type_traits>
/*
using Task = std::function<void()>;

template<typename T>
void CallTaskFunc(const T& f) {
    if (f.operator bool()) {
        f();
    }
    else {
        std::cout << "f‘s callable object not set, not callable" << std::endl;
    }
}

void cb() {
    std::cout << "call back" << std::endl;
}

int main() {
    Task t1;        //不可调用对象
    Task t2 = cb;   //可调用对象
    int x =  3;

    CallTaskFunc(t1);
    CallTaskFunc(t2);
    CallTaskFunc(x);

    return 0;
}

// 运行程序: Ctrl + F5 或调试 >“开始执行(不调试)”菜单
// 调试程序: F5 或调试 >“开始调试”菜单

// 入门使用技巧: 
//   1. 使用解决方案资源管理器窗口添加/管理文件
//   2. 使用团队资源管理器窗口连接到源代码管理
//   3. 使用输出窗口查看生成输出和其他消息
//   4. 使用错误列表窗口查看错误
//   5. 转到“项目”>“添加新项”以创建新的代码文件，或转到“项目”>“添加现有项”以将现有代码文件添加到项目
//   6. 将来，若要再次打开此项目，请转到“文件”>“打开”>“项目”并选择 .sln 文件

*/


template <typename _Function> auto _IsCallable(_Function _Func, int) -> decltype(_Func(), std::true_type()) { (_Func); return std::true_type(); }
template <typename _Function> std::false_type _IsCallable(_Function, ...) { return std::false_type(); }

bool cdeffg() {
    return true;
}

int main() {
    std::cout << _IsCallable("100") << std::endl;
    std::cout << _IsCallable(cdeffg, 190) << std::endl;
    std::cout << _IsCallable(cdeffg(), 2) << std::endl;
}