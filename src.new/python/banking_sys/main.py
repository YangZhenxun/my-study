import sys
import os

import json
import random
import pathlib
import qrcode
import win32ui
import win32print
import pyzbar.pyzbar as pyzbar
import cv2
from pathlib import Path
from PIL import Image,ImageWin

class Admin:
    def __init__(self, admin:str, passwd:int):
        self.admin = admin
        self.passwd = passwd

    def admin_view(self):
        print("**********************************************")
        print("*                                            *")
        print("*                                            *")
        print("*               欢迎登录动物银行             *")
        print("*                                            *")
        print("*                                            *")
        print("**********************************************")

    def func_view(self):
        print("**********************************************")
        print("*           开户（1）     查询（2）          *")
        print("*           取款（3）     存款（4）          *")
        print("*           转账（5）     改密码（6)         *")
        print("*           锁定（7）     解锁（8）          *")
        print("*           补卡（9）     销户（10）         *")
        print("*                   退出（0）                *")
        print("**********************************************")

    def check(self) -> bool:
        inputadmin:str = input("请输入管理员账号：")
        if (inputadmin != self.admin):
            print("账号输入有误！")
            return True
        inputpasswd:int = int(input("请输入管理员密码："))
        if (inputpasswd != self.passwd):
            print("密码输入有误！")
            return True
        print("操作成功！请稍后...")
        return False

    def new():
        return Admin("1", 1)


class Card:
    def __init__(self, \
        cardid: str, \
        cardpasswd: str, \
        cardmoney: int, \
        cardlock:bool):
        self.cardid = cardid
        self.cardpasswd = cardpasswd
        self.cardmoney = cardmoney
        self.cardlock = cardlock


class Person:
    def __init__(self, \
        name: str, \
        idCard: str, \
        phone: str, \
        card: Card):
        self.name = name
        self.idCard = idCard
        self.phone = phone
        self.card = card


class ATM:
    def __init__(self, allusers: dict):
        self.allusers = allusers

    def creat_user(self) -> bool:
        """
        The function `create_user` prompts the user to input personal information, checks the password, creates a new card
        and person object, stores them in a dictionary, and prints out the card details.
        :return: The `creat_user` method is returning a boolean value `True` if the user account creation is successful, and
        `False` if it fails.
        """
        name:str = input("请输入你的姓名：")
        idcard:str = input("请输入你的身份证号：")
        phone:str = input("请输入你的电话号码：")
        ownpasswd:str = input("请输入密码：")
        if not self.check_passwd(ownpasswd):
            print("密码输入错误，开户失败！")
        cardid: int = self.creat_cardid()
        cardn: Card = Card(cardid, ownpasswd, 0, False)
        usern:Person = Person(name, idcard, phone, cardn)
        self.allusers[cardid] = usern
        print(f"开户成功！请收下银行卡！您的银行卡号为：{cardid}")
        self.print_card(cardid)
        return True

    def check_passwd(self, passwd:str, use: bool=False, cardn:Card=None) -> bool|tuple[bool, Card]:
        """
        This function checks if a password matches a given input and returns a boolean value or a tuple containing a boolean
        value and a Card object.

        :param passwd: The `passwd` parameter in the `check_passwd` method is a string that represents the password that
        needs to be confirmed. It is the password that the user is trying to verify
        :type passwd: str
        :param use: The `use` parameter in the `check_passwd` method is a boolean flag that indicates whether the password
        confirmation is being used in the context of a specific card operation. If `use` is `True`, it means that the
        password confirmation is associated with a card operation, and if `use`, defaults to False
        :type use: bool (optional)
        :param cardn: The `cardn` parameter in the `check_passwd` method seems to be of type `Card`, which is likely a class
        representing a card object. This parameter is used to pass in a card object for additional operations within the
        method
        :type cardn: Card
        :return: The function `check_passwd` returns a boolean value or a tuple containing a boolean value and a Card
        object.
        """
        for _i in range(3):
            tempasswd:str = input("请确认密码：")
            if tempasswd == passwd:
                return True if not use else (True, cardn)
        if not use:
            print("确认失败！")
            return False
        else:
            print("确认失败！该卡已经锁定！")
            if cardn:
                cardn.cardlock =True
            return (False, cardn)

    def creat_cardid(self) -> str:
        while True:
            strn:str = ""
            for _i in range(6):
                ia1:str = str(random.randint(1, 9))
                strn += ia1
            if not self.allusers.get(strn):
                return strn

    def print_card(self, cardid:str):
        """
        The function generates a QR code image from a given card ID, prints it using the default printer, and then deletes
        the temporary image file.

        :param cardid: The `print_card` function takes a `cardid` parameter, which is expected to be a string representing
        the ID of the card for which a QR code needs to be generated and printed. The function generates a QR code image
        based on this `cardid`, saves it as a PNG file,
        :type cardid: str
        """
        img = qrcode.make(cardid)
        img.save((Path(__file__).parent/"qrc.png").resolve()._str)
        printer_name = win32print.GetDefaultPrinter()
        im = Image.open((Path(__file__).parent/"qrc.png").resolve()._str)
        hDC = win32ui.CreateDC()
        hDC.CreatePrinterDC(printer_name)
        hDC.StartDoc(((Path(__file__).parent/"qrc.png").resolve()._str))
        hDC.StartPage()
        dib = ImageWin.Dib(im)
        dib.draw(hDC.GetHandleOutput(), (0, 0, im.width, im.height))
        hDC.EndPage()
        hDC.EndDoc()
        del hDC
        (Path(__file__).parent/"qrc.png").resolve().unlink()

    def search_userinfo(self, cardid:str, qrcode: bool) -> bool:
        """
        This function searches for user information based on a card ID and QR code status, checking for card existence, lock
        status, and password validation.

        :param cardid: The `cardid` parameter is a string that represents the unique identifier of a user's card. It is used
        to look up the user's information in the system
        :type cardid: str
        :param qrcode: The `qrcode` parameter in the `search_userinfo` method is a boolean flag that indicates whether the
        user wants to use a QR code for the search operation. If `qrcode` is `True`, it means the user wants to use a QR
        code for the search, and if it
        :type qrcode: bool
        :return: The function `search_userinfo` returns a boolean value.
        """
        personn:Person = self.allusers.get(cardid)
        if not personn:
            print("该卡号不存在！查询失败！")
            return True
        if personn.card.cardlock:
            print("该卡已经锁定！")
            return True
        if not qrcode:
            if not self.check_passwd(personn.card.cardpasswd, True, personn.card)[0]:
                return True
        print(f"账户：{personn.card.cardid}      名字：{personn.name}         余额：{personn.card.cardmoney}")

    def detect(self) ->str:
        """
        The function opens the camera, read the information about QR Code.

        :return: The function `detect` returns a string value.
        """
        cap = cv2.VideoCapture(0)
        while(True):
            ret, frame = cap.read()
            flag = cv2.waitKey(1)
            gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
            barcodes = pyzbar.decode(gray)
            for barcode in barcodes:
                (x, y, w, h) = barcode.rect
                cv2.rectangle(frame, (x, y), (x + w, y + h), (0, 255, 0), 2)
                barcodeData = barcode.data.decode("utf-8")
                barcodeType = barcode.type
                text = "{}".format(barcodeData)
                cv2.putText(frame, text, (x, y - 10), cv2.FONT_HERSHEY_SIMPLEX,1, (0, 255, 0), 2)
                #print("[扫描结果]: {0}".format(barcodeData))
                flag = 27
            cv2.imshow("QRCODE",frame)
            if flag == 27:
                break
        cap.release()
        cv2.destroyAllWindows()
        return barcodeData

    def getmoney(self):
        """
        This function gets money from the user's bank account.
        It checks the card's information and get the user's money.
        It based on print_card function(so it's also unstable) to print moneys.
        """
        pass

    def newcard(self):
        """
        The function is useful when the user's card is lost.
        It asks the user to input the account number and password.
        It based on print_card function to print the card.

        :return: The `newcard` function returns None.
        """
        usr:str = input("请输入您的银行卡号：")
        personn:Person = self.allusers.get(usr)
        if not personn:
            print("该卡号不存在，补卡失败！")
            return True
        if personn.card.cardlock:
            print("该卡已经锁定！")
            return True
        self.print_card(usr)
        print("补卡成功！")
        return False

    def new(allusers:dict[str, Person]):
        """
        Return a class ATM object just like the function "new" in Java and Rust.

        :param allusers:The users' information.It's a `dictionary` that the keys are
        the users' account number and the values are the users' `Person` object.
        :return: It returns a new `ATM` object.
        """
        return ATM(allusers)

def obj2(obj):
    """
    Convert a object to a dictionary that can be used.
    :param obj: The `object` to convert.
    :return: The `dictionary` that is converted.
    """
    return obj.__dict__

def main_no_ui() -> (ATM, Admin, pathlib.Path):
    adm = Admin.new()
    allusers = {}
    filepath = pathlib.Path(__file__).parent/"userinfo.json"
    if not filepath.exists():
        filepath.touch()
    size = filepath.stat().st_size
    if size != 0:
        allusers = json.loads(filepath.read_text())
        for key in allusers.keys():
            allusers[key]["card"] = Card(**allusers[key]["card"])
            allusers[key] = Person(**allusers[key])
            print(allusers)
    atm = ATM.new(allusers)
    adm.admin_view()
    if adm.check():
        sys.exit(-1)
    return atm, adm, filepath

def main():
    atm, adm, filepath = main_no_ui()
    while True:
        adm.func_view()
        opt = input("请输入您的操作：")
        match opt:
            case "1":
                if atm.creat_user():
                    continue
            case "2":
                ask = input("您想要扫二维码查询吗？(Y/n)")
                if ask.lower() == "y" or ask == "":
                    cardid = atm.detect()
                    if atm.search_userinfo(cardid, True):
                        continue
                else:
                    cardid = input("请输入您的银行卡号：")
                    if atm.search_userinfo(cardid, False):
                        continue
            case "9":
                if atm.newcard():
                    continue
            case "0":
                if not adm.check():
                    if atm.allusers.values():
                        for key in atm.allusers.keys():
                            filepath.write_text(json.dumps(atm.allusers, default=obj2 ,indent=2, sort_keys=True), "utf-8")
                    sys.exit(0)
            case _:
                continue

if __name__ == "__main__":
    main()
