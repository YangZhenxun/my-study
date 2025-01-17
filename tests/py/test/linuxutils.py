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
        
        
if __name__ == "__main__":
    for i in getDriverNames():
        print i
