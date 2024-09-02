#include <stdio.h>
#include <assert.h>
#include <float.h>
#include <stdlib.h>
#include <stdint.h>

#include "dla_driver.h"
#include <tvm/runtime/crt/error_codes.h>
#include <tvmgen_default.h>

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

// Headsail newlib provides these symbols
extern char uart8250_getc(void);
extern char uart8250_putc(char ch);

extern tvm_crt_error_t TVMPlatformInitialize();

void read_stimulus(int8_t* buf, size_t len) {
    printf("Waiting for stimulus of length %d...\n", (int)len);
    for(size_t i = 0; i < len; i++) {
        buf[i] = (int8_t)uart8250_getc();
    }
    printf("Got stimulus...\n");
}

void write_prediction(int8_t* prediction, size_t len) {
    printf("Prediction:\n");
    for(int i = 0; i < len; ++i) {
        uart8250_putc(prediction[i]);
    }
    printf("\n");
}

void init_tvm() {
    dla_init();
    TVMPlatformInitialize();
    /* char *json_data = (char *)(graph_c_json); */
    /* char *params_data = (char *)(params_c_bin); */
    /* uint64_t params_size = params_c_bin_len; */
    // create tvm_runtime
    //void *handle = tvm_runtime_create(json_data, params_data, params_size, NULL);
    //return handle;
}

void run_inference() {
    printf("Running inferences\n");
    int8_t stimulus[HEIGHT * WIDTH * CHANNELS] = {0};
    signed char output[OUTPUT_SIZE];

    //read_stimulus(stimulus, HEIGHT * WIDTH * CHANNELS);
    struct tvmgen_default_inputs inputs = {
    .input_1_int8 = (void*)&stimulus
};
    struct tvmgen_default_outputs outputs = {
    .Identity_int8 = (void*)&output,
    };

    printf("Inputs set\n");

    //TVMExecute(&stimulus, &output);
    tvmgen_default_run(&inputs, &outputs);
    for(int i = 0; i < OUTPUT_SIZE; ++i) {
        printf("%d ", output[i]);
    }
    printf("\n");
    write_prediction(output, OUTPUT_SIZE);
    printf("Inference ran\n");
}

int main(void)
{
    printf("Program started!\n");
    init_tvm();
    run_inference();

    printf("Done");
    for(;;) {
    }
    return 0;
}
