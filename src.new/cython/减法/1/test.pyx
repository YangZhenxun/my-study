# cython: language_level=3
import random

while True:
    n1 = random.randint(5,10)
    n2 = random.randint(1,5)
    math = n1 - n2
    inp = int(input(str(n1) + "-" + str(n2) + "="))
    if inp != math:
        print("答案错误\n正确答案是："+str(math))
    else:
        print("正确！")
