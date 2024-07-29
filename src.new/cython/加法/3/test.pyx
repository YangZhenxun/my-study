# cython: language_level=3
#distutils: language = c++
from ctestrand cimport testrandom

cdef int c_main():
    cdef int td = testrandom()
    return td

def main():
    c_main()
