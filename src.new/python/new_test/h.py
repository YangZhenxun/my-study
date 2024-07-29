from urllib.parse import urlencode, urlunparse
import requests
from bs4 import BeautifulSoup
import urllib.request
import re

query = "红色诗词"

page = requests.get('https://www.bing.com/search?q=%s'%query)

soup = BeautifulSoup(page.text, "html.parser")
links = soup.find_all(name="a")
rl = re.compile(r"^j*/*#*")
rk = re.compile(r"^j*#*")
ek = re.compile(r"/search?q=.*&FROM=([a-zA-Z]+[0-9]+)")
lk = None

def lka(link):
    if rl.match(link.get("href")).group() == '':
        print(link.get("href"))
        lk = link.get("href")
        return lk
    if rk.match(link.get("href")).group() == '' and (ek.search(link.get("href")) != "" and ek.search(link.get("href")) != None):
        print(link.get("href"))
        print(ek.search(link.get("href")))
        raise NotImplementedError()

def lkb():
    global lk
    for link in links:
        if link.get("href"):
            if lk:
                if not link.get("href") == lk:
                    lk = lka(link)
            else:
                lk = lka(link)

lkb()
