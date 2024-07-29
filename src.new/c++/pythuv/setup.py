from glob import glob
from setuptools import setup
from pybind11.setup_helpers import Pybind11Extension, build_ext

ext_modules = [
    Pybind11Extension(
        "pythuv",
        [r"D:\my-study--github\src.new\c++\pythuv\src\pythuv.cpp"],  # Sort source files for reproducibility
        include_dirs=[r"D:\Program Files\Python312\include", r"D:\Program Files\Python312\libs",
                      r"C:\Users\86139\vcpkg\installed\x64-windows\include"],
        library_dirs=[r"C:\Users\86139\vcpkg\installed\x64-windows\lib"],
        libraries=[r"uv"],
        include_pybind11=[r"C:\Users\86139\vcpkg\installed\x64-windows\include"]
    ),
]

setup(cmdclass={"build_ext": build_ext}, ext_modules=ext_modules)