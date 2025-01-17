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

import os
import sys
import logging
import pickle
import time

from PyQt4.QtGui import *
from PyQt4.QtCore import *
from PyQt4.QtSql import *
from multiprocessing import freeze_support

from ui.Ui_filesearch import Ui_main
from utils import Contents, Zcompress, fast_traverse
from DAO import DAO

if sys.platform.startswith("win32"):
    from platform.winutils import *
elif sys.platform.startswith("linux"):
    from platform.linuxutils import *
else:
    os.exit(1)
    
_toUnicode = lambda s:unicode(s.toUtf8(),"utf8","ignore")

def createConnection():
    db = QSqlDatabase.addDatabase('QSQLITE')
    db.setDatabaseName(Contents.TEMP_PATH)
    if not db.open():
        QMessageBox.critical(None, qApp.tr("Cannot open database"),
                    qApp.tr("Unable to establish a database connection.\n"
                              "This example needs SQLite support. Please read "
                              "the Qt SQL driver documentation for information "
                              "how to build it.\n\n"
                              "Click Cancel to exit."),
                    QMessageBox.Cancel)
        return False
    return True

#---------------------------------------------------------------------------
class CustomSqlModel(QSqlQueryModel):
    def data(self, index, role):
        value = super(CustomSqlModel, self).data(index, role)
        if value is not None and role == Qt.DisplayRole:
            if index.column() == 0:
                return value
            elif index.column() == 2:
                row, column = index.row(), index.column()
                filename = self.data(self.index(row, column - 2), role).toString()
                dirname = self.data(self.index(row,column - 1), role).toString()
                sep = os.path.sep
                path = dirname.append(QString(sep)).append(filename) 
                path = _toUnicode(path)
                mtime = time.strftime("%Y/%m/%d %H:%M:%S", time.gmtime(os.stat(path).st_mtime))
                return mtime
        if role == Qt.TextColorRole and index.column() == 1:
            return QColor(Qt.blue)
        return value

class filesearch(QMainWindow, Ui_main):
    """
    filesearch界面类，提供了与用户交互的各种操作
    """
    def __init__(self, parent = None):
        """
        constructor
        """
        QMainWindow.__init__(self, parent)
        Ui_main.__init__(self)
        self.setupUi(self)
        self.Db = DAO.Db()
        self.sep = os.path.sep
        self.sql = u'select file,parentdir from catalog where file like "%%%s%%"'
        self.last_fileinfo_path = None
        if not createConnection():
            sys.exit(1)

        self.infomodel = CustomSqlModel()
        self.proxymodel = QSortFilterProxyModel()
        self.proxymodel.setSourceModel(self.infomodel)
        self.proxymodel.setDynamicSortFilter(True)
        self.caseinsensitity = Qt.CaseInsensitive
        self.matchtype = QRegExp.FixedString
        self.fileinfo.setModel(self.proxymodel)
        self.treemodel = QFileSystemModel()
        self.treemodel.setRootPath("/")
        self.tree.setModel(self.treemodel)
        for i in range(1,4):
            self.tree.setColumnHidden(i,True)
        self.tree.setHeaderHidden(True)
        self.tree.setStyleSheet("QTreeView::item:selected{background-color:red}"
            "QTreeView::item:hover{background-color:yellow}")
        self.fileinfo.setStyleSheet("QTableView::item:selected{background-color:red}"
            "QTableView::item:hover{background-color:yellow}")

        #self.searchedit.installEventFilter(self)
        self.connect(self.m_exit, SIGNAL('triggered()'), SLOT('close()'))
        
 #-----------------------------------------------------------------------------
    def set_table(self):
        self.infomodel.insertColumns(2, 1)
        self.infomodel.setHeaderData(0, Qt.Horizontal, "FileName")
        self.infomodel.setHeaderData(1, Qt.Horizontal, "Directory")
        self.infomodel.setHeaderData(2, Qt.Horizontal, "Modefied Time")
        self.fileinfo.setColumnWidth(0, 90)
        self.fileinfo.setColumnWidth(1, 235)
        self.fileinfo.setColumnWidth(2, 125)

#------------------------------------------------------------------------------
    @pyqtSignature("QString")
    def on_searchedit_textChanged(self, p0):
        """搜索条的响应函数，当用户输入内容发生改变时查询数据库并更新界面"""
        if self.m_matchall.isChecked() or self.m_wildcard.isChecked() \
                or self.m_regex.isChecked():pass
        else:
            self.infomodel.setQuery("select * from catalog where file like '%%%s%%'" % p0)
            self.set_table()

    @pyqtSignature("")
    def on_searchedit_returnPressed(self):
        if self.m_matchall.isChecked():
            self.search_by_whole()
        elif self.m_wildcard.isChecked():
            self.search_by_wildcard()
        elif self.m_regex.isChecked():
            self.search_by_regex()
        else:pass

#-----------------------------------------------------------------------------
    @pyqtSignature("QModelIndex")
    def on_fileinfo_clicked(self, index):
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            self.tree.collapseAll()
            filename = self.infomodel.data(index, role).toString()
            dirname = self.infomodel.data(self.infomodel.index(row,column + 1), role).toString()
            path = dirname.append(QString(self.sep)).append(filename) 
            self.statusBar.showMessage(path)
            mindex = self.treemodel.index(path)
            self.tree.setCurrentIndex(mindex)
            if os.path.isfile(_toUnicode(path)):
                self.show_detail(_toUnicode(path))
        elif column == 1:
            path = self.infomodel.data(index, role).toString()
            self.statusBar.showMessage(path)
            mindex = self.treemodel.index(path)
            if self.last_fileinfo_path != path:
                self.tree.setCurrentIndex(mindex)
            self.statusBar.showMessage(path)
            self.last_fileinfo_path = path
        else:
            self.statusBar.showMessage(self.infomodel.data(index, role))

#-----------------------------------------------------------------------------
    @pyqtSignature("QModelIndex")
    def on_fileinfo_doubleClicked(self,index):
        """文件列表项的双击响应函数，打开文件或者文件夹"""
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            filename = self.infomodel.data(index, role).toString()
            dirname = self.infomodel.data(self.infomodel.index(row,column + 1), role).toString()
            path = dirname.append(QString(self.sep)).append(filename) 
        elif column == 1:
            path = self.infomodel.data(index, role).toString()
        else:
            pass
        openfile(_toUnicode(path))

#-----------------------------------------------------------------------------        
    @pyqtSignature("QListWidgetItem*")
    def on_classlist_itemClicked(self, item):
        """
        分类操作的响应函数，这里是对列表下面的文件进行分类
        """
        pass

#-----------------------------------------------------------------------------            
    @pyqtSignature("")
    def on_m_update_triggered(self):
        """更新索引菜单的响应函数，点击之后更新索引文件"""
        fast_traverse.fast_update()
        self.statusBar.showMessage('Index Finished!')
    
#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_matchall_triggered(self):
        """匹配所有菜单的响应函数，选择之后不会进行部分匹配"""
        if self.m_matchall.isChecked():
            self.m_wildcard.setEnabled(False)
            self.m_regex.setEnabled(False)
        else:
            self.m_wildcard.setEnabled(True)
            self.m_regex.setEnabled(True)
        self.search_by_whole()

#-----------------------------------------------------------------------------    
    @pyqtSignature("")
    def on_m_wildcard_triggered(self):
        """使用通配符"""
        if self.m_wildcard.isChecked():
            self.m_matchall.setEnabled(False)
            self.m_regex.setEnabled(False)
        else:
            self.m_matchall.setEnabled(True)
            self.m_regex.setEnabled(True)
        self.matchtype = QRegExp.Wildcard
        self.search_by_wildcard()

#-----------------------------------------------------------------------------            
    @pyqtSignature("")
    def on_m_regex_triggered(self):
        """Slot documentation goes here."""
        if self.m_regex.isChecked():
            self.m_matchall.setEnabled(False)
            self.m_wildcard.setEnabled(False)
        else:
            self.m_matchall.setEnabled(True)
            self.m_wildcard.setEnabled(True)
        self.matchtype = RegExp
        self.search_by_regex()


#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_ssign_directory_triggered(self):
        dirname = QFileDialog.getExistingDirectory(self, "Select Directory",
            ".", QFileDialog.ShowDirsOnly)

#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_Chinese_triggered(self):
        data = {}
        fp = open(Contents.CONFIG_PATH,"wb")
        data["language"] = "zh_CN"
        pickle.dump(data,fp)
        fp.close()

#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_English_triggered(self):
        data = {}
        fp = open(Contents.CONFIG_PATH,"wb")
        data["language"] = "en"
        pickle.dump(data,fp)
        fp.close()
#-----------------------------------------------------------------------------
    def on_quit(self):
       # os.remove(Contents.TEMP_PATH)
       pass
       
    def show_detail(self,filename):
        """显示文件的详细信息"""
        fp = open(filename)
        info = []
        for i in range(5):
            info += (fp.read(40) + "\n")
        self.text_detail.setText("".join(info))  
        
    def sort_files(self, type):
        """根据给定的类型对列出的文件进行分类"""
        self.fileinfo.clear()
        for i in self.result:
            if type == "*":
                filename = os.path.join(i[1].encode("utf8"), i[0].encode("utf8"))
                listone = QTreeWidgetItem(self.fileinfo)
                listone.setText(0, i[0])
                listone.setText(1, i[1])
                if os.path.isdir(filename):
                    icon = QIcon(":file/images/folder.png")
                    #icon.addPixmap(QtGui.QPixmap(":file/images/folder.png"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
                    listone.setIcon(0, icon)
                    listone.setText(3,"1")
                else:
                    icon = QIcon(":file/images/file.ico")
                    #icon.addPixmap(QtGui.QPixmap("images/file.ico"), QtGui.QIcon.Normal, QtGui.QIcon.Off)
                    listone.setIcon(0, icon)
                    listone.setText(3,"0")
            else:
                for j in type:
                    if i[0].endswith(j):
                        listone = QTreeWidgetItem(self.fileinfo)
                        listone.setText(0, i[0])
                        listone.setText(1, i[1])
        self.fileinfo.sortItems(3,1)

    def eventFilter(self, watch, event):
        if watch == self.searchedit:
            if (event.type() == QEvent.FocusIn):
                self.statusBar.showMessage('Press Enter to search')
                return True
        return False

    def search_by_whole(self):
        self.infomodel.setQuery("select * from catalog")
        self.infomodel.removeColumns(2, 1)
        self.proxymodel.setFilterRegExp(
            QRegExp(QString("^").append(self.searchedit.text()).append(QString("$")), 
            self.caseinsensitity, QRegExp.RegExp))
        self.set_table()

    def search_by_wildcard(self):
        self.infomodel.setQuery("select * from catalog")
        self.infomodel.removeColumns(2, 1)
        self.proxymodel.setFilterRegExp(QRegExp(self.searchedit.text(), 
            self.caseinsensitity, self.matchtype))
        self.set_table()

    def search_by_regex(self):
        self.infomodel.setQuery("select * from catalog")
        self.infomodel.removeColumns(2, 1)
        self.proxymodel.setFilterRegExp(QRegExp(self.searchedit.text(), 
            self.caseinsensitity, self.matchtype))
        self.set_table()


if __name__ == '__main__':
    freeze_support() # 多线程打包的操作函数
    logging.basicConfig(filename = Contents.LOG_PATH, level = logging.INFO)
    app = QApplication(sys.argv)
    
    try:
        fp = open(Contents.CONFIG_PATH, "rb")
        data = pickle.load(fp)
        trans = QTranslator() # 添加国际化的支持
        trans.load("./language/" + data.get("language",None))
        app.installTranslator(trans)
    except Exception:
        logging.info("language load failed")
    
    # 启动界面
    splash=QSplashScreen(QPixmap(":/file/images/splash.jpg"))
    splash.show()
    splash.showMessage(u"加载核心模块...", Qt.AlignBottom, Qt.white)
    # if not (os.path.exists(Contents.DATA_PATH) and (Contents.USER_NAME == winutils.getUserName())):
    if not os.path.exists(Contents.DATA_PATH):
        print Contents.DATA_PATH
        splash.showMessage(u"更新数据中...", Qt.AlignBottom, Qt.white)
        fast_traverse.fast_update()
    else:
        Zcompress.decompress(Contents.DATA_PATH,Contents.TEMP_PATH)
    splash.showMessage(u"正在启动...", Qt.AlignBottom, Qt.white)
    app.processEvents()
    filesearch = filesearch()
    filesearch.show()
    splash.finish(filesearch) 
    app.connect(app,SIGNAL('aboutToQuit'),filesearch.on_quit) 
    sys.exit(app.exec_())

# sqlite3是支持正则表达式的！
# 哈希到底是什么 速度如何
# model的过滤函数
# setRootIndex啥意思