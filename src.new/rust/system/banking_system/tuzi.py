import cv2
import pyzbar.pyzbar as pyzbar
import os

def decodeDisplay(video,flag):
    gray = cv2.cvtColor(video, cv2.COLOR_BGR2GRAY)
    barcodes = pyzbar.decode(gray)
    for barcode in barcodes:
        (x, y, w, h) = barcode.rect
        cv2.rectangle(video, (x, y), (x + w, y + h), (0, 255, 0), 2)
        barcodeData = barcode.data.decode("utf-8")
        barcodeType = barcode.type
        text = "{}".format(barcodeData)
        cv2.putText(video, text, (x, y - 10), cv2.FONT_HERSHEY_SIMPLEX,1, (0, 255, 0), 2)
        print("[扫描结果]: {0}".format(barcodeData))
        flag = 27
        return flag
    cv2.imshow("QRCODE",video)

def detect():
    cap = cv2.VideoCapture(0)
    while(True):
        ret, frame = cap.read()
        flag = cv2.waitKey(1)
        flag = decodeDisplay(frame,flag)
        if flag == 27:
            break
    cap.release()
    cv2.destroyAllWindows()

if __name__ == '__main__':
    detect()
