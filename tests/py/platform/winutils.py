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

"""windows系统下面的实用工具操作"""

import os
import win32file
import win32api
import logging

def getDriverNames():
    """
    返回可用的windows系统上面的驱动器的盘符
    如 D:\
    # """
    drivebits = win32file.GetLogicalDrives()
    for d in range(1,26):
        mask = 1 << d
        if drivebits & mask:
            try:
                drname = '%c:\\' % chr(ord('A') + d)
                win32api.GetVolumeInformation(drname)[0]
                yield drname
            except:
                pass
    # return ["C:\\"]

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
    判断windows系统下面是否是隐藏文件
    '''
    flag = win32file.GetFileAttributesW(f)
    if flag&2 != 0:
        return True
    else:
        return False

def getUserName():
    """得到用户的名称"""
    return win32api.GetComputerName()
def openfile(unicodestr):
    """接受一个unicode字符串打开文件"""
    import locale
    cmd = u"explorer.exe " + unicodestr
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
    for i in getDriverNames():
        if win32api.GetVolumeInformation(i)[-1] != "NTFS":
            return 0
    else:return 1
if __name__ == '__main__':
    print getUserName()
    for i in getDriverNames():
        print(i[:-1])
    dirs,files = getTraverseDir()
    for i in dirs:
        print i.encode("gbk")
    print isNTFS()