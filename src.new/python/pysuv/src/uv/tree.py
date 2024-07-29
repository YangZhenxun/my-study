import typing
from typing import TypeVar

def SPLAY_HEAD(name_a, type_a):
    class name:
        type type_b = type_a
        sph_root = TypeVar("sph_root", bound=type_b)
    TypeVar(name_a, bound=name)

def SPLAY_INITIALIZER(root):
    return {None}

def SPLAY_INIT(root):
    root.sph_root = None
    while 0:
        root.sph_root = None

def SPLAY_ENTRY(type_a):
    class none:
        type type_b = type_a
        spe_left = TypeVar("spe_left", bound=type_b)
        spe_right = TypeVar("spe_right", bound=type_b)

def SPLAY_LEFT(elm, field):
    l = dir(elm)
    for i in l:
        if field == i:
            eval("return elm."+i+".spe_left")

def SPLAY_RIGHT(elm, field):
    l = dir(elm)
    for i in l:
        if field == i:
            eval("return elm."+i+".spe_right")


