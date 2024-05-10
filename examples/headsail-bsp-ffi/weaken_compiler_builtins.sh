#!/bin/bash
mkdir -p tmp/
cd tmp
ARCH=riscv64-unknown-elf
FILE=$(find .. -name libheadsail_bsp_ffi.a)
$ARCH-ar x $FILE
for f in compiler_builtins-*;
do
$ARCH-objcopy --weaken $f
done
$ARCH-ar cr $FILE *
cd ..
rm -r tmp
