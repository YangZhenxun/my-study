from pythuv import *
import sys

counter: int = 0

def wait_for_a_while(handle: uv_idle_t):
    counter += 1

    if (counter >= 10000e9):
        uv_idle_stop(handle)


def main():
    print(uv_version_string())
    loop = uv_default_loop()
    idler = uv_idle_t()
    print(idler)

    uv_idle_init(loop, idler)
    uv_idle_start(idler, wait_for_a_while)

    print("Idling...")
    uv_run(loop, UV_RUN_DEFAULT)

    uv_loop_close(loop)

main()
