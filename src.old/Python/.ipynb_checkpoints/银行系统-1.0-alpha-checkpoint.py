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
    