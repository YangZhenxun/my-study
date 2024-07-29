from setuptools import setup
from Cython.Build import cythonize
setup( ext_modules = cythonize(["test.pyx"], language="C++", exclude="ctestrand.pxd"), include_dirs=".\\")
