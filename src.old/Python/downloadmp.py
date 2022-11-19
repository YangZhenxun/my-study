import requests
import fake_useragent

som = requests.session()
ua = fake_useragent.UserAgent()
some = som.get("https://webfs.ali.kugou.com/202210281226/df07a6093535dbd670791d4e4e5c838b/part/0/1664832/KGTX/CLTX001/clip_d6df46334ccd3777d3143fbb1f304e2e.mp3", headers={'User-Agent': ua.random})
with open("./The Cab-Lock me up.mp3", 'wb')as f:
    f.write(some.content)
