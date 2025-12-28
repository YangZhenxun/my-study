#include <iostream>

int main(void)
{
    std::cout << "Fake Terminal\nPowered by exe.\nVersion:0.1."<<std::endl;
    while (true) {
        std::cout << "PS T:\\Users\\Admin\\>";
        char userinput;
        char something;
        std::cin >> userinput >> something;
    }
}
