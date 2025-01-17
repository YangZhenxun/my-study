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

"""This program outputs every changed file in the file system.  The first time it is run, it will print every
currently existing file.  It will then print every file that has been changed since the prior run.  The output
is lossy in the sense that it may print additional paths (for instance, if the journal id is different it has to
assume that every existing path has been changed).

Reference material:

http://msdn.microsoft.com/en-us/library/aa363798%28v=VS.85%29.aspx
http://www.microsoft.com/msj/0999/journal/journal.aspx
http://www.microsoft.com/msj/1099/journal2/journal2.aspx
"""

import sys
import os
import cPickle

from journaltool import *


def default_notifier(msg):
    pass

def print_notifier(msg):
    print msg
    
def normalise(path):
    """Return a normalised path: lowercase, with forward slashes, starting at / (i.e. no drive)."""
    path = os.path.normcase(path)
    path = path.replace('\\', '/')
    path = path.replace('//', '/')    
    if len(path) >= 2 and path[1] == ':':# remove the driver name
        path = path[2:]
    return path


def get_ancestors(path):
    """Return a list of the ancestor directories of this file."""
    ancestors = []
    components = normalise(path).split('/')
    subpath = '/'
    for c in components:
        if subpath == '':
            subpath = c
        else:
            subpath = subpath + '/' + c
        subpath = normalise(subpath)
        ancestors.append(subpath)
    return ancestors
 

class FrnMap(object):
    """A map from FRNs to parent FRNs and names.  This is enough information to
    translate a FRN to a path (as done in build_path)."""
    
    def __init__(self):
        self.map = {}

    def load(self, filename):
        f = open(filename, 'rb')
        self.map.update(cPickle.load(f))
        f.close()

    def save(self, filename):
        f = open(filename, 'wb')
        cPickle.dump(self.map, f)
        f.close()

    def set(self, frn, parent_frn, name):
        self.map[frn] = parent_frn, name

    def build_path(self, frn):
        '''这是一个映射的结构，由本文件的frm->父文件的frm，本文件名
        通过递归调用，得到文件的路径
        '''
        if frn not in self.map:
            return ''
        parent_frn, name = self.map[frn]
        return self.build_path(parent_frn) + '/' + name


class Journal(object):
    
    def __init__(self, drive):
        self.drive = drive
        self.set_state((None, None, {}))
        self.data = {}
    def process_usn(self, tup, fn):
        '''tup is a USN_RECORD struct
        '''
        if tup[10] & win32file.FILE_ATTRIBUTE_DIRECTORY:# if a directory tup[10]:FileAttributes
            self.frn_to_dir_map.set(tup[3], tup[4], fn)# tup[3]:FileReferenceNumber,tup[4]:ParentFileReferenceNumber
        
        parent_frn = tup[4]
        parent_path = self.frn_to_dir_map.build_path(parent_frn)
        try:
            path = parent_path + '/' + fn
        except UnicodeEncodeError, ex:
            print >>sys.stderr, "Error outputting file name:", ex
            return
        for p in get_ancestors(normalise(path))[:-1]:
            if p not in self.affected_dirs:
                self.affected_dirs.add(p)
        self.changed_paths.add(normalise(path))
        self.data[parent_path] = fn
        
    def get_state(self):
        return self.journal_id, self.last_usn, self.frn_to_dir_map.map

    def set_state(self, state):
        '''set the variance of the class,state is a tuple with s=three member
        the first is ...
        second is ...
        last is a map and store the info of the frn in the file system'''
        self.journal_id = state[0]
        self.last_usn = state[1]
        self.frn_to_dir_map = FrnMap()
        self.frn_to_dir_map.map.update(state[2])
    
    def load_state(self, filename):
        '''use the persistence file initialize the class'''
        f = open(filename, 'rb')
        obj = cPickle.load(f)
        f.close()
        self.set_state(obj)
        
    def save_state(self, filename):
        '''save the data '''
        obj = self.get_state()
        f = open(filename, 'wb')
        cPickle.dump(obj, f)
        f.close()

    def get_changed_paths(self):
        return self.changed_paths

    def process(self, notifier=default_notifier):
        notifier('Opening volume %s' % self.drive)
        volh = open_volume(self.drive)# get the handls to the volume
        
        notifier('Querying journal')
        try:
            tup = query_journal(volh)# query the journal,tup is a USN_JOURNAL_DATA struct
        except pywintypes.error, ex:
            if ex.winerror == 1179:   # ERROR_JOURNAL_NOT_ACTIVE
                notifier('Creating new journal')
                create_journal(volh)# create a new journal beceuse the journal not exist before
                notifier('Re-querying')
                tup = query_journal(volh)
            else:
                raise
        queried_journal_id = tup[0]     # the current journal identifer
        first_usn = tup[1]  # the number of the first record that can be read from journal 
        next_usn = tup[2]   # next usn
        self.replay_all = False
        
        if self.journal_id != queried_journal_id or first_usn > self.last_usn:
            # if the self.journal_id(that wo stored which is the journal_id wo last time used) is not equal to the queried journal_id,
            # the the journal may be delete or recreated ;if first_usn wo get this time is gigger than the last_usn wo last time get,
            # then the journal may be out of space and delete the first usn
            # in this case,wo must replay the journal!
            if self.journal_id is None:# if journal_id is None,it is we create just now
                notifier('Journal is new (available id 0x%016x, first available USN 0x%016x)' % (queried_journal_id, first_usn))
            else:# it is out of date
                notifier('Journal is too new (recorded id 0x%016x, available id 0x%016x, last recorded USN 0x%016x, first available USN 0x%016x)' % (self.journal_id, queried_journal_id, self.last_usn, first_usn))
            self.journal_id = queried_journal_id # update the journal id
            self.last_usn = first_usn # update
            self.replay_all = True # we must replay 
        
        self.changed_paths = set()
        self.affected_dirs = set()

        if self.replay_all: # replay the journal
            tup = get_ntfs_volume_data(volh) # tup is a NTFS_VOLUME_DATA_BUFFER struct
            mft_entry_size = tup[7] # BytesPerFileRecordSegment
            mft_max = tup[9] / mft_entry_size # MftValidDataLength/BytesPerFileRecordSegment
            
            notifier('Reading all USNs from MFT')
            last_pct = 0
            for next_frn,tup,fn in generate_usns(volh, 0, next_usn):
                # next_frn
                # tup is a struct of USN_RECORD
                self.process_usn(tup, fn)
                if tup[5] > self.last_usn: # aprent's frn
                    self.last_usn = tup[5]
                    
                mft_pos = next_frn & 0xFFFFFFFFFFFF
                pct = 100 * mft_pos / mft_max
                if pct > last_pct and 0 <= pct <= 100:
                    notifier('Read MFT pos %d; %d percent done' % (mft_pos, pct))
                    last_pct = pct
            
            notifier('Re-querying journal')
            tup = query_journal(volh)
            next_usn = tup[2]
        
        start_usn = self.last_usn # update the data from last time we do
        notifier('Replaying journal from USN 0x%016x to 0x%016x' % (start_usn, next_usn))
        last_pct = 0
        for tup,fn in generate_journal(volh, self.journal_id, start_usn):
            if self.replay_all or self.last_usn < tup[5]:
                self.process_usn(tup, fn)
                self.last_usn = tup[5]
            pct = 100 * (tup[5] - start_usn) / (next_usn - start_usn)
            if pct > last_pct:
                notifier('Replayed USN 0x%016x; %d percent done' % (tup[5], pct))
                last_pct = pct
        
        notifier('Closing volume')
        close_volume(volh)
    
    def affected(self, path):
        """Could this path possibly have changed according to the journal?"""
        
        path = normalise(path)
        
        if path in self.affected_dirs:
            return True
        
        for p in get_ancestors(path):
            if p in self.changed_paths:
                return True
        
        return False


def main(argv=None):
    if argv is None:
        argv = sys.argv
    drive = argv[1]
    journal_filename = argv[2]
    try:
        target_dir = argv[3]
    except IndexError:
        target_dir = os.getcwd()
    
    print 'Opening journal'
    j = Journal(drive)
    try:
        j.load_state(journal_filename)
    except Exception:
        pass
        
    print 'Processing'
    j.process(notifier=print_notifier)
    
    print 'Changed paths:',len(j.get_changed_paths())
    for p in sorted(j.get_changed_paths()):
        try:
            print p
        except UnicodeEncodeError:
            print repr(p),"error"

    print 'Affected paths:'
    for dirpath, dirnames, filenames in os.walk(target_dir):
        for fn in filenames + dirnames:
            path = normalise(os.path.join(dirpath, fn))
            if j.affected(path):
                print path
    
    j.save_state(journal_filename)


if __name__ == '__main__':
    main(argv = (None,"e:","log.log","e:\\"))
