import argparse
import os
import re
import textwrap
from tvm import relay
import tvm
from tvm import runtime
import tvm.micro
import logging
import onnx

def normalize(v):
    norm = np.linalg.norm(v)
    if norm == 0:
       return v
    return v / norm

def write_c_stimulus(data, len_symbol="stimulus_c_len", payload_symbol="stimulus_c", payload_type="unsigned char"):
    c_file = open("model_c/stimulus" + ".c", "w")
    len_line = "unsigned int {len_symbol} = {data_len};\n".format(len_symbol=len_symbol, data_len=len(data))

    print(data)
    data_str = ""
    for x in data:
        data_str += str(x) + ", "
    data_str = data_str[:-2]

    payload = textwrap.fill(data_str, 80, initial_indent="  ", subsequent_indent="  ")
    payload_line = "{payload_type} {payload_symbol}[] = {{\n{payload}}};\n".format(payload_type=payload_type,
                                                                                   payload_symbol=payload_symbol,
                                                                                   payload=payload)
    c_file.write(len_line + payload_line)
    c_file.close()
    print("Stimulus written!")


def build_model_add(opts):
    onnx_path = opts.model_path
    onnx_model = onnx.load(onnx_path)

    input1_name = "Input1"
    input2_name = "Input2"
    shape_dict = {input1_name: [1], input2_name: [1]}
    mod, params = relay.frontend.from_onnx(onnx_model, shape_dict)

    file_format_str = "{name}_c.{ext}"
    RUNTIME = tvm.relay.backend.Runtime("crt", {"system-lib" : True})
    TARGET = tvm.target.Target("llvm -mtriple=riscv64-unknown-elf -mcpu=generic-rv64 -mabi=lp64 -mattr=+64bit")
    with tvm.transform.PassContext(opt_level=3, config={"tir.disable_vectorize": True}):
        lib = relay.build(mod, target=TARGET, runtime=RUNTIME, params=params)


    build_dir = os.path.abspath(opts.out_dir)
    if not os.path.isdir(build_dir):
        os.makedirs(build_dir)

    lib_file_name = os.path.join(build_dir, file_format_str.format(name="model", ext="tar"))
    lib.export_library(lib_file_name)

    with open(
        os.path.join(build_dir, file_format_str.format(name="graph", ext="json")), "w"
    ) as f_graph_json:
        f_graph_json.write(lib.get_graph_json())

    with open(
        os.path.join(build_dir, file_format_str.format(name="params", ext="bin")), "wb"
    ) as f_params:
        f_params.write(runtime.save_param_dict(lib.get_params()))

    # Generate stimulus
    if opts.stimulus != None:
        _, stim_ext = os.path.splitext(opts.stimulus)
        if stim_ext == ".npy":
            import numpy as np
            data = np.load(opts.stimulus).flatten()
            write_c_stimulus(data, payload_type=opts.input_type)

    os.chdir(build_dir)
    # Convert graph and weights to hexdumps
    graph_path_rel = os.path.relpath("{name}_c.{ext}".format(name="graph", ext="json"))
    params_path_rel = os.path.relpath("{name}_c.{ext}".format(name="params", ext="bin"))
    os.system("tar -zxvf {lib}".format(lib=lib_file_name))
    os.system("xxd -i {graph} > {graphc} ".format(graph=graph_path_rel, graphc=(graph_path_rel + ".c")))
    os.system("xxd -i {params} > {paramsc} ".format(params=params_path_rel, paramsc=(params_path_rel + ".c")))

def build_model_mobilenet(opts):
    onnx_path = opts.model_path
    onnx_model = onnx.load(onnx_path)

    input1_name = "input"
    shape_dict = {input1_name: (1,3,224,224)}
    mod, params = relay.frontend.from_onnx(onnx_model, shape_dict)

    file_format_str = "{name}_c.{ext}"
    RUNTIME = tvm.relay.backend.Runtime("crt", {"system-lib" : True})
    TARGET = tvm.target.Target("llvm -mtriple=riscv64-unknown-elf -mcpu=generic-rv64 -mabi=lp64 -mattr=+64bit")
    with tvm.transform.PassContext(opt_level=3, config={"tir.disable_vectorize": True}):
        lib = relay.build(mod, target=TARGET, runtime=RUNTIME, params=params)


    build_dir = os.path.abspath(opts.out_dir)
    if not os.path.isdir(build_dir):
        os.makedirs(build_dir)

    lib_file_name = os.path.join(build_dir, file_format_str.format(name="model", ext="tar"))
    lib.export_library(lib_file_name)

    with open(
        os.path.join(build_dir, file_format_str.format(name="graph", ext="json")), "w"
    ) as f_graph_json:
        f_graph_json.write(lib.get_graph_json())

    with open(
        os.path.join(build_dir, file_format_str.format(name="params", ext="bin")), "wb"
    ) as f_params:
        f_params.write(runtime.save_param_dict(lib.get_params()))

    # Generate stimulus
    if opts.stimulus != None:
        _, stim_ext = os.path.splitext(opts.stimulus)
        if stim_ext == ".npy":
            import numpy as np
            data = np.load(opts.stimulus).flatten()
            data = normalize(data)
            write_c_stimulus(data, payload_type=opts.input_type)

    os.chdir(build_dir)
    # Convert graph and weights to hexdumps
    graph_path_rel = os.path.relpath("{name}_c.{ext}".format(name="graph", ext="json"))
    params_path_rel = os.path.relpath("{name}_c.{ext}".format(name="params", ext="bin"))
    os.system("tar -zxvf {lib}".format(lib=lib_file_name))
    os.system("xxd -i {graph} > {graphc} ".format(graph=graph_path_rel, graphc=(graph_path_rel + ".c")))
    os.system("xxd -i {params} > {paramsc} ".format(params=params_path_rel, paramsc=(params_path_rel + ".c")))


def build_model_conv2dbasic(opts):
    onnx_path = opts.model_path
    onnx_model = onnx.load(onnx_path)

    input1_name = "input"
    shape_dict = {input1_name: (1,3,32,32)}
    mod, params = relay.frontend.from_onnx(onnx_model, shape_dict)

    file_format_str = "{name}_c.{ext}"
    RUNTIME = tvm.relay.backend.Runtime("crt", {"system-lib" : True})
    TARGET = tvm.target.Target("llvm -mtriple=riscv64-unknown-elf -mcpu=generic-rv64 -mabi=lp64 -mattr=+64bit")
    with tvm.transform.PassContext(opt_level=3, config={"tir.disable_vectorize": True}):
        lib = relay.build(mod, target=TARGET, runtime=RUNTIME, params=params)


    build_dir = os.path.abspath(opts.out_dir)
    if not os.path.isdir(build_dir):
        os.makedirs(build_dir)

    lib_file_name = os.path.join(build_dir, file_format_str.format(name="model", ext="tar"))
    lib.export_library(lib_file_name)

    with open(
        os.path.join(build_dir, file_format_str.format(name="graph", ext="json")), "w"
    ) as f_graph_json:
        f_graph_json.write(lib.get_graph_json())

    with open(
        os.path.join(build_dir, file_format_str.format(name="params", ext="bin")), "wb"
    ) as f_params:
        f_params.write(runtime.save_param_dict(lib.get_params()))

    # Generate stimulus
    if opts.stimulus != None:
        _, stim_ext = os.path.splitext(opts.stimulus)
        if stim_ext == ".npy":
            import numpy as np
            data = np.load(opts.stimulus).flatten()
            write_c_stimulus(data, payload_type=opts.input_type)

    os.chdir(build_dir)
    # Convert graph and weights to hexdumps
    graph_path_rel = os.path.relpath("{name}_c.{ext}".format(name="graph", ext="json"))
    params_path_rel = os.path.relpath("{name}_c.{ext}".format(name="params", ext="bin"))
    os.system("tar -zxvf {lib}".format(lib=lib_file_name))
    os.system("xxd -i {graph} > {graphc} ".format(graph=graph_path_rel, graphc=(graph_path_rel + ".c")))
    os.system("xxd -i {params} > {paramsc} ".format(params=params_path_rel, paramsc=(params_path_rel + ".c")))



if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser()
    parser.add_argument("-o", "--out-dir", default="./build")
    parser.add_argument("-m", "--model", required=True)
    parser.add_argument("-p", "--model_path", required=True)
    parser.add_argument("-s", "--stimulus", default=None)
    parser.add_argument("-t", "--input_type", default="unsigned char")

    opts = parser.parse_args()

    if opts.model == "add":
        build_model_add(opts)
    elif opts.model == "mobilenet":
        build_model_mobilenet(opts)
    elif opts.model == "conv2dbasic":
        build_model_conv2dbasic(opts)
    else:
        print("No such model", opts.model, "Availeable models: add, mobilenet")
