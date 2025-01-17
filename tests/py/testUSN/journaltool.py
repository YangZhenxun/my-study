# -*- coding:utf-8
'''the tools to deal with the USN Journal,here are some useful informations
===============================================================
this is the key method:

str = DeviceIoControl(Devive,IoControl,Inbuffer,OutBuffer
    OverLapped)
sends a control code to a device or file system driver
the Parms:
PyHANDLE Deveice :a handle to device
int IoControl:IoControl code to use from winioctlcon
str/buffer Inbuffer:the input data for the operation
int/buffer OutBuffer:size of the buffer to allocate for output
PyOVERLAPPED OverLapped = None:
Accepts keyword args
return output data 

=============================================================='''
import struct
import win32file
import winioctlcon
import win32api
import winerror
import pywintypes

USN_BUFFER_SIZE = 4096
JOURNAL_MAX_SIZE = 16*1048576
JOURNAL_ALLOCATION_DELTA = 65536
   
def open_volume(drive):
    '''get a handle to the driver'''
    volh = win32file.CreateFile('\\\\.\\' + drive, win32file.GENERIC_READ,
            win32file.FILE_SHARE_READ | win32file.FILE_SHARE_WRITE, None, 
            win32file.OPEN_EXISTING, win32file.FILE_ATTRIBUTE_NORMAL, None)
    return volh

def close_volume(volh):
    '''close the handle'''
    win32file.CloseHandle(volh)

def create_journal(volh):
    '''create a USN Journal file,it return none,use FSCTL_CREATE_USN_JOURNAL control code'''
    inp = struct.pack('QQ', JOURNAL_MAX_SIZE, JOURNAL_ALLOCATION_DELTA)
    win32file.DeviceIoControl(volh, winioctlcon.FSCTL_CREATE_USN_JOURNAL, inp, None)

def query_journal(volh):
    '''query the Journal file and use the FSCTL_QUERY_USN_JOURNAL control code 
    if query succeed,then return a struct USN_JOURNAL_DATA:
        
        typedef struct{
        DWORDLONG UsnJournalID;//the current journal identifer
        USN FirstUsn;//the number of first record that can be read from journal
        USN NextUsn;
        USN LowestValidUsn;//
        USN MaxUsn;if NextUsn approaches this value,must delete
        DWORDLONG maximumSize;
        DWORDLONG AllocationDelta;//the number of bytes of disk memory added
                                  //to the end and removed from beginning of the 
                                  //changed journal each time memory is allocated
        }USN_JOURNAL_DATA_V0,*PUSH_JOURNAL_DATA_V0,USN_JOURNAL_DATA,*PUSH_JOURNAL_DATA;'''
    fmt = 'QQQQQQQ'
    len = struct.calcsize(fmt)
    buf = win32file.DeviceIoControl(volh, winioctlcon.FSCTL_QUERY_USN_JOURNAL, None, len)
    tup = struct.unpack(fmt, buf)
    return tup

def get_ntfs_volume_data(volh):
    '''get information about the NTFS volumn,use the FSCTL_GET_NTFS_VOLUME_DATAcontrol code
    if succed return a NTFS_VOLUMN_DATA_BUFFER struct

        typedef struct {
          LARGE_INTEGER VolumeSerialNumber;//the serial number of the volume
          LARGE_INTEGER NumberSectors;//The number of sectors in the specified volume.
          LARGE_INTEGER TotalClusters;//The number of used and free clusters
          LARGE_INTEGER FreeClusters;
          LARGE_INTEGER TotalReserved;
          DWORD         BytesPerSector;
          DWORD         BytesPerCluster;
          DWORD         BytesPerFileRecordSegment;
          DWORD         ClustersPerFileRecordSegment;
          LARGE_INTEGER MftValidDataLength;
          LARGE_INTEGER MftStartLcn;
          LARGE_INTEGER Mft2StartLcn;
          LARGE_INTEGER MftZoneStart;//The starting logical cluster number of the master file table zone.
          LARGE_INTEGER MftZoneEnd;
        } NTFS_VOLUME_DATA_BUFFER, *PNTFS_VOLUME_DATA_BUFFER;'''
    fmt = 'qqqqqLLLLqqqqq'
    len = struct.calcsize(fmt)
    buf = win32file.DeviceIoControl(volh, winioctlcon.FSCTL_GET_NTFS_VOLUME_DATA, None, len)
    tup = struct.unpack(fmt, buf)
    return tup

def get_volume_info(drive):
    '''get the information a the driver
    if succeed return a tuple:
        string-Volume name
        long-serial number
        long-Maxmum component length of a file name
        long-sys flag
        string-file system name,say NTFS,FAT32'''
    return win32api.GetVolumeInformation('\\\\.\\' + drive + '\\')

def decode_usn_data(buf):
    '''decode the Usn data,a struct USN_RECORD_V2'''
    outfmt = 'LHHQQQQLLLLHH'
    outlen = struct.calcsize(outfmt)
    head_usn = struct.unpack('Q', buf[:8])[0]
    buf = buf[8:]
    tups = []
    while len(buf) > 0:
        tup = struct.unpack(outfmt, buf[:outlen])
        recordlen = tup[0]# record length
        filenamelen = tup[11]# filename length
        filenameoffset = tup[12]
        name1 = buf[filenameoffset:filenameoffset+filenamelen]
        name = name1.decode('UTF-16', 'replace')
        tups.append((tup, name))
        buf = buf[recordlen:]
    return head_usn, tups
# the changes we want,used to filter the read journal data 
ALL_INTERESTING_CHANGES = (winioctlcon.USN_REASON_BASIC_INFO_CHANGE | winioctlcon.USN_REASON_CLOSE
        | winioctlcon.USN_REASON_DATA_EXTEND | winioctlcon.USN_REASON_DATA_OVERWRITE | winioctlcon.USN_REASON_DATA_TRUNCATION
        | winioctlcon.USN_REASON_FILE_CREATE | winioctlcon.USN_REASON_FILE_DELETE
        | winioctlcon.USN_REASON_RENAME_NEW_NAME | winioctlcon.USN_REASON_RENAME_OLD_NAME)

def read_journal(volh, journal_id, first_usn):
    '''read a series  usn data from journal,how much of it due to the buffer 
    input args:
        volh:device handle
        journal_id:the UsnJournalID
        first_usn:the first usn you want to begin read
    these three args are used to make a READ_USN_JOURNAL_DATA_V0 struct like this:

        typedef struct {
            USN       StartUsn;
            DWORD     ReasonMask;
            DWORD     ReturnOnlyOnClose;
            DWORDLONG Timeout;
            DWORDLONG BytesToWaitFor;
            DWORDLONG UsnJournalID;
        } READ_USN_JOURNAL_DATA_V0, *PREAD_USN_JOURNAL_DATA_V0, READ_USN_JOURNAL_DATA, *PREAD_USN_JOURNAL_DATA;

    return:
        a struct named USN_RECORD ,which contains the information of the USN record
        and it is maintained by NTFS system itself

        typedef struct {
            DWORD         RecordLength;
            WORD          MajorVersion;
            WORD          MinorVersion;
            DWORDLONG     FileReferenceNumber;// the file's FRN,which is important
            DWORDLONG     ParentFileReferenceNumber;// the parent's FRN
            USN           Usn;
            LARGE_INTEGER TimeStamp;
            DWORD         Reason;
            DWORD         SourceInfo;
            DWORD         SecurityId;
            DWORD         FileAttributes;
            WORD          FileNameLength;
            WORD          FileNameOffset;
            WCHAR         FileName[1];//The name of the file or directory associated with
                                      // this record in Unicode format. This file or directory name is of variable length.
        } USN_RECORD_V2, *PUSN_RECORD_V2, USN_RECORD, *PUSN_RECORD;'''
    reason_mask = ALL_INTERESTING_CHANGES
    inp = struct.pack('QLLQQQ', first_usn, reason_mask, 0, 0, 0, journal_id)
    #input buffer is a READ_USN_JOURNAL_DATA_V0
    buf = win32file.DeviceIoControl(volh, winioctlcon.FSCTL_READ_USN_JOURNAL, inp, USN_BUFFER_SIZE)
    return decode_usn_data(buf)

def enum_usn_data(volh, first_frn, low_usn, high_usn):
    '''ENUM the usn data of range
    input args:
        volh:device handle
        first_frn:the ordinal position within the files on the current volume at which 
                the enumeration is to begin
        low_usn:the lower boundary of the range of usn values used to filter the records returned
        high_usn:the higher...
    these three args are to create a MFT_ENUM_USN_DATA struct:

        typedef struct {
            DWORDLONG StartFileReferenceNumber;
            USN       LowUsn;
            USN       HighUsn;
        } MFT_ENUM_DATA_V0, *PMFT_ENUM_DATA_V0, MFT_ENUM_DATA, *PMFT_ENUM_DATA;
    return:
        USN_RECORD_V2,see above'''
    inp = struct.pack('QQQ', first_frn, low_usn, high_usn)
    try:
        buf = win32file.DeviceIoControl(volh, winioctlcon.FSCTL_ENUM_USN_DATA, inp, USN_BUFFER_SIZE)
    except pywintypes.error, ex:
        if ex.args[0] == winerror.ERROR_HANDLE_EOF:
            return None, []
        raise
    head_usn = struct.unpack('Q', buf[:8])[0]
    return decode_usn_data(buf)

def generate_journal(volh, journal_id, first_usn):
    '''travese the journal to get the data,call the read_journal method'''
    while True:
        try:
            first_usn, tups = read_journal(volh, journal_id, first_usn)
        except pywintypes.error, ex:
            if ex.winerror == 1181:   # ERROR_JOURNAL_ENTRY_DELETED
                break
            raise
        if len(tups) == 0:
            break
        for t,n in tups:
            yield t,n

def generate_usns(volh, low_usn, high_usn):
    '''travese the journal to get the USN_RECORD,call the enum_usn_data method '''
    frn_pos = 0
    while True:
        next_frn, tups = enum_usn_data(volh, frn_pos, low_usn, high_usn)
        if len(tups) == 0:
            break
        for t,n in tups:
            yield frn_pos,t,n
        frn_pos = next_frn
