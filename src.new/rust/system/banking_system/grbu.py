import qrcode
user = input("user:")
money = input("money:")
img = qrcode.make(user+" "+money)
img.save("qrc.png")
from pathlib import Path
import win32print
import win32ui
from PIL import Image,ImageWin

printer_name = win32print.GetDefaultPrinter()

im = Image.open((Path(__file__).parent/"qrc.png").resolve()._str)
hDC = win32ui.CreateDC()
hDC.CreatePrinterDC(printer_name)
hDC.StartDoc(((Path(__file__).parent/"qrc.png").resolve()._str))
hDC.StartPage()
dib = ImageWin.Dib(im)
dib.draw(hDC.GetHandleOutput(), (0, 0, im.width, im.height))

hDC.EndPage()

hDC.EndDoc()

del hDC
