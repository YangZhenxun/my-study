from pathlib import Path
from types import ModuleType
from typing import TypeVar
from collections.abc import Callable
import re
import pathlib

p = Path("C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.38.33130\\include")

selfme = Path(__file__).parent
test = selfme/"a.cpp"
def test_a(test):
    with open(test, "r") as f:
        lines = f.readlines()
        l2 = []
        for line in lines:
            l2.append(line.rstrip())
        try_compile(l2, test)
def try_compile(l2, p2):
    do(l2, p2)

def do(l2, p2):
    with open(selfme/("cpp_to_py/"+(p2.stem)+".py"), "w") as filea:
        i = 0
        for l in l2:
            if m := re.match("#include <(.*)>", l):
                for f in p.iterdir():
                    if f.stem == m.group(1) or f == m.group(1):
                        test_a(f)
                        filea.write("import " + f.stem+"\n")
            if m := re.match("#include \"(.*)\"", l):
                for f in p2.iterdir():
                    if f.is_dir() and (f/m.group(1)).exists():
                        test_a(f/m.group(1))
                        filea.write("import " + f.name+"."+m.group(1)+"\n")
                    if f.is_file() and (f.stem == m.group(1) or f == m.group(1)):
                        test_a(f)
                        filea.write("import " + f.stem)
            if m := re.match("typedef (.*) (.*);", l):
                filea.write("from typing import TypeVar\n")
                filea.write(m.group(2)+"=TypeVar("+m.group(2)+", bound="+m.group(1)+")\n")
            if m := re.match("enum class (.*):(.*)", l) or re.match("enum struct (.*):(.*)", l) or re.match("enum (.*):(.*)", l):
                filea.write("from enum import Enum\n")
                filea.write("class "+m.group(1)+"(Enum):\n")
                if m.group(2) != " " and m.group(2) is not None:
                    do_a(l, filea)
                    do_b(l2[i+1],filea, l2, i+1)
                else:
                    do_b(l2[i+1], filea, l2, i+1)
            i+=1

def do_a(l, filea):
    m2 = re.match(".* {(.*)", l)
    if m2 and m2.group(1) is not None and m2.group(1) != " ":
        if m2.group(1).endswith("};"):
            m3 = m2.group(1).rstrip("};")
        else:
            m3 = m2.group(1)
        filea.write(m3+"\n")

def do_b(l, filea, l2, i):
    m3 = re.match("(.*),(.*)", l)
    if m3 and m3.group(1) != " " and m3.group(1) is not None:
        if m3.group(2).endswith("};"):
            m4 = m3.group(1) + "," +m3.group(2).rstrip("};")
        else:
            m4 = m3.group(0)
        filea.write(m4+"\n")
        do_b(l2[i+1], filea, l2, i+1)

test_a(test)
