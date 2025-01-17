#! /usr/bin/env python
# -*- coding: utf-8 -*- 

# Copyright (c) 2012-12 
# authored by TianWeishu, HuXiaoling, ChengXu.
# All rights reserved.
# This program or module is free software: you can redistribute it and/or
# modify it under the terms of the GNU General Public License as published
# by the Free Software Foundation, either version 2 of the License, or
# version 3 of the License, or (at your option) any later version. It is
# provided for educational purposes and is distributed in the hope that
# it will be useful, but WITHOUT ANY WARRANTY; without even the implied
# warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
# the GNU General Public License for more details.ept:

"""linux系统下面的实用工具，比如挂载磁盘，打开文件，等等"""

import os
def getDriverNames():
    """得到驱动器的名字，在这里是挂载在/media/下面的硬盘，
    如果非root权限运行则只能访问一般文件"""
    root = "/"
    for i in os.listdir(root):
        yield root + i
        

def getTraverseDir():
    """返回需要遍历的目录以及文件"""
    files = []
    dirs =[]
    for d in getDriverNames():
        for i in os.listdir(unicode(d, "utf-8")):
            fullpath = os.path.join(d, i)
            if os.path.isdir(fullpath):
                dirs.append(fullpath)
            else:files.append(fullpath)
    return dirs,files

def isHideFile(f):
    '''
    判断linux系统下面是否是隐藏文件
    '''
    if f.startswith("."):return True
    else:return False   

def getUserName():
    """得到用户的名称"""
    # return win32api.GetComputerName()
    pass
def openfile(unicodestr):
    """接受一个unicode字符串打开文件"""
    import locale
    cmd = u"xdg-open " + unicodestr
    cmd = cmd.replace("/", "\\")
    ccmd = cmd.encode(locale.getpreferredencoding())
    try:
        os.system(ccmd)
        logging.info(u"open file:" + cmd)
    except Exception, e:
        logging.info(str(e))

def getsize(size):
    if size / 1024 < 1:
        return u"%.2fB" % size
    else:
        size = size / 1024
        if size / 1024 < 1:
            return u"%.2fK" % size
        else:
            size = size / 1024
            if size / 1024 < 1:
                return u"%.2fM" % size
            else:
                size = size / 1024
                if size / 1024 < 1:
                    return u"%.2fG" % size
  
def isNTFS():
    pass
if __name__ == '__main__':
    print getUserName()
    for i in getDriverNames():
        print(i[:-1])
    dirs,files = getTraverseDir()
    for i in dirs:
        print i.encode("gbk")
    print isNTFS()
if __name__ == "__main__":
    for i in getDriverNames():
        print i
