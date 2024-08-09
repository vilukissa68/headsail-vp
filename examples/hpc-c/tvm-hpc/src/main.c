#include <stdio.h>
#include <assert.h>
#include <float.h>
#include <stdlib.h>

//#include "headsail_bsp.h"
#include "bundle.h"
#include <tvm/runtime/c_runtime_api.h>

#ifdef IMAGE_CLASSIFICATION
#define HEIGHT 32
#define WIDTH 32
#define CHANNELS 3
#define OUTPUT_SIZE 10
#define INPUT_NAME "input_1_int8"
int64_t SHAPE[4] = {1, HEIGHT, WIDTH, CHANNELS};
#elif VISUAL_WAKEUP_WORD
#define HEIGHT 96
#define WIDTH 96
#define CHANNELS 3
#define OUTPUT_SIZE 2
#define INPUT_NAME "input_1_int8"
int64_t SHAPE[4] = {1, HEIGHT, WIDTH, CHANNELS};
#elif KEYWORD_SPOTTING
#define HEIGHT 10
#define WIDTH 49
#define CHANNELS 1
#define OUTPUT_SIZE 12
#define INPUT_NAME "input_1"
int64_t SHAPE[4] = {1, HEIGHT, WIDTH, CHANNELS};
#endif


extern const char graph_c_json[];
extern unsigned int graph_c_json_len;

extern const char params_c_bin[];
extern unsigned int params_c_bin_len;

extern char uart8250_getc(void);
extern char uart8250_putc(char ch);

void read_stimulus(int8_t* buf, size_t len) {
    printf("Waiting for stimulus...\n");
    for(size_t i = 0; i < len; i++) {
        buf[i] = (int8_t)uart8250_getc();
    }
}

void write_prediction(int8_t* prediction, size_t len) {
    printf("Prediction:\n");
    for(int i = 0; i < len; ++i) {
        uart8250_putc(prediction[i]);
    }
    printf("\n");
}

void* init_tvm() {
    char *json_data = (char *)(graph_c_json);
    char *params_data = (char *)(params_c_bin);
    uint64_t params_size = params_c_bin_len;
    // create tvm_runtime
    void *handle = tvm_runtime_create(json_data, params_data, params_size, NULL);
    return handle;
}

void run_inference(void *handle) {

    int8_t stimulus[HEIGHT * WIDTH * CHANNELS];
    read_stimulus(stimulus, HEIGHT * WIDTH * CHANNELS);

    // Preprate input and output device and type
    DLDevice dev = {kDLCPU, 0};
    DLDataType dtype = {kDLInt, 8, 1};

    // Prepare output storage and device
    int8_t output_storage[1 * OUTPUT_SIZE];
    DLDevice out_dev = {kDLCPU, 0};
    DLDataType out_dtype = {kDLInt, 8, 1};
    int64_t out_shape[2] = {1, OUTPUT_SIZE};

    // prepare intput tensor
    DLTensor input;
    input.data = stimulus;
    input.device = dev;
    input.ndim = 4;
    input.dtype = dtype;
    input.shape = SHAPE;
    input.strides = NULL;
    input.byte_offset = 0;

    // prepare output tensor
    DLTensor output;
    output.data = output_storage;
    output.device = out_dev;
    output.ndim = 2;
    output.dtype = out_dtype;
    output.shape = out_shape;
    output.strides = NULL;
    output.byte_offset = 0;

    tvm_runtime_set_input(handle, INPUT_NAME, &input);

    printf("Running inference...\n");
    tvm_runtime_run(handle);

    printf("Inference run succesfully\n");
    tvm_runtime_get_output(handle, 0, &output);

    write_prediction(output_storage, OUTPUT_SIZE);
}

int main(void)
{
    void* handle = init_tvm();
    for(;;) {
        run_inference(handle);
    }
    return 0;
}
