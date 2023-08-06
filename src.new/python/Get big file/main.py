import sys
import os
import time
from PySide6.QtWidgets import QApplication, QMainWindow,QMessageBox, QListWidgetItem
from PySide6.QtCore import Slot, QObject, Signal, QThread
from PySide6.QtGui import QIcon, QPixmap
from get_big_file_ui import Ui_MainWindow



class AThread(QObject):
    signal_str = Signal(str)

    def __init__(self):
        super().__init__()

    def get_big_file(self, path, filesize):
        print("ok")
        for dirpath, dirnames, filenames in os.walk(path):
            QApplication.processEvents()
            for filename in filenames:
                QApplication.processEvents()
                target_file = os.path.join(dirpath, filename)
                if not os.path.isfile(target_file):
                    continue
                size = os.path.getsize(target_file)
                size = size//1024
                if size > filesize:
                    size = '{size}KB'.format(size=size)
                    self.signal_str.emit(str(target_file))
                    print(target_file)
                    QApplication.processEvents()


class MainWindow(QMainWindow):
    a = Signal(str)
    b = Signal(int)
    def __init__(self):
        super().__init__()
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)
        self.a_thread = AThread()
        self.work_athread = QThread()
        self.a.connect(self.a_thread)
        self.b.connect(self.a_thread)
        self.a_thread.moveToThread(self.work_athread)
        self.a_thread.signal_str.connect(self.add_item)

    def start(self):
        self.ui.pushButton.setEnabled(False)



    def add_item(self, text):
        self.ui.listWidget.addItems(text)
        QApplication.processEvents()

    def shows(self) -> None:
        self.main_thread.start()

    def show_a(self):
        self.a_thread.get_big_file(self.path, self.filesize)


if __name__ == "__main__":
    try:
        app = QApplication(sys.argv)
        window = MainWindow()
        QApplication.processEvents()
        window.show()
        sys.exit(app.exec())
    except Exception as e:
        QMessageBox.critical(MainWindow(), "Error!", "%s" % e)
