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
import win32file,win32api

def getDriverNames():
    """
    返回可用的windows系统上面的驱动器的盘符
    如 D:\
    """
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
    cmd = unicodestr.encode(locale.getpreferredencoding()).replace("\\", "\\\\")
    print cmd
    os.system(cmd)


if __name__ == '__main__':
    print getUserName()
    for i in getDriverNames():
        print(i[:-1])
