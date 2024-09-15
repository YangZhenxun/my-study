#include <iostream>
#include <string>
using namespace std;

int main() {
    char x;
    cin >> x;
    int y = 3;
    int z = 2*y;
    for (int i = 1; i <= 3; i++){
        string y((z-(2*i-1))/2, ' ');
        string a(i*2-1, x);
        cout << y << a << y << endl;
    }
}