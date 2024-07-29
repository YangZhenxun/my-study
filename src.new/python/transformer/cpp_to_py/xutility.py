import climits
import cstdlib
import cstring
from enum import Enum, auto
class _TrimResult (Enum):
    _KeepTrimming=auto(),
    _HaveWorkAfterTrimming=auto(),
    _ReturnFalse=auto(),
    _ReturnTrue=auto()
