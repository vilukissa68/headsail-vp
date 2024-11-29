import argparse
import os
import re
import textwrap
from tvm import relay
import tvm
from tvm import runtime
from tvm.relay.op.contrib.register import get_pattern_table
from tvm.relay.op.contrib.headsail import legalize_qnn_for_headsail
from tvm.micro import export_model_library_format
import tvm.micro
import logging
import onnx
import numpy as np
from pathlib import Path
from tiny_perf_benchmark import get_ic_stimulus, get_kws_stimulus, get_vww_stimulus

SHAPES = {
    "perf_ic": {"input_1_int8": (1, 32, 32, 3)},
    "perf_vww": {"data": (1, 96, 96, 3)},
    "perf_kws": {"data": (1, 49, 10, 1)},
}

def normalize(v):
    norm = np.linalg.norm(v)
    if norm == 0:
       return v
    return v / norm

def import_onnx_model(path_to_model, shape_dict):
    onnx_model = onnx.load(path_to_model)
    mod, params = relay.frontend.from_onnx(onnx_model, shape_dict)
    return mod, params

def import_pytorch_model(path_to_model, shape_dict):
    shape_list = []
    for input_key in shape_dict:
        shape_list.append((input_key, shape_dict[input_key]))

    onnx_model = onnx.load(path_to_model)
    mod, params = relay.frontend.from_pytorch(onnx_model, shape_list)
    return mod, params

def import_tflite_model(path_to_model, shape_dict, input_type):
    import tflite
    import tensorflow as tf
    tflite_model_buf = open(path_to_model, "rb").read()
    tf_model = tflite.Model.GetRootAsModel(tflite_model_buf, 0)
    dtype_dict = {}
    for input_key in shape_dict:
        dtype_dict[input_key] = input_type

    print(shape_dict)
    print(dtype_dict)
    mod, params = relay.frontend.from_tflite(tf_model, shape_dict, dtype_dict)
    return mod, params

def import_tf_model(path_to_model, shape_dict, input_type):
    dtype_dict = {}
    for input_key in shape_dict:
        dtype_dict[input_key] = input_type

    mod, params = relay.frontend.from_tensorflow(path_to_model, shape_dict, dtype_dict)
    return mod, params


def headsail_annotate(mod):
    print("Annotating graph for Headsail DLA")
    headsail_patterns = get_pattern_table("headsail")

    # Pre process
    desired_layouts = {'qnn.conv2d': ['NCHW', 'OIHW']}
    preprocessor_pass = tvm.transform.Sequential([relay.transform.InferType(),
                                                  relay.transform.ConvertLayout(desired_layouts),
                                                  relay.transform.SimplifyExpr()])

    mod = legalize_qnn_for_headsail(mod)
    annotation_pass = tvm.transform.Sequential(
        [
            relay.transform.MergeComposite(headsail_patterns),
            relay.transform.AnnotateTarget(["headsail"]),
            relay.transform.MergeCompilerRegions(),
            relay.transform.PartitionGraph(),
            relay.transform.FoldConstant(),
        ])
    mod = annotation_pass(mod)

    return mod

def export_annotated_library(mod, params, build_dir):
    print(tvm.target.Target.list_kinds())
    file_format_str = "{name}_c.{ext}"
    RUNTIME = tvm.relay.backend.Runtime("crt", {"system-lib" : False})
    TARGET = tvm.target.Target("c")
    EXECUTOR = tvm.relay.backend.Executor("aot", {"unpacked-api": True, "interface-api": "c", "link-params": True})
    with tvm.transform.PassContext(opt_level=3, config={"tir.disable_vectorize": True}):
        lib = relay.build(mod, target=TARGET, runtime=RUNTIME, params=params, executor=EXECUTOR)

    lib_file_name = os.path.join(build_dir, file_format_str.format(name="model", ext="tar"))
    export_model_library_format(lib, lib_file_name)
    return lib, lib_file_name

def generate_hex_dumps(lib_file_name, build_dir):
    owd = os.getcwd()
    os.chdir(build_dir)
    os.system("tar -xvf {lib}".format(lib=lib_file_name))
    os.chdir(owd)

def build_model(opts, shape_dict):

    # Create build dir for model files
    build_dir = os.path.abspath(opts.out_dir)
    if not os.path.isdir(build_dir):
        os.makedirs(build_dir)

    model_ext = os.path.splitext(opts.model_path)[1]
    if model_ext == ".onnx":
        mod, params = import_onnx_model(opts.model_path, shape_dict)
    elif model_ext == ".pth":
        mod, params = import_pytorch_model(opts.model_path, shape_dict)
    elif model_ext == ".tflite":
        mod, params = import_tflite_model(opts.model_path, shape_dict, opts.input_type)
    elif model_ext == ".tf":
        mod, params = import_tf_model(opts.model_path, shape_dict, opts.input_type)
    else:
        print("Error! Unsupported model", opts.model_path)
        return

    # Annotate model with headsail tags
    if opts.annotate_graph:
        mod = headsail_annotate(mod)

    # Write mod log to output
    with open(os.path.join(build_dir, "mod_output.txt"), "w") as mod_log:
        mod_log.write(str(mod))

    # Export library
    lib, lib_file_name = export_annotated_library(mod, params, build_dir)
    file_format_str = "{name}_c.{ext}"

    # Convert graph and weights to hexdumps
    os.chdir(build_dir)
    generate_hex_dumps(lib_file_name, build_dir)

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser()
    parser.add_argument("-o", "--out-dir", default="./build")
    parser.add_argument("-m", "--model", required=True)
    parser.add_argument("-p", "--model_path", required=True)
    parser.add_argument("-t", "--input_type", default="unsigned char")
    parser.add_argument("--annotate-graph", action="store_true")

    opts = parser.parse_args()

    if opts.model == "perf_image_classification":
        build_model(opts, shape_dict=SHAPES["perf_ic"])
        os.chdir(os.path.abspath("../"))
    elif opts.model == "perf_visual_wakeup_word":
        build_model(opts, shape_dict=SHAPES["perf_vww"])
        os.chdir(os.path.abspath("../"))
    elif opts.model == "perf_keyword_spotting":
        build_model(opts, shape_dict=SHAPES["perf_kws"])
        os.chdir(os.path.abspath("../"))
    else:
        print("No such model", opts.model, "Availeable models: perf_image_classification, perf_visual_wakeup_word, perf_keyword_spotting")
