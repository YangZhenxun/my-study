import itchat
import time
from itchat import content

itchat.auto_login(True)

#print(itchat.get_friends(True))
for i in itchat.get_friends(True):
    print(i["NickName"] + i["RemarkName" + i["DisplayName"]])
try:
    author = itchat.search_friends(nickName="emo侠")[0]
    author.send("Boot on.")
    for i in range(100):
        author.send('OKKKKKKKKKKKKKK \n'*20)
        #author.send_raw_msg(content.CARD, "test@Car")
        #time.sleep(0.1)
    author.send("Shutdown.")

except IndexError:
    print(itchat.search_friends(nickName="故障瑞"))
