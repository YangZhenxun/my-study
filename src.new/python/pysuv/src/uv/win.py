import sys
from typing import TypeVar

if not (globals().get("_WIN32_WINNT")):
    _WIN32_WINNT = 0x0600

if (not globals().get("_SSIZE_T_")) and (not globals().get("_SSIZE_T_DEFINED")):
    ssize_t = TypeVar("ssize_t", bound=int)
    SSIZE_MAX = sys.maxsize
    _SSIZE_T_ = True
    _SSIZE_T_DEFINED = True

if (not globals().get("LOCALE_INVARIANT")):
    LOCALE_INVARIANT = 0x007f

import socket
