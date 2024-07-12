import argparse
import os
from tvm import relay
import tvm
from tvm import runtime
import tvm.micro
import logging

def build_model_add(opts):
    import onnx
    onnx_path = opts.model
    onnx_model = onnx.load(onnx_path)

    input1_name = "Input1"
    input2_name = "Input2"
    shape_dict = {input1_name: [1], input2_name: [1]}
    mod, params = relay.frontend.from_onnx(onnx_model, shape_dict)

    file_format_str = "{name}_c.{ext}"
    RUNTIME = tvm.relay.backend.Runtime("crt", {"system-lib" : True})
    TARGET = tvm.target.Target("c -mcpu=generic-rv64")
    with tvm.transform.PassContext(opt_level=3, config={"tir.disable_vectorize": True}):
        lib = relay.build(mod, target=TARGET, runtime=RUNTIME, params=params)


    build_dir = os.path.abspath(opts.out_dir)
    if not os.path.isdir(build_dir):
        os.makedirs(build_dir)

    # lib_file_name = os.path.join(build_dir, file_format_str.format(name="model", ext="tar"))
    # lib.export_library(lib_file_name)
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

    # Convert graph and weights to hexdumps
    graph_path_rel = os.path.relpath("{name}_c.{ext}".format(name="graph", ext="json"))
    params_path_rel = os.path.relpath("{name}_c.{ext}".format(name="params", ext="bin"))
    os.chdir(build_dir)
    os.system("tar -zxvf {lib}".format(lib=lib_file_name))
    os.system("xxd -i {graph} > {graphc} ".format(graph=graph_path_rel, graphc=(graph_path_rel + ".c")))
    os.system("xxd -i {params} > {paramsc} ".format(params=params_path_rel, paramsc=(params_path_rel + ".c")))

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser()
    parser.add_argument("-o", "--out-dir", default="./build")
    parser.add_argument("-m", "--model", required=True)
    opts = parser.parse_args()

    build_model_add(opts)
