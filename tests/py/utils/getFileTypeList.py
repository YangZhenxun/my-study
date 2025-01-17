#! /usr/bin/env python
# -*- coding: utf-8 -*- 

# Copyright (c) 2012-12 
# authored by TianWeishu, HuXiaoling, ChengXu.
# All rights reserved.
# This program or module is free software: you can redistribute it and/or
# modify it under the terms of the GNU General Public License as published
# by the Free Software Foundation, either version 2 of the License, or
# version 3 of the License, or (at your option) any later version. It is
# provided for educational purposes and is distributed in the hope that
# it will be useful, but WITHOUT ANY WARRANTY; without even the implied
# warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
# the GNU General Public License for more details.ept:

import mimetypes
mimetypes.common_types.update(mimetypes.types_map)

class mimeType(object):
    """docstring for mime"""
    def __init__(self,):
        super(mimeType, self).__init__()
        self.knowntype = mimetypes.common_types
        self.set_state()

    def set_state(self):
        self.data = {}
        self.data["text"] = self._get_file_typelist("text")
        self.data["music"] = self._get_file_typelist("audio")
        self.data["vedio"] = self._get_file_typelist("video")
        self.data["image"] = self._get_file_typelist("image")

    def get_text(self):
        return self.data["text"]

    def get_music(self):
        return self.data["music"]

    def get_vedio(self):
        return self.data["vedio"]

    def get_image(self):
        return self.data["image"]

    def _get_file_typelist(self, filetype):
        result = []
        for key, value in self.knowntype.items():
            if value.startswith(filetype):
                result.append(unicode(key))
        return result

if __name__ == '__main__':
    t = mimeType()
    print t.get_music()
    print t.get_text()