import sys, ctypes
from ctypes import c_ubyte, c_int, c_float, POINTER, CFUNCTYPE
from PIL import Image
import numpy as np

lib = ctypes.cdll.LoadLibrary("py_render_lib.dll")

lib.noise_texture_ds.args = (c_int, c_int, c_float, c_float, c_float,)
lib.noise_texture_ds.restype = POINTER(c_ubyte)

lib.noise_texture_ds_middlefunc.restype = POINTER(CFUNCTYPE(c_float,c_float,c_float,c_float))

#return (lt - rb - lb + rt) * 1.0
#return (lb + rt - lt - rb) * 1.0
#return (lb + rt + lt - rb) * 1.0
#return (lb + rb - lt - rt) * 1.0
#return (lb + rb + lt - rt) * 1.0
#return (lb + rb - lt + rt) * 1.0
#return (rt + rt - rb + rb) * 1.0
#return (lt + lt - rb + rb) * 1.0
#return (rt + rt - lb + lb) * 1.0
@CFUNCTYPE(c_float,c_float,c_float,c_float,c_float)
def pymiddle(lt, rt, lb, rb):
	return (rt + rt - rb + rb) * 1.0

def run_test():
	print("start noise test")
	w = 513
	#middlefunc = lib.noise_texture_ds_middlefunc()
	middlefunc = pymiddle
	result = lib.noise_texture_ds(c_int(w), c_int(w), c_float(1.0), c_float(0.5), c_float(1.0), middlefunc)
	px = []
	i = 0
	max = w * w * 3
	while i < max:
		px.append((result[i], result[i+1], result[i+2]))
		i += 3
	#lib.noise_texture_ds_free(result)
	#for col in result:
	#	px.append(col)
	#im = Image.fromarray(np.asarray(result[:max]), mode='RGB')
	im = Image.new('RGB',(513, 513))
	im.putdata(px)
	#im.save("noise.bmp")
	im.show(im)

run_test()