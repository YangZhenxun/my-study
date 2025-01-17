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

import zlib
"""ʹ��zlib���ѹ���㷨�Եõ��������ļ�����ѹ���洢"""


def compress(infile, dst, level=1):
    """ѹ�������ļ�
    input:
        infile: ��Ҫѹ���������ļ�
           dst: ѹ�����ļ����õ�·��
         level: ѹ������,Ĭ�����9������Խ����Ҫʱ��Խ��
    return:
        None"""
    infile = open(infile, 'rb')
    dst = open(dst, 'wb')
    compress = zlib.compressobj(level)
    data = infile.read(1024)
    while data:
        dst.write(compress.compress(data))
        data = infile.read(1024)
    dst.write(compress.flush())
    
def decompress(infile, dst):
    """��ѹ�������ļ�
    input:
        infile: ��Ҫ��ѹ�����ļ�
           dst: ��ѹ��֮���ļ����õ�·��
    return:
        None"""
    infile = open(infile, 'rb')
    dst = open(dst, 'wb')
    decompress = zlib.decompressobj()
    data = infile.read(1024)
    while data:
        dst.write(decompress.decompress(data))
        data = infile.read(1024)
    dst.write(decompress.flush())

if __name__ == '__main__':
    filename = "D:\workspace\FileSearch\data.db"
    dst = "D:\workspace\FileSearch\data.zlib"
    compress(filename,dst)
