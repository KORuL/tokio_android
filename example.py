#!/usr/bin/env python

from ctypes import *
cdll.LoadLibrary("./target/debug/libtokio_android.so")
libc = CDLL("./target/debug/libtokio_android.so")
libc.helloworld()
