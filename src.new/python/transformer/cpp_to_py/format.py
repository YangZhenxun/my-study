import bit
import charconv
import concepts
import cstdint
import iterator
import limits
import limits
import locale
import stdexcept
import xstring
import xutility
from enum import Enum
class _Fmt_align (Enum):
 _None, _Left, _Right, _Center
from enum import Enum
class _Fmt_sign (Enum):
 _None, _Plus, _Minus, _Space
from enum import Enum
class _Basic_format_arg_type (Enum):
    _None,
    _Int_type,
    _UInt_type,
    _Long_long_type,
    _ULong_long_type,
    _Bool_type,
    _Char_type,
    _Float_type,
    _Double_type,
    _Long_double_type,
    _Pointer_type,
    _CString_type,
    _String_type,
    _Custom_type,
from enum import Enum
class _Add_newline (Enum):
 _Nope, _Yes
