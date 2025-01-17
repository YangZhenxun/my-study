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

'''此模块是应用程序用到的一些常量，比如正在工作的路径
sqlite3数据库存放的路径，USN日志存放的路径等等
为了解耦单独放在一个文件
'''
import os
import pickle
import sys
import locale
import mimetypes
mimetypes.common_types.update(mimetypes.types_map)
known_types = mimetypes.common_types

def _get_file_typelist(filetype):
    result = []
    for key, value in known_types.items():
        if value.startswith(filetype):
            result.append(unicode(key))
    return result

# the paths
LOCAL_ENCODING = locale.getpreferredencoding()
current_path = os.path.abspath(unicode(__file__, LOCAL_ENCODING))# current path of the application
folder_path = os.path.dirname(os.path.dirname(current_path))
DATA_DIR = os.path.join(folder_path,u"data")  # 数据库文件存放的路径
TEMP_PATH = os.path.join(DATA_DIR,u"temp")
TEMP_DIR = os.path.join(DATA_DIR,u"tempdir")
DATA_PATH = os.path.join(DATA_DIR,u"data")
USNLOG_PATH = os.path.join(DATA_DIR,u"usnlog")  # usn日志存放的路径 
LOG_PATH = os.path.join(DATA_DIR,u"log")
CONFIG_PATH = os.path.join(DATA_DIR,u"config")
TEMP_INDEX = os.path.join(DATA_DIR,u"tempindex")
try:  #　判断是否是同一个用户以便更新数据库
    USER_NAME = pickle.load(open("data.dat"),"rb")
except:
    USER_NAME = ""
# the types

MUSIC_TYPE = [u".mp3", u".wav", u".aac"] + (_get_file_typelist("audio"))  # 文件类型识别，暂时只是用后缀名
MOVIE_TYPE = [u".mp4", u".rvmb", u".swf"] + (_get_file_typelist("video"))
DOC_TYPE = [u".pdf", u".doc", u".ppt", u".txt", u".php"] + (_get_file_typelist("text"))
PIC_TYPE = [u".png", u".jpg", u".gif"] + (_get_file_typelist("image"))
APPLICATION_TYPE = _get_file_typelist("application")

if __name__ == '__main__':
    print "data path",DATA_PATH,type(DATA_DIR)
    print "temp path",TEMP_PATH
    print "log path",USNLOG_PATH
    print "current path",current_path
    print _get_file_typelist("audio")
    print _get_file_typelist("application")
    # print u"|".join([u"/.*(?!\%s)$/" % i for i in DOC_TYPE + MUSIC_TYPE + PIC_TYPE + MOVIE_TYPE + APPLICATION_TYPE])
    print u"|".join([u"/.*(?!\.%s$)/" % i[1:] for i in DOC_TYPE])