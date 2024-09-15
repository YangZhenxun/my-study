#include <cmath>
#include <cstring>
#include <iostream>
#include <map>
#include <utility>
#include <vector>
#include "cpy.hpp"
#include "lazy.hpp"
#include "fraction.hpp"

using namespace std;

double a11 = 1.5;
double b11 = 3.14;
int c11 = -5;
double d11 = -M_PI;

#define comp(x, y) \
    x >= y ? x : y

inline double one_one(){
    return comp(fabs(a11), (comp(fabs(b11), (comp(fabs(c11), fabs(d11))))));
}

int a16 = 14;
int b16 = 15;
int c16 = 20;
int d16 = 35;

vector<pair<int, int>> add_of_16(pair<int, int> x, pair<int, int> to){
    auto l = lazy_vec_init_func(add_of_16, x, to);
    vector<pair<int, int>> m;
    pair<int, int> x1;
    pair<int, int> x2;
    paircpy(x, x1);
    paircpy(x, x2);
    if (get<0>(x1) != get<0>(to)){
        get<0>(x1) += 1;
        l.push_back(lazy(add_of_16, x1, to));
    }
    if (get<1>(x2) != get<1>(to)){
        get<1>(x2) += 1;
        l.push_back(lazy(add_of_16, x2, to));
    }
    else if ((x1 == to) && (x2 == to)){
        m.push_back(to);
    }
    for (int i = 0; i < l.size(); i++){
        auto v = l[i].value();
        for (int j = 0; j < v.size(); j++){
            m.push_back(v[j]);
        }
        v.clear();
        v.shrink_to_fit();
    }
    l.clear();
    l.shrink_to_fit();
    return m;
}

inline int one_six(){
    pair<int, int> a = {1, 1};
    pair<int, int> b = {5, 4};
    return add_of_16(a,b).size();
}

inline int two_one(){
    int v = 1;
    for (double i = -2.1; i < 3.4; i++){
        v *= round(i);
    }
    return v == 1 ? 0 : v;
}

int main() {
    auto res11 = one_one();
    char res = res11 == fabs(a11) ? 'A' : (res11 == fabs(b11) ? 'B' : (res11 == fabs(c11) ? 'C' : 'D'));
    map<char, double> co = {{'A', (double)a11}, {'B', (double)b11}, {'C', (double)c11}, {'D', (double)d11}};
    cout << "2024.8.24 练习" << endl;
    cout << "第一大题 第一小题 答案是：" << co[res] << " 选 " << res << endl;
    auto res16 = one_six();
    res = res16 == a16 ? 'A' : (res16 == b16 ? 'B' : (res16 == c16 ? 'C' : 'D'));
    cout << "第一大题 第六小题 答案是：" << res16 << " 选 " << res << endl;
    cout << "第二大题 第一小题 答案是：" << two_one() << endl;
    return 0;
}