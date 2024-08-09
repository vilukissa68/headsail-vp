#include <stdio.h>
#include <assert.h>
#include <float.h>
#include <stdlib.h>

//#include "headsail_bsp.h"
#include "bundle.h"
#include "model.h"

#include <tvm/runtime/c_runtime_api.h>


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

int main(void)
{
    run_inference(1);
    return 0;
}
