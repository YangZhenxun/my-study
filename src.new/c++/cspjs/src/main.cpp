#include <iostream>
#include <cmath>
#include <algorithm>
#include <vector>
using namespace std;

int main(void) {
    int n;
    cin >> n;
    int *l = new int[n];
    vector<int> v;
    for (int i = 0; i < n; i++){
        cin >> l[i];
    }
    sort(l, l+n);
    for (int i = 1; i <= n; i++){
        if ((l[i-1] > l[i-2])&&(i!=1)){
            v.insert(v.begin(), n-i+1);
        }
    }
    v.insert(v.begin(), n-v[v.size()-1]);
    if (v.size() <= 1) cout << n << endl;
    else {
        cout << n - *min_element(v.begin(), v.end()) << endl;
    }
    delete [] l;
}
