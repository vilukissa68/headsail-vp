#!/usr/bin/env python3
import argparse
import onnx
import tvm
from tvm import relay
from tvm.relay.backend import Executor, Runtime

def onnx_to_relay(onnx_model_path):
    onnx_model = onnx.load(onnx_model_path)
    print(dir(onnx_model))
    onnx.checker.check_model(onnx_model)
    mod, params = relay.frontend.from_onnx(onnx_model)
    return mod, params


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-o", "--out_dir", default=".")
    parser.add_argument("-m", "--model_path", default=".")
    opts = parser.parse_args()

    RUNTIME = tvm.relay.backend.Runtime("crt", {"system-lib" : True})
    TARGET = "llvm -mtriple=riscv64-unknown-elf -mcpu=generic-rv64 -mabi=lp64 -mfloat-abi=hard"

    onnx_path = opts.model_path
    onnx_model = onnx.load(onnx_path)

    input1_name = "Input1"
    input2_name = "Input2"
    shape_dict = {input1_name: [1], input2_name: [1]}
    mod, params = relay.frontend.from_onnx(onnx_model, shape_dict)

    with tvm.transform.PassContext(opt_level=3, config={}):
        lib = relay.build(mod, target=TARGET, runtime=RUNTIME, params=params)
        lib.export_library(opts.out_dir, cc="riscv64-unknown-elf-gcc")
