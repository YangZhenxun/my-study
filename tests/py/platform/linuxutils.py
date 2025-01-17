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

"""linuxϵͳ�����ʵ�ù��ߣ�������ش��̣����ļ����ȵ�"""

import os
def getDriverNames():
    """�õ������������֣��������ǹ�����/media/�����Ӳ�̣�
    �����rootȨ��������ֻ�ܷ���һ���ļ�"""
    root = "/"
    for i in os.listdir(root):
        yield root + i
        

def getTraverseDir():
    """������Ҫ������Ŀ¼�Լ��ļ�"""
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
    �ж�linuxϵͳ�����Ƿ��������ļ�
    '''
    if f.startswith("."):return True
    else:return False   

def getUserName():
    """�õ��û�������"""
    # return win32api.GetComputerName()
    pass
def openfile(unicodestr):
    """����һ��unicode�ַ������ļ�"""
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
