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

'''
Data access object 数据访问对象
此模块封装了对数据库的各种元操作，比如增删改查等等
old_version:
起初想法是新建一个Db类就能代表一个数据库的实例，没一个实例都获得了数据库的连接
后来发觉这样不合理，不应该进行这种绑定，作为一个Data Access object
提供能访问数据库的接口就可以了
另外一个原因是，创建多线程的时候，如果Add_Record是方法，那么创建类的时候
就获得了数据库的绑定，在此调用的时候不在同一个线程，无法完成
new_version:
数据库只提供操作不绑定具体是那一个数据库
'''

import os
import sqlite3
from utils import Contents

class Db():
    """提供访问数据库sqlite3的类，对其进行良好封装以便外部使用，
    将数据层与业务层分开
    """
    
    def __init__(self):
        """初始化数据库文件的路径"""
        self.path = Contents.TEMP_PATH
        self.__createable()
        
    def __del__(self):
        """释放资源，提交事务关闭数据库"""
        self.con.commit()
        self.con.close()

    def __conn(self):
        '''
        获取数据库连接对象
        '''
        try:
            self.con = sqlite3.connect(self.path)
        except Exception as e:
            print(e,"cant't get a database")
        
    def create_index(self):
        """为数据库添加索引"""
        self.con.execute("create index idx_file on catalog(file)")
        
    def __createable(self):
        '''
        新建数据表
        '''
        self.__conn()
        # sql = '''
        #     create TABLE IF NOT EXISTS catalog(
        #     file text not null,
        #     parentdir text not null,
        #     isdir integer not null,
        #     mtime real not null
        #     )
        # '''
        sql = '''
            create TABLE IF NOT EXISTS catalog(
            file char(30) not null,
            parentdir text not null
            )
        '''#不支持太长的文件名以提高效率
        self.con.execute(sql)

                    
    def add(self,data):
        '''添加数据，
        input:
            data:字典，键是一个目录，值是目录下面所有文件的列表
            {parentdir1: [file1, file2, file3 ...],
                    parentdir2: [file1, file2, file3...]}
        rertun:
            None
        其中file，parentdirs均为列表
        '''
        for parentdir in data:
            # directory,filename = os.path.split(j)
            # try:
            #     mtime = os.path.getmtime(j)
            # except Exception as e:
            #     print(e)
            #     continue
            for filename in data[parentdir]:
                self.con.execute('insert into catalog(file,parentdir) values ("%s","%s")' % (filename, parentdir))
        
        # for i in data:
            
        #     parentdir,filename = os.path.split(i)
        #     self.con.execute('insert into catalog(file,parentdir) values ("%s","%s")' % (filename, parentdir))
        self.con.commit()
        
    def add_by_fullpath(self,fullpaths):
        """根据全路径添加数据，这是为了USN访问的方便，因为他返回的是全部数据
        input:
            fullpath:文件的全路径
        return:
            None"""
        
        for parentdir,filename in [os.path.split(fullpath) for fullpath in fullpaths]:
            self.con.execute('insert into catalog(file,parentdir) values ("%s","%s")' % (filename, parentdir))
        self.con.commit()
        
    def drop_data(self):
        """删除数据，由于sqlite3中使用drop命令数据删除但是文件并没有变小
        因此这里采用清空文件的做法"""
        # sql = 'delete TABLE catalog'
        # self.query(sql)
        # 此方法不可行，并没有完全删除文件
        with open(self.path,'w'):pass
        self.__createable()
        
    def update(self):
        """更新数据库，暂未实现"""
        pass
        
    def query(self, sql):
        """数据库查询操作
        input:
            sql:待查询的sql语句
        return:
            查询结果，是一个列表"""
        cur = self.con.cursor()
        cur.execute(sql)
        self.con.cursor()
        return cur
        
        
if __name__ == '__main__':
    db = Db()
    t = ["E:\\FileSearch\\filesearch.py","E:\\FileSearch\\DAO.py"]
    db.add_by_fullpath(t)
    
