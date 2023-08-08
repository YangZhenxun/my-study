import sys
import os
from PySide6.QtWidgets import QApplication, QMainWindow, QFileDialog, QMessageBox
from PySide6.QtCore import Slot, QObject, Signal, QThread
import get_big_file_ui



class AThread(QObject):
    signal_str = Signal(str)
    complete = Signal()
    progress = Signal()

    def get_big_file(self, path, filesize):
        for dirpath, dirnames, filenames in os.walk(path):
            for filename in filenames:
                target_file = os.path.join(dirpath, filename)
                if not os.path.isfile(target_file):
                    continue
                self.progress.emit()
                size = os.path.getsize(target_file)
                size = size//1024
                if size >= filesize:
                    size = '{size}KB'.format(size=size)
                    self.signal_str.emit(target_file)
        self.complete.emit()


class MainWindow(QMainWindow):
    a = Signal(str, int)

    def __init__(self):
        super().__init__()
        self.ui = get_big_file_ui.Ui_MainWindow()
        self.ui.setupUi(self)
        self.work_athread = QThread()
        self.a_thread = AThread()
        self.a_thread.complete.connect(self.complete)
        self.a_thread.signal_str.connect(self.add_item)
        self.a_thread.progress.connect(self.update_progress)
        self.a.connect(self.a_thread.get_big_file)
        self.a_thread.moveToThread(self.work_athread)
        self.Catalog = ""

    def add_item(self, text):
        self.ui.listWidget.addItem(str(text))

    def complete(self):
        self.ui.pushButton.setEnabled(True)
        self.ui.progressBar.setRange(0, 100)
        self.ui.progressBar.setValue(100)
        self.work_athread.quit()
        self.work_athread.wait()

    def to_work_athread(self):
        self.ui.pushButton.setEnabled(False)
        self.ui.listWidget.clear()
        self.ui.progressBar.setRange(0, 0)
        if self.Catalog == "":
            QMessageBox.critical(self, "Error", "Please select a catalog!")
        self.a.emit(self.Catalog, int(eval(self.ui.lineEdit.text())))
        self.work_athread.start()

    def update_progress(self):
        self.ui.progressBar.setValue(int(self.ui.progressBar.value() + 1))

    def select_catalog(self):
        self.Catalog = QFileDialog.getExistingDirectory(self, "Select Catalog")


app = QApplication(sys.argv)
window = MainWindow()
window.show()
sys.exit(app.exec())
