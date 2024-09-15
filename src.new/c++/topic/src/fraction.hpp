#include <cmath>
#include <cstdlib>
#include <numeric>
#include <string>

using namespace std;

class fraction{
public:
    int Num, Den;
    fraction(int num, int den = 1, bool simplification = true){
        if (den == 0){
            throw 0;
        }
        Num = num;
        Den = den;
        if (simplification){
            simplify();
        }
    }
    fraction(double num, double den = 1.0, bool simplification = true){
        double num1 = num;
        int y = 1;
        for(int _i = 0; _i < y; _i++){
            num1 *= 10;
            if (num-round(num1) != 0){
                y++;
            }
        }
        double den1 = den*pow(10, y);
        if (den1-round(den1) != 0){
            for(int _i=0;_i<y;_i++){
                den1 *= 10;
                num *= 10;
                if(den1-round(den1)!=0){
                    y++;
                }
            }
        }
        fraction(static_cast<int>(num1), static_cast<int>(den1), simplification);
    }
    void simplify(void){
        Num = Num/gcd(Num, Den);
        Den = Den/gcd(Num, Den);
        if ((Den < 0) && (Num > 0)){
            Num = -Num;
            Den = -Den;
        }
        if (Den == 0){
            throw 0;
        }
    }
    fraction operator+(fraction rhs) {
        Num = Num*(lcm(Den, rhs.Den)/Den)+rhs.Num*(lcm(Den, rhs.Den)/rhs.Den);
        Den = lcm(Den, rhs.Den);
        return *this;
    }
    fraction operator+(int rhs) {
        return *this + fraction(rhs);
    }
    fraction operator+(double rhs){
        return *this + fraction(rhs);
    }
    fraction operator+(float rhs){
        return *this + fraction(static_cast<double>(rhs));
    }
    fraction operator+(void) {
        simplify();
        return *this;
    }
    fraction operator-(int rhs) {
        return *this - fraction(rhs);
    }
    fraction operator-(fraction rhs) {
        *this = *this + (fraction(-rhs.Num, rhs.Den));
        return *this;
    }
    fraction operator-(double rhs){
        return *this - fraction(rhs);
    }
    fraction operator-(float rhs){
        return *this - fraction(static_cast<double>(rhs));
    }
    fraction operator-(void) {
        Num = -Num;
        simplify();
        return *this;
    }
    fraction operator*(fraction rhs) {
        Num = Num*rhs.Num;
        Den = Den*rhs.Den;
        return *this;
    }
    fraction operator*(int rhs) {
        return *this * fraction(rhs);
    }
    fraction operator/(fraction rhs) {
        *this = *this * fraction(rhs.Den, rhs.Num);
        return *this;
    }
    fraction operator/(int rhs) {
        return *this / fraction(rhs);
    }
    fraction operator++(void) {
        return (*this + fraction(1));
    }
    fraction operator--(void) {
        return (*this - fraction(1));
    }
    bool operator>(fraction rhs){
        fraction l = fraction(Num*(lcm(Den, rhs.Den)/Den), lcm(Den, rhs.Den));
        fraction r = fraction(rhs.Num*(lcm(Den, rhs.Den)/rhs.Den), lcm(Den, rhs.Den));
        return (l.Num > r.Num);
    }
    bool operator>(int rhs){
        return (*this > fraction(rhs));
    }
    bool operator>(double rhs){
        return (*this > fraction(rhs));
    }
    bool operator>(float rhs){
        return (*this > fraction(static_cast<double>(rhs)));
    }
    bool operator>=(fraction rhs){
        return ((*this > rhs)||(*this == rhs));
    }
    bool operator>=(int rhs){
        return (*this >= fraction(rhs));
    }
    bool operator>=(double rhs){
        return (*this >= fraction(rhs));
    }
    bool operator>=(float rhs){
        return (*this >= fraction(static_cast<double>(rhs)));
    }
    bool operator<(fraction rhs){
        fraction l = fraction(Num*(lcm(Den, rhs.Den)/Den), lcm(Den, rhs.Den));
        fraction r = fraction(rhs.Num*(lcm(Den, rhs.Den)/rhs.Den), lcm(Den, rhs.Den));
        return (l.Num < r.Num);
    }
    bool operator<(int rhs){
        return (*this < fraction(rhs));
    }
    bool operator<(double rhs){
        return (*this < fraction(rhs));
    }
    bool operator<(float rhs){
        return (*this < fraction(static_cast<double>(rhs)));
    }
    bool operator<=(fraction rhs){
        return ((*this > rhs)||(*this == rhs));
    }
    bool operator<=(int rhs){
        return (*this <= fraction(rhs));
    }
    bool operator<=(double rhs){
        return (*this <= fraction(rhs));
    }
    bool operator<=(float rhs){
        return (*this <= fraction(static_cast<double>(rhs)));
    }
    bool operator==(fraction rhs){
        simplify();
        rhs.simplify();
        return ((Num == rhs.Num) && (Den == rhs.Den));
    }
    bool operator==(int rhs){
        return (*this == fraction(rhs));
    }
    bool operator==(double rhs){
        return (*this == fraction(rhs));
    }
    bool operator==(float rhs){
        return (*this == fraction(static_cast<double>(rhs)));
    }
    inline fraction abs(void){
        return *this >= 0 ? *this : -(*this);
    }
    operator double()const{
        return (1.0*Num/Den);
    }
    operator float()const{
        return (1.0*Num/Den);
    }
    operator int()const{
        return (Num/Den);
    }
    operator long() const{
        return (Num/Den);
    }
    operator long long()const{
        return (Num/Den);
    }
    operator string() const{
        return string(to_string(Num)+"/"+to_string(Den));
    }
    friend fraction operator+(int lhs, fraction rhs){
        return (rhs + lhs);
    }
    friend fraction operator-(int lhs, fraction rhs){
        return (lhs + -rhs);
    }
    friend fraction operator*(int lhs, fraction rhs){
        return (rhs * lhs);
    }
    friend fraction operator/(int lhs, fraction rhs){
        return (lhs * fraction());
    }
};

inline fraction abs(fraction f) noexcept{
    return f.abs();
}
inline string to_string(fraction f){
    return string(f);
}
