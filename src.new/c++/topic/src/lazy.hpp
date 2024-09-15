#include <functional>
#include <type_traits>
#include <utility>
#include <vector>
using namespace std;

template<typename T>
class Lazy{
public:
    Lazy(){}
    
    template<typename Func, typename ...Args>
    Lazy(Func& func, Args&&... args){
        m_func = bind(func, std::forward<Args>(args)...);
    }

    T& value(){
        if(!m_isCreated){
            m_result = m_func();
            m_isCreated = true;
        }
        return m_result;
    }
private:
    function<T()> m_func;
    T m_result;
    bool m_isCreated = false;
};

template<class Func, typename ...Args>
Lazy<invoke_result_t<Func, Args...>>
lazy(Func&& f, Args&& ...args)
{
    return Lazy<invoke_result_t<Func, Args...>>(
        std::forward<Func>(f), std::forward<Args>(args)...);
}

template<typename Func, typename ...Args>
vector<Lazy<invoke_result_t<Func, Args...>>>
lazy_vec_init_func(Func&& f, Args&& ...args){
    vector<Lazy<invoke_result_t<Func, Args...>>> some;
    return some;
}