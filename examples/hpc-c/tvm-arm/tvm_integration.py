#!/usr/bin/env python3
import os
import numpy as np
import pathlib
import json
from PIL import Image
import tarfile

import tvm
from tvm import relay
from tvm.relay.backend import Executor, Runtime
from tvm.contrib.download import download_testdata
from tvm.micro import export_model_library_format
from tvm.relay.op.contrib import cmsisnn
from tvm.micro.testing.utils import create_header_file

MODEL_URL = "https://github.com/mlcommons/tiny/raw/bceb91c5ad2e2deb295547d81505721d3a87d578/benchmark/training/visual_wake_words/trained_models/vww_96_int8.tflite"
MODEL_NAME = "vww_96_int8.tflite"
MODEL_PATH = download_testdata(MODEL_URL, MODEL_NAME, module="model")

tflite_model_buf = open(MODEL_PATH, "rb").read()
try:
    import tflite

    tflite_model = tflite.Model.GetRootAsModel(tflite_model_buf, 0)
except AttributeError:
    import tflite.Model

    tflite_model = tflite.Model.Model.GetRootAsModel(tflite_model_buf, 0)

input_shape = (1, 96, 96, 3)
INPUT_NAME = "input_1_int8"
relay_mod, params = relay.frontend.from_tflite(
    tflite_model, shape_dict={INPUT_NAME: input_shape}, dtype_dict={INPUT_NAME: "int8"}
)

# Use the C runtime (crt)
RUNTIME = Runtime("crt")

# We define the target by passing the board name to `tvm.target.target.micro`.
# If your board is not included in the supported models, you can define the target such as:
TARGET = tvm.target.Target("llvm -mtriple=riscv64-unknown-elf -mcpu=generic-rv64 -mabi=lp64")
#TARGET = tvm.target.Target("llvm -mcpu=riscv64")

# Use the AOT executor rather than graph or vm executors. Use unpacked API and C calling style.
EXECUTOR = tvm.relay.backend.Executor(
    name="aot", options={"unpacked-api": True, "interface-api": "c", "workspace-byte-alignment": 8}
)

# Now, we set the compilation configurations and compile the model for the target:
config = {"tir.disable_vectorize": True}

with tvm.transform.PassContext(opt_level=3, config=config):
    lowered = tvm.relay.build(
        relay_mod, target=TARGET, params=params, runtime=RUNTIME, executor=EXECUTOR
    )
parameter_size = len(tvm.runtime.save_param_dict(lowered.get_params()))
print(f"Model parameter size: {parameter_size}")

# We need to pick a directory where our file will be saved.
# If running on Google Colab, we'll save everything in ``/root/tutorial`` (aka ``~/tutorial``)
# but you'll probably want to store it elsewhere if running locally.

BUILD_DIR = pathlib.Path("/Users/vainogranat/work/headsail-vp/examples/hpc-c/tvm-hpc/model")

BUILD_DIR.mkdir(exist_ok=True)

# Now, we export the model into a tar file:
TAR_PATH = pathlib.Path(BUILD_DIR) / "model.tar"
export_model_library_format(lowered, TAR_PATH)

with tarfile.open(TAR_PATH, mode="a") as tar_file:
    SAMPLES_DIR = "samples"
    SAMPLE_PERSON_URL = (
        "https://github.com/tlc-pack/web-data/raw/main/testdata/microTVM/data/vww_sample_person.jpg"
    )
    SAMPLE_NOT_PERSON_URL = "https://github.com/tlc-pack/web-data/raw/main/testdata/microTVM/data/vww_sample_not_person.jpg"

    SAMPLE_PERSON_PATH = download_testdata(SAMPLE_PERSON_URL, "person.jpg", module=SAMPLES_DIR)
    img = Image.open(SAMPLE_PERSON_PATH)
    create_header_file("sample_person", np.asarray(img), SAMPLES_DIR, tar_file)

    SAMPLE_NOT_PERSON_PATH = download_testdata(
        SAMPLE_NOT_PERSON_URL, "not_person.jpg", module=SAMPLES_DIR
    )
    img = Image.open(SAMPLE_NOT_PERSON_PATH)
    create_header_file("sample_not_person", np.asarray(img), SAMPLES_DIR, tar_file)
