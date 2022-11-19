import random
import os
import time
import pickle

class Admin(object):
    admin = 'YangZhenxun'
    passwd = 'Yzx20120413'

    # 管理员界面
    def AdminView(self):
        print("**********************************************")
        print("*                                            *")
        print("*                                            *")
        print("*               欢迎登录联盟银行              *")
        print("*                                            *")
        print("*                                            *")
        print("**********************************************")

    # 功能界面
    def FunctionView(self):
        print("**********************************************")
        print("*           开户（1）     查询（2）          *")
        print("*           取款（3）     存款（4）          *")
        print("*           转账（5）     改密码（6)         *")
        print("*           锁定（7）     解锁（8）          *")
        print("*           补卡（9）     销户（10）         *")
        print("*                   退出（0）                *")
        print("**********************************************")

    # 管理员验证
    def Check(self):
        inputAdmin = input("请输入管理员账户: ")
        if self.admin != inputAdmin:
            print("账号输入错误！")
            return -1
        inputPasswd = input("请输入管理员密码: ")
        if self.passwd != inputPasswd:
            print("密码输入错误！")
            return -1
        # 能执行到这里说明账号密码输入正确
        print("操作成功,请稍后...")
        # time.sleep(2)
        return 0

class Person(object):
    def __init__(self,name,idCard,phone,card):
        self.name = name  #用户名字
        self.idCard = idCard  #用户身份证
        self.phone = phone   #用户电话
        self.card = card  #用户的信用卡

class Card(object):
    def __init__(self,cardid,cardpasswd,cardmoney):
        self.cardid = cardid  #信用卡号
        self.cardpasswd = cardpasswd  #信用卡密码
        self.cardmoney = cardmoney  #卡内余额
        self.cardlock = False  #是否被锁定

class ATM(object):
    def __init__(self, allusers):
        # 存储所有用户的信息，用字典
        self.allUsers = allusers

    # 创建用户
    def CreatUser(self):
        # 目标：向用户字典中添加一对键值对（卡号-用户）
        name = input("请输入您的姓名：")
        idCard = input("请输入您的身份证号：")
        phone = input("请输入您的电话号码：")
        preMoney = int(input("请输入您的预存款金额："))
        if preMoney < 0:
            print("预存款输入有误，开户失败......")
            return -1
        # 设置密码
        onePasswd = input("请输入密码：")
        # 验证密码
        if not self.checkPasswd(onePasswd):
            print("密码输入有误，开户失败......")
            return -1
        # 信息收集完成
        # 生成卡号
        cardId = self.creatCardId()

        card = Card(cardId, onePasswd, preMoney)
        user = Person(name, idCard, phone, card)
        # user存到字典
        self.allUsers[cardId] = user
        print("开户成功，请牢记卡号(%s)......" % cardId)

    # 查询余额
    def searchUserInfo(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
        # 判断是否锁定
        if user.card.cardlock == True:
            print("该卡已经被锁定，请解锁后再使用其他操作......")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误，该卡已经被锁定......")
            user.card.cardlock = True
            return -1
        # 查询成功，输出结果
        print("账户：%s     余额：%d" % (user.card.cardid, int(user.card.cardmoney)))

    # 取款
    def getMoney(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
        # 判断是否锁定
        if user.card.cardlock == True:
            print("该卡已经被锁定，请解锁后再使用其他操作......")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误，该卡已经被锁定......")
            user.card.cardlock = True
            return -1
        getmoney = int(input("请输入您取款金额："))
        nowmoney = int(user.card.cardmoney)
        if getmoney > nowmoney:
            print("余额不足，取款失败......")
            return -1
        nowmoney -= getmoney
        user.card.cardmoney = nowmoney
        print("取款成功，您目前余额为：%d" % user.card.cardmoney)

    # 存款
    def saveMoney(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
        # 判断是否锁定
        if user.card.cardlock == True:
            print("该卡已经被锁定，请解锁后再使用其他操作......")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误，该卡已经被锁定......")
            user.card.cardlock = True
            return -1
        savemoney = int(input("请输入您存款金额："))
        nowmoney = int(user.card.cardmoney)
        nowmoney += savemoney
        user.card.cardmoney = nowmoney
        print("取款成功，您目前余额为：%d" % user.card.cardmoney)

    # 转账
    def transferMoney(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
        # 判断是否锁定
        if user.card.cardlock == True:
            print("该卡已经被锁定，请解锁后再使用其他操作......")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误，该卡已经被锁定......")
            user.card.cardlock = True
            return -1
        # 查询成功，输出结果
        print("账户：%s     余额：%d" % (user.card.cardid, int(user.card.cardmoney)))
        tocardid = input("请输入您希望转账的账户：")
        # 验证是否存在该卡号
        usertoid = self.allUsers.get(tocardid)
        if not usertoid:
            print("该卡号不存在，查询失败......")
            return -1
        tomoney = int(input("请输入您希望转账的金额："))
        nowmoney = int(user.card.cardmoney)
        tonowmoney = int(usertoid.card.cardmoney)
        if tomoney > nowmoney:
            print("余额不足，转账失败......")
            return -1
        nowmoney -= tomoney
        tonowmoney += tomoney
        user.card.cardmoney = nowmoney
        usertoid.card.cardmoney = tonowmoney
        print("转账成功，您目前余额为：%d" % user.card.cardmoney)

    # 改密码
    def changePasswd(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
        # 判断是否锁定
        if user.card.cardlock == True:
            print("该卡已经被锁定，请解锁后再使用其他操作......")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误，该卡已经被锁定......")
            user.card.cardlock = True
            return -1
        newpasswd = input("请输入新密码：")
        newpasswd2 = input("请确认新密码:")
        if newpasswd != newpasswd2:
            print("两次密码输入不一致，密码修改失败")
            return -1
        user.card.cardpasswd = newpasswd
        print("密码修改成功......")

    # 锁定
    def lockUser(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
        if user.card.cardlock:
            print("该卡已经被锁定，请结束后再使用其他功能")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误......")
            return -1
        # 锁定
        user.card.cardlock = True
        print("锁定成功......")

    # 解锁
    def unlockUser(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，解锁失败......")
            return -1
        if not user.card.cardlock:
            print("该卡没有锁定，无需解锁......")
            return -1
        # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误,解锁失败......")
            return -1
        temid = input("请输入您的身份证号：")
        if temid != user.idCard:
            print("身份证号输入错误，解锁失败......")
            return -1
        # 解锁
        user.card.cardlock = False
        print("解锁成功......")

    # 补卡
    def newCard(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
            # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误......")
            return -1
        # 重新生成卡号
        newcard = self.creatCardId()
        user.card.cardid = newcard
        print("补卡办理成功，这是您的新卡号：%s" % user.card.cardid)

    # 销户
    def killUser(self):
        cardnum = input("请输入您的卡号：")
        # 验证是否存在该卡号
        user = self.allUsers.get(cardnum)
        if not user:
            print("该卡号不存在，查询失败......")
            return -1
            # 验证密码
        if not self.checkPasswd(user.card.cardpasswd):
            print("密码错误，销户失败......")
            return -1
        self.allUsers.pop(cardnum)
        print("该账户已经被销户......")

    # 验证密码,循环三次没有正确就输出错误
    def checkPasswd(self, realPasswd):
        for i in range(3):
            temPasswd = input("请确认密码：")
            if temPasswd == realPasswd:
                return True
        return False

    # 随机生成卡号
    def creatCardId(self):
        # 验证密码是否重复
        while True:
            str = ""
            for i in range(6):
                ch = chr(random.randrange(ord('0'), ord('9') + 1))
                str += ch
            # 判断是否重复
            if not self.allUsers.get(str):
                return str

def main():
    # 界面对象
    admin = Admin()

    # 管理员开机
    admin.AdminView()
    if admin.Check():
        return -1

    #存储信息的文件是否存在
    if os.path.exists("userinfo.txt"):
        filepath = "userinfo.txt"
    else:
        open("userinfo.txt", "wb")
        filepath = "userinfo.txt"
    # 提款机对象

    #如果存储信息的文件非空
    if os.path.getsize(filepath):
        f = open(filepath, "rb")
        # print(f)
        allusers = pickle.load(f)
    else:
        # print("*****")
        allusers = {}
    print((allusers))
    atm = ATM(allusers)

    while True:
        admin.FunctionView()
        # 等待用户操作
        option = input("请输入您的操作：")
        if option == '1':
            # 开户
            atm.CreatUser()
        elif option == '2':
            # 查询
            atm.searchUserInfo()
        elif option == '3':
            # 取款
            atm.getMoney()
        elif option == '4':
            # 存款
            atm.saveMoney()
        elif option == '5':
            # 转账
            atm.transferMoney()
        elif option == '6':
            # 改密码
            atm.changePasswd()
        elif option == '7':
            # 锁定
            atm.lockUser()
        elif option == '8':
            # 解锁
            atm.unlockUser()
        elif option == '9':
            # 补卡
            atm.newCard()
        elif option == '10':
            # 销户
            atm.killUser()
        elif option == '0':
            # 退出
            if not admin.Check():
                # 将当前系统中的用户信息保存到文件中
                f = open(filepath, "wb")
                pickle.dump(atm.allUsers, f)
                f.close()
                return -1

if __name__ == "__main__":
    main()
