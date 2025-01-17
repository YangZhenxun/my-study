import qrcode
qr = qrcode.make("hello zbar-rs!")
qr.save("he.qrc")
