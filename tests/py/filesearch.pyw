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

from __future__ import division

import os
import sys
import logging
import pickle
import time
import threading

from PyQt4.QtGui import *
from PyQt4.QtCore import *
from PyQt4.QtSql import *
from multiprocessing import freeze_support

from ui.Ui_filesearch import Ui_main
from utils import Contents, Zcompress, journal
import fast_traverse
from DAO import DAO

IS_NTFS = 0
sep = '/'
if sys.platform.startswith("win32"):
    from platform.winutils import *
    IS_NTFS = isNTFS()
    if not IS_NTFS:sep = os.path.sep
elif sys.platform.startswith("linux"):
    from platform.linuxutils import *
else:
    os.exit(1)
    
_toUnicode = lambda s:unicode(s.toUtf8(),"utf8","ignore")

def createConnection():
    try:
        db = QSqlDatabase.addDatabase('QSQLITE')
    except Exception, e:
        logging.info(str(e))
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
        row, column = index.row(), index.column()
        if role == Qt.DisplayRole:
            if column == 2:
                filename = self.data(self.index(row, column - 2), role).toString()
                dirname = self.data(self.index(row,column - 1), role).toString()
                path = dirname.append(QString(sep)).append(filename) 
                path = _toUnicode(path)
                if os.path.exists(path):
                    mtime = time.strftime("%Y/%m/%d %H:%M:%S", time.gmtime(os.stat(path).st_mtime))
                    # return os.path.getsize(path)
                    return mtime
                else:return QString()
            else:
                return super(CustomSqlModel, self).data(index, role)
        elif role == Qt.DecorationRole:
            if column == 0:
                filename = super(CustomSqlModel, self).data(index, Qt.DisplayRole).toString()
                dirname = self.data(self.index(row, column + 1), Qt.DisplayRole).toString()
                path = dirname.append(QString(sep)).append(filename) 
                logging.info(_toUnicode(path))
                if os.path.isdir(_toUnicode(path)):
                    return QPixmap(":/file/images/folder.png")
                else:
                    postfix = os.path.splitext(_toUnicode(filename))[-1]#得到后缀名
                    if postfix in Contents.MUSIC_TYPE:
                        return QPixmap(":/file/images/music.png")
                    elif postfix in Contents.MOVIE_TYPE:
                        return QPixmap(":/file/images/vedio.png")
                    elif postfix in Contents.DOC_TYPE:
                        return QPixmap(":/file/images/doc.png")
                    elif postfix in Contents.PIC_TYPE:
                        return QPixmap(":/file/images/picture.png")
                    else:return QPixmap(":/file/images/andoc.png")
            return QString()
        elif role == Qt.TextColorRole:
            if column == 1:
                return QColor(Qt.blue)
            else:return QString()            
        else:     
            return super(CustomSqlModel, self).data(index, role)

#     def flag(self, index):
#         if not index.isValid():
#             return Qt.ItemIsEnabled;
#         return QAbstractTableModel.flags(index) | Qt.ItemIsDragEnabled | Qt.ItemIsDropEnabled
# class CustomProxyModel(QSortFilterProxyModel):
#     """docstring for CustomProxyModel"""
#     def __init__(self):
#         super(CustomProxyModel, self).__init__()
        
#     def flag(self, index):
#         if not index.isValid():
#             return Qt.ItemIsEnabled;
#         return QAbstractTableModel.flags(index) | Qt.ItemIsDragEnabled | Qt.ItemIsDropEnabled
#     def supportedDropActions(self):
#         return Qt.MoveAction;
#-----------------------------------------------------------------------------


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
        self.setGeometry(100, 150, 800, 520)
        self.setupUi(self)
        self.open_database()
        # fast_traverse.update()
        self.last_fileinfo_path = None
        self.open_database()
        self.total = [i for i in self.Db.con.execute("SELECT count(*) from catalog")][0][0]
        # print self.total
        self.setAcceptDrops(True)
        
        self.progressBar.setVisible(False)
        self.stop.setVisible(False)
        self.textdetail.setVisible(False)
        self.collapse.setVisible(False)
        self.infomodel = CustomSqlModel()
        # for i in self.infomodel.mimeTypes():
        #     print i
        # self.fileinfo.setDragDropOverwriteMode(False);
        # self.fileinfo.setDragEnabled(True);
        # self.fileinfo.setDragDropMode(QAbstractItemView.InternalMove);
        # self.fileinfo.setSelectionMode(QAbstractItemView.ExtendedSelection);
        # self.fileinfo.setDefaultDropAction(Qt.MoveAction);
        self.proxymodel = QSortFilterProxyModel()
        self.proxymodel.setSourceModel(self.infomodel)
        self.proxymodel.setDynamicSortFilter(True)
        self.rowcount = 0
        self.interrupt = False
        self.config = {}
        self.caseinsensitity = Qt.CaseInsensitive
        self.position = qApp.tr("All Volume")
        self.typefilter = QSortFilterProxyModel()
        # self.typefilter = CustomProxyModel()
        self.typefilter.setSourceModel(self.proxymodel)
        self.typefilter.setDynamicSortFilter(True)
        self.fileinfo.setModel(self.typefilter)
        self.treemodel = QFileSystemModel()
        self.treemodel.setRootPath("/")
        for i in self.treemodel.mimeTypes():
            print i
        # self.treemodel.setFilter(QDir.AllDirs | QDir.Readable | QDir.NoDotAndDotDot)
        self.tree.setModel(self.treemodel)
        for i in range(1,4):
            self.tree.setColumnHidden(i,True)
        self.tree.setHeaderHidden(True)
        

        self.tablemenu = QMenu(self.fileinfo)
        self.mopenfile = self.tablemenu.addAction(qApp.tr("open"))
        self.mopendirectory = self.tablemenu.addAction(qApp.tr("open directory"))
        self.mcopyfilename = self.tablemenu.addAction(qApp.tr("copy file name to clipboard"))
        self.mcopydirectoryname = self.tablemenu.addAction(qApp.tr("copy directory name to clipboard"))
        
        self.treemenu = QMenu(self.tree)
        #self.searchedit.installEventFilter(self)
        self.connect(self.m_exit, SIGNAL('triggered()'), SLOT('close()'))
        self.connect(self.mopenfile, SIGNAL('triggered()'), self.on_menu_open)
        self.connect(self.mopendirectory, SIGNAL("triggered()"), self.on_mopendirectory)
        self.connect(self.mcopyfilename, SIGNAL("triggered()"), self.on_mcopyfilename)
        self.connect(self.mcopydirectoryname, SIGNAL("triggered()"), self.on_mcopydirectoryname)

        self.stringlist = QStringList()
        self.completemodel = QStringListModel()
        self.completer = QCompleter(self)
        self.completer.setModel(self.completemodel)
        self.searchedit.setCompleter(self.completer)
        self.searchedit.setFocus(True)
        self.tree.setStyleSheet("QTreeView.item:selected{background-color:#ccd5ff}"
            "QTreeView.item:hover{background-color:#ccd566}")
        self.fileinfo.setStyleSheet("QTableView.item:selected{background-color:#ccd5ff}"
            "QTableView.item:hover{background-color:#ccd566}")
 #-----------------------------------------------------------------------------
    def set_table(self):
        self.infomodel.insertColumns(2, 1)
        self.infomodel.setHeaderData(0, Qt.Horizontal, qApp.tr("FileName"))
        self.infomodel.setHeaderData(1, Qt.Horizontal, qApp.tr("Directory"))
        self.infomodel.setHeaderData(2, Qt.Horizontal, qApp.tr("Modefied Time"))
        self.fileinfo.setColumnWidth(0, 150)
        self.fileinfo.setColumnWidth(1, 260)
        self.fileinfo.setColumnWidth(2, 125)

#------------------------------------------------------------------------------
    @pyqtSignature("QString")
    def on_searchedit_textChanged(self, p0):
        """搜索条的响应函数，当用户输入内容发生改变时自动补齐"""
        self.setWindowTitle(p0 + qApp.tr(" -Filesearch- Location: ") + self.position)
        text = _toUnicode(p0)
        textafter = u""
        try:
            textafter = text[:-1] + unichr(ord(text[-1]) + 1)
        except Exception, e:
            logging.info(str(e))
        cur = self.Db.query("SELECT file FROM catalog WHERE file > '%s' AND file < '%s' LIMIT 10" % (text, textafter))
        for i in cur:
            self.stringlist.append(i[0])
        self.completemodel.setStringList(self.stringlist)

#------------------------------------------------------------------------------
    @pyqtSignature("")
    def on_searchedit_returnPressed(self):
        self.searchedit.setVisible(False)
        self.progressBar.setVisible(True)
        self.progressBar.setRange(0,self.total)
        self.stop.setVisible(True)
        self.infomodel.removeColumns(2, 1) 
        start = time.clock()
        if self.m_matchall.isChecked():
            self.infomodel.setQuery("SELECT * FROM catalog")
            self.search_by_whole()
        elif self.m_wildcard.isChecked():
            self.infomodel.setQuery("SELECT * FROM catalog")
            self.search_by_wildcard()
        elif self.m_regex.isChecked():
            self.infomodel.setQuery("SELECT * FROM catalog")
            self.search_by_regex()
        else:
            self.normal_search()
        self.set_table()
        
        self.progressBar.setVisible(False)
        self.searchedit.setVisible(True)
        self.stop.setVisible(False)
        self.searchedit.setFocus(True)
        self.statusBar.showMessage("Finished! %.6fms used!" % (time.clock() - start))

#------------------------------------------------------------------------------
    @pyqtSignature("")
    def on_stretch_clicked(self):
        self.resize(1070, 520)
        self.stretch.setVisible(False)
        self.textdetail.setVisible(True)
        self.collapse.setVisible(True)
        index = self.fileinfo.currentIndex()
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            filename = self.typefilter.data(index, role).toString()
            dirname = self.typefilter.data(self.typefilter.index(row,column + 1), role).toString()
            path = dirname.append(QString(sep)).append(filename) 
            self.show_detail(_toUnicode(path))
        elif column == 1:
            path = self.typefilter.data(index, role).toString()
            self.show_detail(_toUnicode(path))
        else:pass

#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_collapse_clicked(self):
        self.resize(800, 520)
        self.collapse.setVisible(False)
        self.textdetail.setVisible(False)
        self.stretch.setVisible(True)

#-----------------------------------------------------------------------------
    @pyqtSignature("QModelIndex")
    def on_fileinfo_clicked(self, index):
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            self.tree.collapseAll()
            filename = self.typefilter.data(index, role).toString()
            dirname = self.typefilter.data(self.typefilter.index(row,column + 1), role).toString()
            path = dirname.append(QString(sep)).append(filename) 
            self.statusBar.showMessage(path)
            mindex = self.treemodel.index(path)
            self.tree.setCurrentIndex(mindex)
            if self.textdetail.isVisible():
                self.show_detail(_toUnicode(path))
        elif column == 1:
            path = self.typefilter.data(index, role).toString()
            self.statusBar.showMessage(path)
            mindex = self.treemodel.index(path)
            if self.last_fileinfo_path != path:
                self.tree.setCurrentIndex(mindex)
            self.statusBar.showMessage(path)
            self.last_fileinfo_path = path
            if self.textdetail.isVisible():
                self.show_detail(_toUnicode(path))
        else:
            self.statusBar.showMessage(self.typefilter.data(index, role))

#-----------------------------------------------------------------------------
    @pyqtSignature("QModelIndex")
    def on_fileinfo_doubleClicked(self,index):
        """文件列表项的双击响应函数，打开文件或者文件夹"""
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            filename = self.typefilter.data(index, role).toString()
            dirname = self.typefilter.data(self.typefilter.index(row,column + 1), role).toString()
            path = dirname.append(QString(sep)).append(filename) 
        elif column == 1:
            path = self.typefilter.data(index, role).toString()
        else:
            pass
        openfile(_toUnicode(path))
        logging.info("open file: " + _toUnicode(path))

#-----------------------------------------------------------------------------        
    @pyqtSignature("int")
    def on_classlist_currentIndexChanged(self, item):
        """
        分类操作的响应函数，这里是对列表下面的文件进行分类
        """
        self.filter_type(item)

#-----------------------------menu action ------------------------------------
#-----------------------------------------------------------------------------                
    @pyqtSignature("")
    def on_m_new_index_triggered(self):
        """更新索引菜单的响应函数，点击之后更新索引文件"""
        self.close_database()
        fast_traverse.fast_update(IS_NTFS)
        self.open_database()
        self.statusBar.showMessage(qApp.tr('Index Finished!'))

#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_update_triggered(self):
        self.close_database()
        self.update()
        self.open_database()
        self.statusBar.showMessage(qApp.tr("Update Finished!"))

#----------------------------------------------------------------------------- 
    @pyqtSignature("")
    def on_m_normal_mode_triggered(self):
        if self.m_normal_mode.isChecked():
            self.m_matchall.setChecked(False)
            self.m_wildcard.setChecked(False)
            self.m_regex.setChecked(False)
            
            self.normal_search()

#----------------------------------------------------------------------------- 
    @pyqtSignature("")
    def on_m_matchall_triggered(self):
        """匹配所有菜单的响应函数，选择之后不会进行部分匹配"""
        if self.m_matchall.isChecked():
            self.m_normal_mode.setChecked(False)
            self.m_wildcard.setChecked(False)
            self.m_regex.setChecked(False)
            # self.search_by_whole()

#-----------------------------------------------------------------------------    
    @pyqtSignature("")
    def on_m_wildcard_triggered(self):
        """使用通配符"""
        if self.m_wildcard.isChecked():
            self.m_normal_mode.setChecked(False)
            self.m_matchall.setChecked(False)
            self.m_regex.setChecked(False)
            # self.search_by_wildcard()

#-----------------------------------------------------------------------------            
    @pyqtSignature("")
    def on_m_regex_triggered(self):
        """Slot documentation goes here."""
        if self.m_regex.isChecked():
            self.m_normal_mode.setChecked(False)
            self.m_matchall.setChecked(False)
            self.m_wildcard.setChecked(False)
            # 把过滤器的信息清零
            # self.search_by_regex()

#----------------------------------------------------------------------------- 
    @pyqtSignature("")
    def on_m_case_sensitive_triggered(self):
        if self.m_case_sensitive.isChecked():
            self.caseinsensitity = Qt.CaseSensitive
        else:
            self.caseinsensitity = Qt.CaseInsensitive
        self.on_searchedit_returnPressed()

#-----------------------------------------------------------------------------       
    @pyqtSignature("")
    def on_m_all_volume_triggered(self):
        self.m_select_directory.setChecked(False)
        self.proxymodel.setFilterKeyColumn(0)
        self.proxymodel.setFilterRegExp(
            QRegExp(QString(""), 
            self.caseinsensitity, QRegExp.RegExp))
        self.on_searchedit_returnPressed()
        self.setWindowTitle(qApp.tr("Filesearch- Location: All Volume"))
        self.m_all_volume.setChecked(True)

#-----------------------------------------------------------------------------  
    @pyqtSignature("")
    def on_m_select_directory_triggered(self):
        dirname = QFileDialog.getExistingDirectory(self, qApp.tr("Select Directory"),
            ".", QFileDialog.ShowDirsOnly)
        dirname = dirname.replace("\\", "/")
        print dirname
        self.proxymodel.setFilterKeyColumn(1)
        self.proxymodel.setFilterRegExp(
            QRegExp(QString("^").append(dirname), 
            self.caseinsensitity, QRegExp.RegExp))
        self.position = dirname
        self.setWindowTitle(qApp.tr("Filesearch- Location:") + dirname)
        self.m_all_volume.setChecked(False)
        self.m_select_directory.setChecked(True)
            
#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_Chinese_triggered(self):
        fp = open(Contents.CONFIG_PATH,"wb")
        self.config["language"] = "zh_CN"
        pickle.dump(self.config,fp)
        fp.close()
        self.close()
        os.system(os.path.join(os.getcwd(), __file__))


#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_English_triggered(self):
        fp = open(Contents.CONFIG_PATH,"wb")
        self.config["language"] = "en"
        pickle.dump(data,fp)
        fp.close()
        self.close()
        os.system(os.path.join(os.getcwd(), __file__))

#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_help_triggered(self):
        QMessageBox.aboutQt(self, qApp.tr("help")) 

#-----------------------------------------------------------------------------
    @pyqtSignature("")
    def on_m_about_triggered(self):
        QMessageBox.about(self, qApp.tr("about filesearch"), 
            qApp.tr(
                '<p>Filesearch Version: 0.13.1</p>'
                'Created by TianWeishu, Chengxu, Huxiaoling<br />'
                'Copyright (c) 2012-12 All Rights Reserved'
                '<p>Contact us <a href="mailto:twsxtd@gmail.com">twsxtd@gmail.com</a></p>'))

    @pyqtSignature("")
    def on_m_style_triggered(self):
        dialog = QDialog(self)
        ui_style_label = QLabel(qApp.tr("UI Style"))
        ui_style_combobox = QComboBox()
        ui_style_combobox.addItems(["windowsVista", "windows", "motif", "cde", "plastique", "windowsxp", 
            "macintosh", "cleanlooks", "gtk"])
        ui_style_label.setBuddy(ui_style_combobox)
        ui_font = QLabel("UI Font")
        ui_font_button = QPushButton(qApp.tr("choose font"))
        ui_font.setBuddy(ui_font_button)
        buttonBox = QDialogButtonBox(QDialogButtonBox.Ok | QDialogButtonBox.Cancel)
        buttonBox.button(QDialogButtonBox.Ok).setDefault(True)
        layout = QGridLayout()
        layout.addWidget(ui_style_label, 0, 0)
        layout.addWidget(ui_style_combobox, 0, 1)
        layout.addWidget(ui_font, 1, 0)
        layout.addWidget(ui_font_button, 1, 1)
        layout.addWidget(buttonBox, 2, 0, 1, 2)
        def chooseFont():
            font, ok = QFontDialog.getFont()
            if ok:
                self.font = font
        buttonBox.connect(buttonBox, SIGNAL("accepted()"), dialog, SLOT("accept()"))
        buttonBox.connect(buttonBox, SIGNAL("rejected()"), dialog, SLOT("reject()"))
        ui_font_button.connect(ui_font_button, SIGNAL("clicked()"), chooseFont)
        dialog.setLayout(layout)
        if dialog.exec_():
            QApplication.setStyle(ui_style_combobox.currentText())
            QApplication.setFont(self.font)
#-----------------------------------------------------------------------------
# --------------------------menu action end-----------------------------------

    @pyqtSignature("")
    def on_stop_clicked(self):
        self.interrupt = True

# ----------------------------------------------------------------------------
    
    @pyqtSignature("QPoint")
    def on_fileinfo_customContextMenuRequested(self, point):
        if self.tablemenu:
            # self.tablemenu.exec_(self.fileinfo.mapToGlobal(point))
            self.tablemenu.exec_(QCursor.pos())
    @pyqtSignature("QPoint")
    def on_tree_customContextMenuRequested(self):
        print "1"

#-----------------------------------------------------------------------------
    def on_menu_open(self):
        index = self.fileinfo.currentIndex()
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            filename = self.typefilter.data(index, role).toString()
            dirname = self.typefilter.data(self.typefilter.index(row,column + 1), role).toString()
            path = dirname.append(QString(sep)).append(filename) 
        elif column == 1:
            path = self.typefilter.data(index, role).toString()
        else:
            pass
        openfile(_toUnicode(path))
#-----------------------------------------------------------------------------
    def on_mopendirectory(self):
        index = self.fileinfo.currentIndex()
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            path = self.typefilter.data(self.typefilter.index(row,column + 1), role).toString()
        elif column == 1:
            path = self.typefilter.data(index, role).toString()
        else:
            pass
        openfile(_toUnicode(path))

#-----------------------------------------------------------------------------
    def on_mcopyfilename(self):
        index = self.fileinfo.currentIndex()
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            path = self.typefilter.data(index, role).toString()
        elif column == 1:
            path = self.typefilter.data(self.typefilter.index(row,column - 1), role).toString()
        else:pass
        clipboard = QApplication.clipboard()
        clipboard.setText(path)
#-----------------------------------------------------------------------------
    def on_mcopydirectoryname(self):
        index = self.fileinfo.currentIndex()
        row, column = index.row(), index.column()
        role = Qt.DisplayRole
        if column == 0:
            path = self.typefilter.data(self.typefilter.index(row,column + 1), role).toString()
        elif column == 1:
            path = self.typefilter.data(index, role).toString()
        else:
            pass
        clipboard = QApplication.clipboard()
        clipboard.setText(path)

    def on_quit(self):
        self.Db.close()
        QSqlDatabase.database().close(); # 必须释放资源，非常重要的一步
        # Zcompress.compress(Contents.TEMP_PATH,Contents.DATA_PATH)
        # os.remove(Contents.TEMP_PATH)
        fp = open(Contents.CONFIG_PATH, "wb")
        # self.config["font"] = QApplication.font()
        # self.config["style"] = QApplication.style()
        pickle.dump(self.config, fp)
        fp.close()
        logging.info("closed")
    
#-----------------------------------------------------------------------------      
    def show_detail(self,filename):
        """显示文件的详细信息"""
        text = []
        text.append(u"<html><head/><body>")
        if os.path.isdir(filename):
            text.append(u"<p>Folder Name: " + filename + "</p>")
            text.append(u"<p>filename    size    type</p>")
            filelist = os.listdir(filename)
            for i in filelist:
                fullname = os.path.join(filename, i)
                if os.path.isdir(fullname):
                    text.append("<p>" + i + u'\t' + getsize(os.stat(fullname).st_size) + u'\t' + 'folder' + "</p>")
                else:
                    text.append("<p>" + i + u'\t' + getsize(os.stat(fullname).st_size) + u'\t' + 'file' + "</p>")
        else:
            text.append(u'<p style="color:#332b33;font-size:10;">File Name: ' + os.path.split(filename)[-1] + "</p>")
            text.append(u'<p style="color:#ffaa00;font-size:10;">Size: ' + getsize(os.stat(filename).st_size) + "</p>")
            text.append(u'<p style="color:#ff2b66;font-size:10;">Create time: ' + time.strftime("%Y/%m/%d %H:%M:%S", time.gmtime(os.stat(filename).st_ctime)) + "</p>")
            text.append(u'<p style="color:#66d5cc;font-size:10;">Last Access Time: ' + time.strftime("%Y/%m/%d %H:%M:%S", time.gmtime(os.stat(filename).st_atime)) + "</p>")
            text.append(u'<p style="color:#9980ff;font-size:10;">Stretch</p>')
            
            if os.path.splitext(filename)[-1] in Contents.PIC_TYPE:
                text.append(u'<p align="center"><img src="' + filename + '"/></p>')
            else:
                if os.path.getsize(filename) < 1024 * 1024:
                    try:
                        fp = open(filename)
                        text.append(fp.read())
                    except Exception, e:
                        logging.info(unicode(e))
                    finally:
                        fp.close()
                else:text.append(u'<p style="color:red;">' + filename + "is too big,please open it to look</p>")
        text.append(u'</body></html>')
        self.textdetail.setText(u'\n'.join(text))

#-----------------------------------------------------------------------------        
    def eventFilter(self, watch, event):
        if watch == self.searchedit:
            if (event.type() == QEvent.FocusIn):
                self.statusBar.showMessage('Press Enter to search')
                return True
        return False

#-----------------------------------------------------------------------------
    def normal_search(self):
        self.proxymodel.setFilterRegExp(
            QRegExp("", self.caseinsensitity, QRegExp.RegExp))
        self.infomodel.setQuery("SELECT * from catalog where file like '%%%s%%'" % self.searchedit.text())
        

#-----------------------------------------------------------------------------
    def search_by_whole(self):
        for i in range(30): # 首先取20行数据再说
            if self.infomodel.canFetchMore():self.infomodel.fetchMore()
            else:break
        self.proxymodel.setFilterRegExp(
            QRegExp(QString("^").append(self.searchedit.text()).append(QString("$")), 
            self.caseinsensitity, QRegExp.RegExp))
        self.progressBar.setValue(self.infomodel.rowCount())
        if self.interrupt:
            self.interrupt = False
            return
        QCoreApplication.processEvents()
        if self.typefilter.rowCount() < 14 and self.infomodel.canFetchMore():
            self.search_by_wildcard()
        else:
            self.progressBar.setValue(self.total)
            return
#-----------------------------------------------------------------------------
    def search_by_wildcard(self):
        for i in range(30): # 首先取20行数据再说
            if self.infomodel.canFetchMore():self.infomodel.fetchMore()
            else:break
        self.proxymodel.setFilterRegExp(QRegExp(self.searchedit.text(), 
            self.caseinsensitity, QRegExp.Wildcard))
        print self.infomodel.rowCount(), self.typefilter.rowCount(),self.rowcount
        self.progressBar.setValue(self.infomodel.rowCount())
        if self.interrupt:
            self.interrupt = False
            return
        QCoreApplication.processEvents()
        # 如果显示数量不够，递归调用
        if self.typefilter.rowCount() < 14 and self.infomodel.canFetchMore():
            self.search_by_wildcard()
        else:
            self.progressBar.setValue(self.total)
            return
#-----------------------------------------------------------------------------
    def search_by_regex(self):
        for i in range(30):
            if self.infomodel.canFetchMore():self.infomodel.fetchMore()
            else:break
        # 进行过滤
        print self.infomodel.rowCount(),self.proxymodel.rowCount(), self.typefilter.rowCount()
        self.proxymodel.setFilterRegExp(QRegExp(self.searchedit.text(), 
            self.caseinsensitity, QRegExp.RegExp))
        if self.interrupt:
            self.interrupt = False
            return
        self.progressBar.setValue(self.infomodel.rowCount())
        QCoreApplication.processEvents()
        # 如果显示数量不够，递归调用
        if self.typefilter.rowCount() < 5 and self.infomodel.canFetchMore():
            self.search_by_regex()
        else:
            self.progressBar.setValue(self.total)
            return


    def filter_type(self, index):
        regstr = []
        if index == 0:
            pass
        elif index == 1:# document
            regstr = [i + u"$" for i in Contents.DOC_TYPE]
        elif index == 2:# music
            regstr = [i + u"$" for i in Contents.MUSIC_TYPE]
        elif index == 3:# picture
            regstr = [i + u"$" for i in Contents.PIC_TYPE]
        elif index == 4:# vedio
            regstr = [i + u"$" for i in Contents.MOVIE_TYPE]
        elif index == 5:# application
            regstr = [i + u"$" for i in Contents.APPLICATION_TYPE]
        else:
            # regstr = [u"/.*(?!\.%s$)/" % i[1:] for i in Contents.DOC_TYPE]
            regstr = [u".*(?!\.txt)$", u".*(?!\.txt)$"]
        self.typefilter.setFilterRegExp(QRegExp(u"|".join(regstr), Qt.CaseInsensitive, QRegExp.RegExp))

#-----------------------------------------------------------------------------
#-----------------------------------------------------------------------------
    def dragEnterEvent(self, event):
        if event.mimeData().hasFormat("application/x-draganddrop") or event.mimeData().hasFormat("text/uri-list"):
            event.accept()
            print event.mimeData().hasUrls()
        else:event.ignore()

    def dropEvent(self, event):
        print "droped"

    def update(self):
        Db = DAO.Db(Contents.TEMP_PATH)
        Db.con.execute("PRAGMA catche_size = 8000")
        Db.con.execute("PRAGMA synchronous = 0")
        self.searchedit.setVisible(False)
        self.progressBar.setVisible(True)
        self.progressBar.setValue(0)
        QCoreApplication.processEvents()
        drivers = [i for i in getDriverNames()]
        for i in drivers:
            usn_path = Contents.USNLOG_PATH + i[:-2]
            j = journal.Journal(i[:-1])
            j.load_state(usn_path)
            j.process()
            QCoreApplication.processEvents()
            changed_paths = j.get_changed_paths()
            self.progressBar.setRange(0, len(changed_paths))
            for num, j in enumerate(changed_paths):
                self.progressBar.setValue(num)
                QCoreApplication.processEvents()
                filepath = i[:-1] + j
                if filepath.startswith(os.environ["appdata"].replace("\\", "/")) \
                    or filepath.startswith(os.environ["localappdata"].replace("\\", "/")) \
                    or filepath.startswith(os.environ["temp"].replace("\\", "/")):
                    continue
                try:
                    parentdir, filename = os.path.split(filepath)
                    if not os.path.exists(filepath):
                        Db.con.execute("DELETE FROM catalog WHERE parentdir = '%s' AND file = '%s'" % (parentdir, filename))
                    else:
                        Db.con.execute("INSERT INTO catalog(parentdir, file) VALUES ('%s', '%s')" % (parentdir, filename))
                except:
                    continue
        Db.close()
        # self.progressBar.setValue(len(changed_paths))
        self.progressBar.setVisible(False)
        self.searchedit.setVisible(True)

    def open_database(self):
        self.Db = DAO.Db(Contents.TEMP_PATH)
        if not createConnection():
            sys.exit(1)

    def close_database(self):
        self.Db.close()
        QSqlDatabase.database().close()

if __name__ == '__main__':
    freeze_support() # 多线程打包的操作函数
    logging.basicConfig(filename = Contents.LOG_PATH.encode(Contents.LOCAL_ENCODING), level = logging.INFO)
    app = QApplication(sys.argv)
    
    try:
        fp = open(Contents.CONFIG_PATH, "rb")
        data = pickle.load(fp)
        trans = QTranslator() # 添加国际化的支持
        trans.load("./language/" + data.get("language",None))
        app.installTranslator(trans)
        # QApplication.setFont(data["font"])
        # QApplication.setStyle(data["style"])
    except Exception:
        logging.info("language load failed")
    # 启动界面
    splash=QSplashScreen(QPixmap(":/file/images/splash.jpg"))
    splash.show()
    splash.showMessage(qApp.tr("loading core module..."), Qt.AlignBottom, Qt.white)
    # if not (os.path.exists(Contents.DATA_PATH) and (Contents.USER_NAME == winutils.getUserName())):
    if not os.path.exists(Contents.TEMP_PATH):
        splash.showMessage(qApp.tr("update data..."), Qt.AlignBottom, Qt.white)
        fast_traverse.fast_update(IS_NTFS)
    # else:
    #     Zcompress.decompress(Contents.DATA_PATH,Contents.TEMP_PATH)

    splash.showMessage(qApp.tr("starting..."), Qt.AlignBottom, Qt.white)
    app.processEvents()
    filesearch = filesearch()
    filesearch.show()
    splash.finish(filesearch) 
    app.connect(app,SIGNAL('aboutToQuit()'),filesearch.on_quit) 
    # QApplication.setStyle("cleanlooks")
    # Plastique, WindowsXP, GTK
    # "windows", "motif", "cde", "plastique", "windowsxp", "macintosh", "cleanlooks", "gtk", "windowsVista"
    sys.exit(app.exec_())

# sqlite3是支持正则表达式的！
# 哈希到底是什么 速度如何
# model的过滤函数
# setRootIndex啥意思