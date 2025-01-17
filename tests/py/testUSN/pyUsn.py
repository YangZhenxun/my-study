import struct
import win32file
import winioctlcon

def open_volume(drive):
    volh = win32file.CreateFile('\\\\.\\' + drive, win32file.GENERIC_READ,
            win32file.FILE_SHARE_READ | win32file.FILE_SHARE_WRITE, None,
            win32file.OPEN_EXISTING, win32file.FILE_ATTRIBUTE_NORMAL, None)
    return volh

def close_volume(volh):
    win32file.CloseHandle(volh)

def query_journal(volh):
    fmt = 'QQQQQQQ'
    len = struct.calcsize(fmt)
    buf = win32file.DeviceIoControl(volh, winioctlcon.FSCTL_QUERY_USN_JOURNAL, None, len)
    tup = struct.unpack(fmt, buf)
    return tup

volh = open_volume('C:')
UsnJournalID, FirstUsn, NextUsn, LowestValidUsn, MaxUsn, MaximumSize, AllocationDelta = query_journal(volh)
close_volume(volh)
print 'Journal id is 0x%016x' % UsnJournalID
