# cython: language_level=3
#distutils: language = c++
import random

cdef void _run():
    while True:
        n1 = random.randint(1,10)
        n2 = random.randint(1,10)
        math = n1 + n2
        try:
            inp = int(input(str(n1) + "+" + str(n2) + "="))
        except ValueError:
            print("不可直接回车！")
        else:
            if (inp != math):
                print("答案错误\n正确答案是：" + str(math))
            else:
                print("正确！")
