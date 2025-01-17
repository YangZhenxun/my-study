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


import sys
import multiprocessing
import os
import locale
from utils import Contents, Zcompress, journal
from DAO import DAO

if sys.platform == "win32":
    from platform.winutils import *
elif sys.platform.startswith("linux"):
    from platform.linuxutils import *
else:
    os.exit(1)    
    
def traverse(path):
    '''
    接收一个目录参数，对此目录下面的所有文件进行遍历
    返回一个字典类型以供欧插入数据库
    '''
    data = {}
    #for root,dirs,files in os.walk(path):
        # for d in dirs:
        #     if "$RECYCLE.BIN" in d:
        #         dirs.remove(d)
        #     if "$Recycle.Bin" in d:
        #     temp.remove(d)
    #    data[root] = dirs + files
    # 使用usn读取数据
    # print path
    # j = journal.Journal(path[:-1])
    # j.process()
    # data = j.changed_paths
    # j.save_state(Contents.USNLOG_PATH + path[:-2])
    # 坑爹的python字符串
    # code = locale.getpreferredencoding()
    # print code
    path = path.decode("utf-8")
    # 转化为unicode码print path
    for  root, dirs, files in os.walk(path):
        data[root] = dirs + files
        # print [(i,i.decode(code)) for i in dirs + files], #type(root),repr(files[0])
        # print dirs,files
    return data
    
def fast_traverse(task_quene,result_quene):
    '''
    进行遍历的多进程操作函数，采用队列的方式防止死锁
    task_quene为任务队列，result是结果队列
    多个进程进行遍历操作
    '''
    # print printinfo()
    while True:
        next_task = task_quene.get()
        print printinfo(), "get task: ", next_task
        if next_task is None:
            print printinfo(), "Exit..."
            task_quene.task_done()
            result_quene.put(None)
            break
        else:
            data = traverse(next_task)
            task_quene.task_done()
            result_quene.put(data)

def insert_data(result_quene,pro_num):
    '''
    根据多进程的遍历结果将数据插入数据库
    '''
    # print printinfo()
    Db = DAO.Db()
    Db.drop_data()
    count = pro_num
    while True:
        if count ==0:
            print printinfo(),"done.."
            break
        next_data = result_quene.get()
        print printinfo(), "get task add: "
        if  next_data is None:
            count = count - 1
        else:
            #Db.add_by_fullpath(next_data)
            Db.add(next_data)
            print "task done!"
        result_quene.task_done()
    Db.create_index()
    Zcompress.compress(Contents.TEMP_PATH,Contents.DATA_PATH,1)

    
def fast_update():
    """创建两倍于CPU核的进程同时执行任务进行高效遍历"""
    num = multiprocessing.cpu_count() * 2
    tasks = multiprocessing.JoinableQueue()
    results = multiprocessing.JoinableQueue()
    tralist = [multiprocessing.Process(target = fast_traverse,args = (tasks,results)) for i in range(num)]
    insertpro = multiprocessing.Process(target = insert_data,args = (results,num))
    
    for i in tralist:
        i.start()
    insertpro.start()

    for i in getDriverNames():
        tasks.put(i)
    for i in range(num):
        tasks.put(None)
    print tasks
    for i in tralist:
        i.join()
    insertpro.join()
    # pickle.dump(winutils.getUserName(),open("data.dat","wb"))
    
def printinfo():
    # print('module name:', __name__)
    # print('parent process:', os.getppid())
    return ('process id:', os.getpid())


def test():
    """测试函数"""
    tralist = [multiprocessing.Process(target = traverse,args = (c,)) for c in getDriverNames()]
    for i in tralist:
        i.start()
    for i in tralist:
        i.join()
        
        
if __name__ == '__main__':
    import profile,pstats
    profile.run("fast_update()","log.log")
    p = pstats.Stats("log.log")
    p.sort_stats("time").print_stats(20)
    # fast_update()
    # print traverse(r"e:\new").values()[0][0].encode("gbk")
    # data = traverse(u"E:")
    # for (m,n) in data.items():
    #     print m.encode("gbk")
    # for root,dirs,files in os.walk(u"E:\\"):
    #      print dirs,files

    # print repr(u"E:\\哈哈")
    # print repr(unicode("E:\\哈哈","utf-8"))