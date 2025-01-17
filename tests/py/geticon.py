import os
import _winreg
def getDefaultIcon(filename):
    '''Retrieve the default icon of a filename'''
    (root, extension) = os.path.splitext(filename)
    if extension:
        value_name = _winreg.QueryValue(_winreg.HKEY_CLASSES_ROOT, extension) 
        try:
            value_name = _winreg.QueryValue(_winreg.HKEY_CLASSES_ROOT, extension) 
        except _winreg.error:
            value_name = None
    else:
        value_name = None
    if value_name:
        try:
            icon = _winreg.QueryValue(_winreg.HKEY_CLASSES_ROOT, value_name + "\\DefaultIcon") 
        except _winreg.error:
            icon = None 
    else: 
        icon = None
    return icon
if __name__ == "__main__": 
    print getDefaultIcon("icon.pdf")
    print getDefaultIcon("icon.mp3")
    print getDefaultIcon("icon.torrent")
    print getDefaultIcon("icon.doc")
    print getDefaultIcon("icon.chm")
    print getDefaultIcon("icon.dll")
