# -*- coding: utf-8 -*-

import os

current_path = current_dir = os.path.dirname(os.path.abspath(__file__))
lib_path = os.path.join(current_path, 'target/release/libembed.dylib')

from ctypes import cdll

lib = cdll.LoadLibrary(lib_path)

lib.process()

print "Done"
