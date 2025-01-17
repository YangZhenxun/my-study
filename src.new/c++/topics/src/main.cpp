#include <iostream>
#include <string>
#include <cmath>
using namespace std;

inline void told(int un, int r, int a){
    if (r == 1){
        switch (un){
            case 2: cout << 1 << endl;return;
            case 3: cout << 7 << endl;return;
            case 4: cout << 4 << endl;return;
            case 5: cout << 2 << endl;return;
            case 6: cout << (a==1?6:0) << endl;return;
            case 7: cout << 8 << endl;return;
            default: cout << -1 << endl;
        }
    }
    else{
        int z = 1;
        int k = un - 2;
        if (k%7!=0&&k/7>=r-1){
            k = un-5;
            z = 2;
        }
        if (k < 0) {cout << -1 << endl;return;}
        if (k%7!=0&&k/7>=r-1){
            k = un-4;
            z = 4;
        }
        if (k < 0) {cout << -1 << endl;return;}
        if (k%7!=0&&k/7>=r-1){
            k = un-5;
            z = 2;
        }
        if (k < 0) {cout << -1 << endl;return;}
        if (k%7!=0&&k/7>=r-1){
            k = un-6;
            z = (a==1?6:0);
        }
        if (k < 0) {cout << -1 << endl;return;}
        if (k%7!=0&&k/7>=r-1){
            k = un-7;
            z = 8;
        }
        if (k < 0) {cout << -1 << endl;return;}
        cout << z;
        told(k, r-1, 0);
    }
}

int main(void) {
    int T;
    cin >> T;
    for (int i = 0; i < T; i++){
        int n;
        cin >> n;
        int r  = (n-1) / 7 + 1;
        told(n, r, 1);
    }
}
