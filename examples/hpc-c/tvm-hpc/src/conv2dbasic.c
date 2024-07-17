#include "model.h"
#include "bundle.h"
#include <stdio.h>

#define HEIGHT 32
#define WIDTH 32
#define CHANNELS 3
#define OUTPUT_SIZE 10

#define CONV_NET

extern const char graph_c_json[];
extern unsigned int graph_c_json_len;

extern const char params_c_bin[];
extern unsigned int params_c_bin_len;

extern const float stimulus_c[];
extern unsigned int stimulus_c_len;

#include <tvm/runtime/c_runtime_api.h>

int run_inference(int batch_size) {
    printf("Running mobile net\n");
    char *json_data = (char *)(graph_c_json);
    char *params_data = (char *)(params_c_bin);
    uint64_t params_size = params_c_bin_len;
    printf("Running mobile net\n");

    // create tvm_runtime
    void *handle = tvm_runtime_create(json_data, params_data, params_size, NULL);
    printf("Running mobile net\n");

    float input_storage[1 * HEIGHT * WIDTH * CHANNELS];
    printf("Running mobile net\n");

    for(int i = 0; i < stimulus_c_len; i++) {
        input_storage[i] = stimulus_c[i];
    }
    printf("Loaded %d inputs to active memory\n", stimulus_c_len);

    // Preprate input and output device and type
    DLDevice dev = {kDLCPU, 0};
    DLDataType dtype = {kDLFloat, 32, 1};
    int64_t shape[4] = {1, CHANNELS, HEIGHT, WIDTH};

    // prepare intput tensor
    DLTensor input;
    input.data = input_storage;
    input.device = dev;
    input.ndim = 4;
    input.dtype = dtype;
    input.shape = shape;
    input.strides = NULL;
    input.byte_offset = 0;

    tvm_runtime_set_input(handle, "input", &input);
    printf("Inputs set to CPU\n");

    printf("Setting up output\n");

    // Prepare output storage and device
    float output_storage[OUTPUT_SIZE];
    DLDevice out_dev = {kDLCPU, 0};
    DLDataType out_dtype = {kDLFloat, 32, 1};
    int64_t out_shape[2] = {1, OUTPUT_SIZE};

    // prepare output tensor
    DLTensor output;
    output.data = output_storage;
    output.device = out_dev;
    output.ndim = 2;
    output.dtype = out_dtype;
    output.shape = out_shape;
    output.strides = NULL;
    output.byte_offset = 0;

    printf("Running inference\n");
    tvm_runtime_run(handle);


    printf("Getting output of inference\n");
    tvm_runtime_get_output(handle, 0, &output);
    printf("Output got\n");
    for(int i = 0; i < OUTPUT_SIZE; ++i) {
        printf("%.2f ", output_storage[i]);
    }
    tvm_runtime_destroy(handle);
    printf("TVM Runtime destroyed!\n");
    return 0;
}
