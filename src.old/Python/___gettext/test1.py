import gettext
trans = gettext.translation('trans', 'locale', languages=['en'])
trans.install()
print(_("Hello!"))
sel = gettext.translation('trans', 'locale', languages=['zh_Hans'])
sel.install()
print(_("Hello!"))
