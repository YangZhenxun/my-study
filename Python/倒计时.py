import time

def timedown(time_s):
    while time_s:
        m , s = divmod(time_s,60)
        timeformat = "{:02d}:{:02d}".format(m, s)
        print(timeformat, end='\r')
        time.sleep(1)
        time_s -= 1

    print("时间到！")

if __name__ == '__main__':
    a = int(input("请输入时间："))
    timedown(time_s=a)
