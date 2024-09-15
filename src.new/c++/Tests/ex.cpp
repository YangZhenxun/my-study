#include <cmath>
#include <iostream>
#include <map>
#include <tuple>
#include <functional>
#include <type_traits>
#include <vector>
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

template<typename T>
class lazy{
public:
    lazy(){};
    template<typename func, typename ...args>
    lazy(func& f, args&&... arg){
        m_function = [&f, &arg...]() {return f(std::forward<args>(arg)...);};
        m_function = bind(f, std::forward<args>(arg)...);
    }
    T& value()
    {
        if (!m_isCreate)
        {
            m_result = m_function();
            m_isCreate = true;
        }
        return m_result;
    }
private:
    function<T()> m_function;
    T m_result;
    bool m_isCreate = false;
};

template<class Func, typename ...Args>
lazy<invoke_result_t<Func, Args...>>
laz(Func&& f, Args&& ...args)
{
    return lazy<invoke_result_t<Func, Args...>>(
        std::forward<Func>(f), std::forward<Args>(args)...);
}

vector<tuple<int, int>> add_of_16(tuple<int, int> x, tuple<int, int> to){
    vector<tuple<int, int>> m;
    vector<
        lazy<invoke_result_t<
        vector<tuple<int, int>> (&)(
        tuple<int, int>, 
        tuple<int, int>), 
        tuple<int, int> &, 
        tuple<int, int> &>>> 
    l;
    auto x1 = x;
    auto x2 = x;
    while (true) {
        if (get<0>(x1) != get<0>(to)){
            get<0>(x1) += 1;
            l.push_back(laz(add_of_16, x1, to));
        }
        if (get<1>(x2) != get<1>(to)){
            get<1>(x2) += 1;
            l.push_back(laz(add_of_16, x2, to));
        }
        else{
            auto ls = l.begin();
            if (ls!=l.end()){
                auto la = (*ls).value();
                auto las = la.begin();
                if (las!=la.end()){
                    m.push_back(*las);
                    l.erase(ls);
                }
                m.push_back(*las);
            }
            auto la = (*l.end()).value();
            auto las = la.begin();
            if (las!=la.end()){
                m.push_back(*las);
                l.erase(ls);
            }
            m.push_back(*las);
            m.push_back(to);
            break;
        }
    }
    return m;
}

inline int one_six(){
    tuple<int, int> a = {1, 1};
    tuple<int, int> b = {5, 4};
    return add_of_16(a,b).size();
}

int main() {
    char res = one_one() == fabs(a11) ? 'A' : (one_one() == fabs(b11) ? 'B' : (one_one() == fabs(c11) ? 'C' : 'D'));
    map<char, double> co = {{'A', (double)a11}, {'B', (double)b11}, {'C', (double)c11}, {'D', (double)d11}};
    cout << "第一大题 第一小题 答案是：" << co[res] << "选 " << res << endl;
    cout << "第二大题 答案是：" << one_six() << endl;
    return 0;
}