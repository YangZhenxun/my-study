# coding: utf-8
import requests
import os
import time
from fake_useragent import *
from threading import Thread
from alive_progress import alive_bar

ua = UserAgent()
url = 'https://nchc.dl.sourceforge.net/project/mingw-w64/Toolchains%20targetting%20Win64/Personal%20Builds/mingw-builds/8.1.0/threads-posix/seh/x86_64-8.1.0-release-posix-seh-rt_v6-rev0.7z' # 下载地址
filename = url[url.rindex('/') + 1:]
if os.path.exists("D:/Downloads/"+ filename):
    size = os.path.getsize("D:/Downloads/"+ filename)
else:
    size = 0
headers ={
    "User-Agent":ua.random,
    "Range" : "bytes=%d-" % size
}
print('filename = ' + filename)

path = 'D:\\Downloads\\'
if not os.path.exists(path):
    os.mkdir(path)

response = requests.get(url, stream=True, headers=headers)
content_size = int(response.headers['content-length'])

with open(path+filename, 'ab') as f:
    with alive_bar(content_size) as progress:
        for data in response.iter_content(chunk_size = 1):
            if data:
                f.write(data)
                size +=len(data)
                progress()
