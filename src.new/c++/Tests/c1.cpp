#include <iostream>
#include <bitset>

int main() {
    std::cout << std::bitset<10>(1) << "+" << std::bitset<10>(1) << "=" << std::bitset<10>(1+1) << std::endl;
}