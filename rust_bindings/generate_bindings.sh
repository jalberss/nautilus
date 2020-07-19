#!/bin/bash
for filename in ~/nautilus/include/nautilus/*; do
    f=$(basename $filename)
    f=${f%.*}
    bindgen --ctypes-prefix ::libc --use-core --builtins $filename -o ./bindings/$f"_bindings.rs" -- -I../include/ -I../include/nautilus/ -include stdint.h -include ../include/nautilus/naut_types.h
    #bindgen $filename -o "Bindings"$f --rust-target nightly -- -I /home/jalbers/nautilus/include/
done

# bindgen --ctypes-prefix ::libc --use-core --builtins ../include/nautilus/dev.h -o dev_bindings.rs -- -I ../include/

