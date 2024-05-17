# Notes
This build is only tested on python3.11.

# Installing dependencies
## Installing TVM
Get sources
``` sh
git clone --recursive https://github.com/apache/tvm tvm
cd tvm
mkdir build
```

Configure cmake for building tvm, by copying the config.cmake file in tvm-hpc directory to tvm repository.
``` sh
cp <config.cmake-in-tvm-hpc-directory> <path-to-tvm-repository>/build/config.cmake 
```

For example:
``` sh
cp headsail-vp/examples/hpc-c/tvm-hpc/config.cmake tvm/build/config.cmake

```

To enable codegen modify config.cmake file in the build directory by setting line 162 value to pointing at llvm-config.
``` sh
vim build/config.cmake
set(USE_LLVM /bin/llvm-config)
```

Build tvm

``` sh
cd build
cmake ..
make
```

Export TVM environment variables with
```sh
export TVM_HOME=$HOME/work/tvm
export TVM_LIBRARY_PATH=$HOME/work/tvm/build
export PYTHONPATH=$TVM_HOME/python:${PYTHONPATH}
```

More information: https://tvm.apache.org/docs/install/from_source.html


## Python dependencies
Python dependencies are needed for building TVM models from onnx graphs. 

Install python dependencies for TVM
``` sh
cd tvm/python
python gen_requirements.py
pip install -r requirements/core.txt
```

Install python dependencies for tvm-hpc project
``` sh
cd headsail-vp/examples/hpc-c/tvm-hpc/
pip install -r requirements.txt
```

# Building project

In project folder (tvm-hpc)
```sh
mkdir build
cd build
cmake ..
make
```
This creates a binary called headsail-tvm
# Running in renode
```sh
cd /headsail-vp/scripts
./run_on_hpc.sh ../examples/hpc-c/tvm-hpc/build/headsail-tvm
```


