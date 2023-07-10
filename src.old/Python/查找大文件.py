from operator import length_hint
import os
from alive_progress import alive_bar

def get_big_file(path,filesize):
    with alive_bar(length_hint(os.walk(path))) as bar:
        for dirpath, dirnames, filenames in os.walk(path):
            for filename in filenames:
                target_file = os.path.join(dirpath, filename)
                bar()
                if not os.path.isfile(target_file):
                    continue
                size = os.path.getsize(target_file)
                size = size//1024
                if size > filesize:
                    size = '{size}KB'.format(size=size)
                    print(target_file, size)

if __name__ == '__main__':
    a = input("请输入要查找的地址：")
    b = int(eval(input("请输入要查找的大小(KB)：")))
    get_big_file(a,b)
