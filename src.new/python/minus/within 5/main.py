import random
import time
import sys

def __init__():
    start = time.perf_counter()
    a = 0
    err = 0
    source = 0
    no = 0
    while True:
        rand_a = random.randint(1, 5)
        rand_b = random.randint(1, 5)
        if rand_a >= rand_b:
            try:
                inp = int(input(f"{rand_a}-{rand_b}="))
            except:
                print(f"\r请输入数字！")
                no += 1
                a += 1
                now = time.perf_counter()
                print(f"\r共答{a}题，错误{err}题，未答{no}题，正确{a - err - no}题，分数：{source}，共用时{int(now - start)}秒")
                continue
            maths = rand_a - rand_b
            if inp == maths:
                print(f"\r你答对了！")
                source += 1
            else:
                print(f"\r你答错了！\n正确答案是{maths}!")
                err += 1
                source -= 1
            a += 1
            now = time.perf_counter()
            print(f"\r共答{a}题，错误{err}题，未答{no}题，正确{a - err}题，分数：{source}，共用时{int(now - start)}秒")
        else:
            continue

if __name__ == "__main__":
    __init__()
