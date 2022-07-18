'''
Python version:3.11.0b1
Python version:3.7.9
Package:
os
pickle
'''
import pickle,os

class Card(object):
    '''
    函数 -> __init__
    :详情见__init__函数
    '''
    def __init__(self,Id,Password,money):
        '''
        @card
        :cardId -> 卡号
        :cardPasswd -> 密码
        :cardmoney -> 存的钱
        :cardlock -> 是否锁定 -> 默认False
        '''
        self.cardId = Id
        self.cardPasswd = Password
        self.cardmoney = money
        self.cardlock = False

class User(object):
    '''
    函数 -> __init__
    :详情见__init__函数
    '''
    def __init__(self,name,search_name,card):
        '''
        @User
        :name -> 名字
        :search_name -> 索引
        :card -> @card
        '''
        self.name = name
        self.search_name = search_name
        self.card = card

class Admin(object):
    '''
    函数:openview,haveview,incase_admin,admin_xinxi
    :用于显示开机/使用视图，也用于检查管理员的身份。
    '''
    def admin_xinxi(self):
        '''
        定义管理员的信息
        '''
        one = "杨"
        two = "震"
        three = "勋"
        non = one + three
        note = non + two
        self.admin_name = (non - three) + (note - two) + three
        self.admin_passwd = "201"+"204"+"13"

    def openview(self):
        '''
        开始视图
        '''
        print("**********************************************")
        print("*                                            *")
        print("*                                            *")
        print("*               欢迎登录动物银行              *")
        print("*                                            *")
        print("*                                            *")
        print("**********************************************")

    def haveview(self):
        '''
        使用视图
        '''
        print("**********************************************")
        print("*           开户（1）     查询（2）          *")
        print("*           取款（3）     存款（4）          *")
        print("*           转账（5）     改密码（6)         *")
        print("*           锁定（7）     解锁（8）          *")
        print("*           补卡（9）     销户（10）         *")
        print("*                   退出（0）                *")
        print("**********************************************")

    def incase_admin(self):
        '''
        检查Admin的身份
        '''
        input_name_admin = input("请输入管理员名字:")
        if self.admin_name != input_name_admin:
            print("管理员信息有误!")
            return -1
        input_passwd_admin = input("请输入管理员密码:")
        if input_passwd_admin != self.admin_passwd:
            print("管理员信息有误!")
            return -1
        print("正在设置，请稍后......")

class ATM(object):
    def __init__(self,allusers):
        self.allusers = allusers

    def CreatUser(self):
        Username = input("请输入你的名字:")

        if Username == self.allusers.user.name:
            print("已有此名字，请换一个！")
            return -1
        Usersearch_name = input("请输入你希望的索引:")

        if Usersearch_name == self.allusers.user.search_name:
            print("已有此索引，请换一个!")
            return -1

        User_savemoney = input("请输入你要存的金额:")

        if User_savemoney <= 0:
            print("存款金额无效!")
            return -1

        UserPasswd = input("请输入你希望的密码:")

        if not self.casePassword(UserPasswd):
            print("密码验证失败......")
            return -1

        Cardid = self.Creatcardid()

        save_card = Card(Cardid, UserPasswd,User_savemoney)
        save_user = User(Username,Usersearch_name,save_card)

        self.allusers[Cardid] = save_user

        print(f"开户成功，你的卡号是{Cardid}")

        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)

    def searchUser(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == True:
            print("你的卡现在是锁定的，请解锁后再试......")
            return -1

        if not self.casePassword:
            print("密码验证失败.......")
            self.allusers.card.cardlock = True
            with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
                pickle.dump(self.allusers,f)
            return -1

        print(f"账户：{user}    余额：{user.card.cardmoney}")

    def get_money(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == True:
            print("你的卡现在是锁定的，请解锁后再试......")
            return -1

        if not self.casePassword:
            print("密码验证失败.......")
            self.allusers.card.cardlock = True
            with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
                pickle.dump(self.allusers,f)
            return -1

        get_moneies = int(input("请输入您要取的钱:"))
        if get_moneies > user.card.cardmoney:
            print("金额过大，请重试!")
            return -1

        user.card.cardmoney -= get_moneies
        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)
        print(f"取款成功，您的余额是:{user.card.cardmoney}")

    def save_money(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == True:
            print("你的卡现在是锁定的，请解锁后再试......")
            return -1

        if not self.casePassword:
            print("密码验证失败.......")
            self.allusers.card.cardlock = True
            with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
                pickle.dump(self.allusers,f)
            return -1

        save_moneies = int(input("请输入你要存款的金额:"))
        user.card.cardmoney += save_moneies
        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)
        print(f"存款成功，您的余额是:{user.card.cardmoney}")

    def give_money(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == True:
            print("你的卡现在是锁定的，请解锁后再试......")
            return -1

        if not self.casePassword:
            print("密码验证失败.......")
            self.allusers.card.cardlock = True
            with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
                pickle.dump(self.allusers,f)
            return -1

        inputothercardid = input("请输入对方卡号:")
        other_user = self.allusers.get(inputothercardid)

        if not other_user:
            print("对方卡号不存在！")
            return -1

        give_moneies = int(input("请输入转账金额:"))

        if give_moneies > user.card.cardmoney:
            print("金额过大，请重试！")
            return -1

        if other_user.card.cardlock == True:
            print("对方的卡是锁定的，请等对方解锁后再试......")
            return -1

        user.card.cardmoney -= give_moneies
        other_user.card.cardmoney += give_moneies
        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)

        print(f"转账成功！您的余额是：{user.card.cardmoney}")

    def chagePassword(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == True:
            print("你的卡现在是锁定的，请解锁后再试......")
            return -1

        if not self.casePassword:
            print("密码验证失败.......")
            self.allusers.card.cardlock = True
            with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
                pickle.dump(self.allusers,f)
            return -1

        newPassword = input("请输入新密码:")

        if not self.casePassword(newPassword):
            print("密码验证失败......")
            return -1

        user.card.cardpasswd = newPassword
        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)

        print("改密成功！")

    def cardlock(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == True:
            print("你的卡现在已经是锁定的，无需再锁定......")
            return -1

        if not self.casePassword:
            print("密码验证失败，你的卡直接被锁定了......")
            self.allusers.card.cardlock = True
            with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
                pickle.dump(self.allusers,f)
            return -1

        user.card.cardlock = True
        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)
        print("锁定成功！")

    def uncardlock(self):
        inputcardid = input("请输入你的卡号：")
        user = self.allusers.get(inputcardid)

        if not user:
            print("卡号有误!")
            return -1

        if user.card.cardlock == False:
            print("你的卡现在没有锁定，无需再解锁......")
            return -1

        if not self.casePassword:
            print("密码验证失败.......")
            return -1

        user.card.cardlock = False
        with open("D:\\my-study--github\\Python\\info.yzx","wb") as f:
            pickle.dump(self.allusers,f)
        print("解锁成功！")

    def newCard(self):
        input_searchcard = input("请输入您的索引:")
        user = list(self.allusers.keys())
        print(user)
ATM(allusers={}).newCard()
