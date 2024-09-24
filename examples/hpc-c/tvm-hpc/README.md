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

### Configuring cmake
Configure cmake for building tvm, by copying the config.cmake file in tvm-hpc directory to tvm repository.
``` sh
cp <config.cmake-in-tvm-hpc-directory> <path-to-tvm-repository>/build/config.cmake 
```

For example
``` sh
cp headsail-vp/examples/hpc-c/tvm-hpc/config.cmake tvm/build/config.cmake

```

To enable codegen modify config.cmake file in the build directory by setting line 162 value to pointing at llvm-config.
``` sh
vim build/config.cmake
set(USE_LLVM <path-to-llvm-config>)
set(USE_HEADSAIL ON)
```

### Building TVM

Build in the previously greated build directory
``` sh
cd build
cmake ..
make
```

Export TVM environment variables with, these need to be set when building the tvm-hpc project
```sh
export TVM_HOME=<path-to-tvm-root>
export TVM_LIBRARY_PATH=<path-to-tvm-root>/build
export PYTHONPATH=<path-to-tvm-root>/python:${PYTHONPATH}
```

More information in https://tvm.apache.org/docs/install/from_source.html


## Python dependencies
Python dependencies are needed for building TVM models from onnx graphs and must be available during tvm-hpc compilation. 

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
After succesful build, the resulting binary can be run with headsail's virtual prototype in Renode
```sh
cd /headsail-vp/scripts
./run_on_hpc.sh ../examples/hpc-c/tvm-hpc/build/headsail-tvm
```


