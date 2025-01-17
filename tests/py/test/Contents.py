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

# the paths
current_path = os.path.abspath(
    os.path.dirname(sys.argv[0]))# current path of the application
DATA_DIR = os.path.join(current_path,"data")  # 数据库文件存放的路径
TEMP_PATH = os.path.join(DATA_DIR,"temp")
DATA_PATH = os.path.join(DATA_DIR,"data")
USNLOG_PATH = os.path.join(DATA_DIR,"usnlog")  # usn日志存放的路径 
LOG_PATH = os.path.join(DATA_DIR,"log")
CONFIG_PATH = os.path.join(DATA_DIR,"config")
TEMP_INDEX = os.path.join(DATA_DIR,"tempindex")
try:  #　判断是否是同一个用户以便更新数据库
    USER_NAME = pickle.load(open("data.dat"),"rb")
except:
    USER_NAME = ""
# the types

MUSIC_TYPE = [".mp3", ".wav", ".aac"]  # 文件类型识别，暂时只是用后缀名
MOVIE_TYPE = [".mp4", ".rvmb", ".swf"]
DOC_TYPE = [".pdf", ".doc", ".ppt", ".txt", ".php"]
PIC_TYPE = [".png", ".jpg", ".gif"]

if __name__ == '__main__':
    print "数据库路径: ",DATA_PATH
    print "临时文件路径",TEMP_PATH
    print "日志路径",USNLOG_PATH
    print "当前路径",current_path
