import cv2
from pyzbar import pyzbar
import csv
found = set()
capture = cv2.VideoCapture(0)
PATH = "test_b.png"
while(1):
    ret,frame = capture.read()
    test = pyzbar.decode(frame, symbols=[pyzbar.ZBarSymbol.QRCODE])
    _testdat = None
    for tests in test:
        testdate = tests.data.decode('utf-8')
        if _testdat != None:
            if _testdat != testdate:
                print("\n")
        print("\r"+testdate, end="")
        if testdate not in found:
            with open(PATH,'a+') as f:
                csv_write = csv.writer(f)
                date = [testdate]
                csv_write.writerow(date)
            found.add(testdate)
        _testdat = testdate
    cv2.imshow('Test',frame)
    if cv2.waitKey(1) == ord('q'):
        break