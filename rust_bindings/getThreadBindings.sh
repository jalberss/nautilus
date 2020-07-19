bindgen --ctypes-prefix ::libc --use-core --builtins ../include/nautilus/thread.h -o thread_bindings -- -I ../include/ -I ../include/nautilus/nautilus.h -include ../include/autoconf.h 
