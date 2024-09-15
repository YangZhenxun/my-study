#include <iostream>
#include <cmath>
using namespace std;

#define mid(a) \
    floor(a)

#define big(a) \
    mid(a) - a

class f{
    int num;
    int del;
public:
    f(int n, int d): num(n), del(d) {}
};

int main(){
    cout << big(2.9);
}