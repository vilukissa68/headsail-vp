#include <stdio.h>
#include <assert.h>
#include <float.h>
#include <stdlib.h>

//#include "headsail_bsp.h"
#include "bundle.h"
//#include "mobile_net.h"
#include "conv2dbasic.h"

#include <tvm/runtime/c_runtime_api.h>


extern const char graph_c_json[];
extern unsigned int graph_c_json_len;

extern const char params_c_bin[];
extern unsigned int params_c_bin_len;

int main(void)
{
    run_inference(1);
    return 0;
}
