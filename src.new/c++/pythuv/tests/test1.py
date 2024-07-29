import sys
sys.path.append("..")
from pythuv.pyd import *
print(uv_version_string())
loop = uv_default_loop()
uv_loop_init(loop)
print("Now quitting.")
uv_run(loop, UV_RUN_DEFAULT)
uv_loop_close(loop)
