#include "model.h"
#include "bundle.h"
#include <stdio.h>

#define HEIGHT 10
#define WIDTH 49
#define CHANNELS 1
#define OUTPUT_SIZE 12

#define PERF_IMAGE_CLASSIFICATION

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
    printf("Runtime crated \n");

    for(;;) {
       run_inference_a(handle);
    }

    tvm_runtime_destroy(handle);
}


int run_inference_a(void *handle) {
    // Preprate input and output device and type
    DLDevice dev = {kDLCPU, 0};
    DLDataType dtype = {kDLInt, 8, 1};
    int64_t shape[4] = {1, HEIGHT, WIDTH, CHANNELS};
    int8_t stimulus[HEIGHT * WIDTH * CHANNELS];
    read_stimulus(stimulus, HEIGHT * WIDTH * CHANNELS);

    // prepare intput tensor
    DLTensor input;
    input.data = stimulus;
    input.device = dev;
    input.ndim = 4;
    input.dtype = dtype;
    input.shape = shape;
    input.strides = NULL;
    input.byte_offset = 0;

    tvm_runtime_set_input(handle, "input_1", &input);
    printf("Inputs set to CPU\n");

    printf("Running inference\n");
    tvm_runtime_run(handle);
    printf("Setting up output\n");

    // Prepare output storage and device
    int8_t output_storage[1 * OUTPUT_SIZE];
    DLDevice out_dev = {kDLCPU, 0};
    DLDataType out_dtype = {kDLInt, 8, 1};
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

    printf("Getting output of inference\n");
    tvm_runtime_get_output(handle, 0, &output);
    /* for(int i = 0; i < OUTPUT_SIZE; i++) { */
    /*     printf("%d ", output_storage[i]); */
    /* } */
    write_prediction(output_storage, OUTPUT_SIZE);
    return 0;
}
