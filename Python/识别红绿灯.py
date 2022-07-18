import time
from alive_progress import alive_bar

def Tosee(redtime,greentime,yellowtime):
    ifZW = input("现在要转弯吗?(Y/n)")
    if ifZW == "Y" or "y":
        whereZW = input("现在往哪里转弯?(左/右)")
        if whereZW == "左":
            hasD = input("有红绿灯吗?(Y/n)")
            if hasD == "Y" or "y":
                whatD = input("现在是什么灯?(红/黄/绿)")
                if whatD == "红":
                    print("等待红灯中......")
                    with alive_bar(len(range(redtime))) as bar:
                        for i in range(redtime):
                            time.sleep(1)
                            bar()
                    print("等待黄灯中......")
                    with alive_bar(len(range(yellowtime))) as bar:
                        for i in range(yellowtime):
                            time.sleep(1)
                            bar()
                elif whatD == "黄":
                    print("等待黄灯中......")
                    with alive_bar(len(range(yellowtime))) as bar:
                        for i in range(yellowtime):
                            time.sleep(1)
                            bar()
                elif whatD == "绿":
                    print("None......")
