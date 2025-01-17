import os
import sys
import random
import pickle

if os.path.exists(os.path.abspath(sys.path[0]+"\\userinfo.txt")):
    filepath = os.path.abspath(sys.path[0]+"\\userinfo.txt")
else:
    open(os.path.abspath(sys.path[0]+"\\userinfo.txt"), "wb")
    filepath = os.path.abspath(sys.path[0]+"\\userinfo.txt")

if os.path.exists(os.path.abspath(sys.path[0]+"\\classinfo.txt")):
    filepath2 = os.path.abspath(sys.path[0]+"\\classinfo.txt")
else:
    open(os.path.abspath(sys.path[0]+"\\classinfo.txt"), "wb")
    filepath2 = os.path.abspath(sys.path[0]+"\\classinfo.txt")

class User:
    def __init__(self, username: str, classes= None, star = 0):
        self.username = username
        self.classes = classes
        self.userlock = False
        self.star = star

class Class:
    def __init__(self, username, userlist):
        self.username = username
        self.userlist = userlist
        self.classname = self.classrandom()

    def classrandom(self) -> str:
        while True:
            _str = ""
            for i in range(6):
                rands = chr(random.randint(ord('0'), ord('9')))
                _str += rands
            if not self.userlist.get(_str):
                return _str

class Person:
    def __init__(self, user, classes):
        self.user = user
        self.classes = classes


class Admin:
    def AdminView(self):
        print("**********************************************")
        print("*                                            *")
        print("*                                            *")
        print("*              正在启动计分器...             *")
        print("*                                            *")
        print("*                                            *")
        print("**********************************************")

    def FunctionView(self):
        print("**********************************************")
        print("*           开户（1）     查询（2）          *")
        print("*           加分（3）     扣分（4）          *")
        print("*           兑奖（5）     清零（6）          *")
        print("*           锁定（7）     解锁（8）          *")
        print("*           加入（9）     销户（10）         *")
        print("*                   退出（0）                *")
        print("**********************************************")

class Scoring:
    def __init__(self, userlist, classlist):
        self.userlist = userlist
        self.classlist = classlist

    def CreatUser(self):
        name = input("请输入姓名：")
        if self.userlist.get(name):
            print("您已经注册过了！")
            return -1
        team = input("请输入要加入的团体（家庭，班级......若是没有，请填无）：")
        _class = Class(name, self.userlist)
        _user = User(name, _class)
        user = Person(_user, _class)
        self.userlist[name] = user
        self.classlist[user.classes.classname] = team

        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("开户成功！")

    def searchUserInfo(self):
        userinp = input("请输入您的姓名：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        print(f"账户：{user.user.username}     分数：{int(user.user.star)}    团体：{self.classlist[user.classes.classname]}")

    def AddStar(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，请解锁后再使用其他操作......")
            return -1
        addstar = int(input("请输入您加的分数："))
        nowstar = int(user.user.star)
        nowstar += addstar
        user.user.star = nowstar
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("加分成功，您目前分数为：%d" % user.user.star)

    def GetStar(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，请解锁后再使用其他操作......")
            return -1
        getstar = int(input("请输入您扣的分数："))
        nowstar = int(user.user.star)
        if getstar > nowstar:
            print("分数不足！")
            return -1
        nowstar -= getstar
        user.user.star = nowstar
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("扣分成功，您目前分数为：%d" % user.user.star)

    def StarTo(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，请解锁后再使用其他操作......")
            return -1
        try:
            getstar = int(input("请输入兑换哪种奖品？（20/100， 默认20）："))
        except ValueError:
            print("请输入20或100，两种奖品选一种！")
            return -1
        nowstar = int(user.user.star)
        if getstar == 100:
            print("你可以买好一点的东西了！")
        elif getstar == 20:
            print("你只可以买20元左右的东西。")
        else:
            print("请输入20或100，两种奖品选一种！")
            return -1
        if getstar > nowstar:
            print("分数不足！")
            return -1
        nowstar -= getstar
        user.user.star = nowstar
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("兑奖成功，您目前分数为：%d， 可以去找妈妈换奖励了！" % user.user.star)

    def Clear(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，请解锁后再使用其他操作......")
            return -1
        yes = input("你真的确定要清零你的分数吗？此操作不可逆！请在下面输入%s！\n" % (user.user.username))
        if yes != user.user.username:
            print("正在取消清零......")
            return -1
        user.user.star = 0
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("清零成功！")


    def Lock(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，无需再次锁定......")
            return -1
        user.user.userlock = True
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("锁定成功！")

    def Unlock(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if not user.user.userlock:
            print("该账号已经被解锁，无需再次解锁......")
            return -1
        user.user.userlock = False
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("解锁成功！")

    def Join(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，请解锁后再使用其他操作......")
            return -1
        team = input("请输入你要加入的团体：")
        self.classlist[user.classes.classname] = team
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("加入成功！")

    def KillUser(self):
        userinp = input("请输入您的账号：")
        user = self.userlist.get(userinp)
        if not user:
            print("该账号不存在，查询失败......")
            return -1
        if user.user.userlock == True:
            print("该账号已经被锁定，请解锁后再使用其他操作......")
            return -1
        self.userlist.pop(userinp)
        self.classlist.pop(user.classes.classname)
        with open(filepath, "wb") as f:
            pickle.dump(self.userlist, f)
        with open(filepath2, "wb") as f:
            pickle.dump(self.classlist, f)
        print("销户成功！")

def _init():
    admin = Admin()
    admin.AdminView()
    if os.path.getsize(filepath):
        f = open(filepath, "rb")
        userlist = pickle.load(f)
    else:
        userlist = {}
    if os.path.getsize(filepath2):
        f = open(filepath2, "rb")
        classlist = pickle.load(f)
    else:
        classlist = {}
    scoring = Scoring(userlist, classlist)

    while True:
        admin.FunctionView()
        option = input("请输入您的操作：")
        match option:
            case '1':
                scoring.CreatUser()
            case '2':
                scoring.searchUserInfo()
            case '3':
                scoring.AddStar()
            case '4':
                scoring.GetStar()
            case '5':
                scoring.StarTo()
            case '6':
                scoring.Clear()
            case '7':
                scoring.Lock()
            case '8':
                scoring.Unlock()
            case '9':
                scoring.Join()
            case '10':
                scoring.KillUser()
            case '0':
                with open(filepath, "wb") as f:
                    pickle.dump(scoring.userlist, f)
                with open(filepath2, "wb") as f:
                    pickle.dump(scoring.classlist, f)
                return 0

if __name__ == "__main__":
    _init()
