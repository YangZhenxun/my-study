#include <iostream>
using namespace std;
int t, n;
int ans[22] = { 0, -1, 1, 7, 4, 2, 6, 8, 10, 18, 22, 20, 28, 68, 88, 108, 188, 200, 208, 288, 688, 888};
int main() {
    cin >> t;
    while(t--) {
        cin >> n;
        if (n <= 21) {
            cout << ans[n] << "\n";
        } else {
            int cnt;
            if (n % 7 == 0) {
                cnt = n / 7;
            } else if (n % 7 == 1) {
                cout << 10;
                cnt = (n - 8) / 7;
            } else if (n % 7 == 2) {
                cout << 1;
                cnt = (n - 2) / 7;
            } else if (n % 7 == 3) {
                cout << 200;
                cnt = (n - 17) / 7;
            } else if (n % 7 == 4) {
                cout << 20;
                cnt = (n - 11) / 7;
            } else if (n % 7 == 5) {
                cout << 2;
                cnt = (n - 5) / 7;
            } else if (n % 7 == 6) {
                cout << 6;
                cnt = (n - 6) / 7;
            }
            for(int i=1;i<=cnt;i++){
                cout<<8;
            }
            cout<<"\n";
        }
    }
    return 0;
}
