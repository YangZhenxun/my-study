from googletrans import Translator

trans = Translator(service_urls=[
    'translate.google.cn'
    ])
text = input('Please Enter Translate Text:')
tr = trans.translate(text)
print(tr.text)
