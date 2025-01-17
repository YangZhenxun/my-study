from __future__ import division
import mimetypes

def getsize(size):
    if size / 1024 < 1:
        return u"%.2fB" % size
    else:
        size = size / 1024
        if size / 1024 < 1:
            return u"%.2fK" % size
        else:
            size = size / 1024
            if size / 1024 < 1:
                return u"%.2fM" % size
            else:
                size = size / 1024
                if size / 1024 < 1:
                    return u"%.2fG" % size

if __name__ == '__main__':
    print getsize(12342323213).encode("gbk")