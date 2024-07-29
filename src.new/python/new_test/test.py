import re
_is_legal_header_name = re.compile(rb'[^:\s][^:\r\n]*').fullmatch
print(_is_legal_header_name, re.fullmatch)
print({"hello"})
print(type({'hello'}))
hello_dict = {"hello"}
class HTTPException(Exception):
    pass
class LineTooLong(HTTPException):
    def __init__(self, line_type):
        HTTPException.__init__(self, "got more than %d bytes when reading %s"
                               % (35536, line_type))

raise LineTooLong("header line")