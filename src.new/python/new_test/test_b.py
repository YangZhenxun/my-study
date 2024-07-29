import qrcode
from PIL import Image
from IPython.display import display

qrcode.make("https://github.com/YangZhenxun/my-study").save("test_b.png", "png")
display(Image.open('test_b.png', 'r'))