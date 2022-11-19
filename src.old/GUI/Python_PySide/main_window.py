import sys
import Qt设计
from PySide6.QtWidgets import QMainWindow,QApplication

class MainWindow(QMainWindow):
    def __init__(self,parent = None):
        super(MainWindow, self).__init__(parent)
        self.ui = Qt设计.Ui_MainWindow()
        self.ui.setupUi(self)

def __main__():
    myapp = QApplication(sys.argv)
    showapp = MainWindow()
    showapp.show()
    sys.exit(myapp.exec())

if __name__ == '__main__':
    __main__()
