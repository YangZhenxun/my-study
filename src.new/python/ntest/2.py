from pathlib import Path
import re

target_in = Path(r"C:\Users\86139\vcpkg\installed\x64-windows\include\uv\errno.h")
target_out = Path(r"D:\my-study--github\src.new\python\pysuv\src\errno.py")
loc = False
with target_out.open("w", encoding="utf-8") as o_f:
    with target_in.open("r", encoding="utf-8") as i_f:
        for line in i_f.readlines():
            x = re.match("#( ?)define( +)(UV__.+)( +)\\((-\\d+)\\)", line, re.I)
            print(line)
            print(x)
            if x:
                if x.group(5):
                    k = x.group(3)+": int = "+ x.group(5) + "\n"
                    o_f.write(k)
