#include <tuple>
#include <utility>
using namespace std;

template <typename T, typename P>
inline pair<T, P> copy_pair(pair<T, P> src){
    pair<T, P> dst;
    dst.first = get<0>(src);
    dst.second = get<1>(src);
    return dst;
}

#define paircpy(src, dst) \
    dst = copy_pair(src)

//Not used in this project, but it's useful.
//没有在本项目中用到，但是以下内容十分有用(tuple copy)

template <typename ...T>
inline tuple<T...> copy_tuple(tuple<T...> src){
    tuple<T...> dst;
    for (int i = 0; i < src.size(); i++){
        get<i>(dst) = get<i>(src);
    }
    return dst;
}

#define tuplecpy(src, dst) \
    dst = copy_tuple(src)