# coding: utf-8
import requests
import os
from fake_useragent import *
from queue import Queue
from requests.auth import *
import sys
import rich
from rich.progress import Progress
from threading import Thread

ua = UserAgent()
url = "https://repo.anaconda.com/archive/Anaconda3-2022.05-Windows-x86_64.exe"
headers = {
    'user-agent': ua.random
}
filename = os.path.basename(url)
print('filename = ' + filename)
path = 'D:\\Downloads\\'
if not os.path.exists(path):
    os.mkdir(path)
session = requests.Session()

def write():
    size = 0
    response_1 = session.get(url, timeout=10, headers={'User-Agent': ua.random}, stream=True)
    content_size = int(session.head(url, timeout=10, headers={'User-Agent': ua.random}).headers.get('Content-Length'))
    print(content_size)
    with open(path+filename, 'wb') as f:
        with Progress() as progress:
            down = progress.add_task(description='all:%d, now:%d' % (content_size , size),total=content_size)
            for data in response_1.iter_content(1024):
                f.write(data)
                size += len(data)
                if not progress.finished:
                    progress.update(down, advance=size)
write()
