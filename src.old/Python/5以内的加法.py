import random

def __init__():
    while True:
        n1 = random.randint(1,10)
        n2 = random.randint(1,10)
        maths = n1 + n2
        try:
            a = int(input(str(n1) + "+" + str(n2) + "="))
            if a == maths:
                print("你答对了!\n\r",end = "")
            else:
                print(f"你答错了！\n正确答案是{maths}\n\r",end = "")
        except:
            pass

if __name__ == '__main__':
    __init__()
