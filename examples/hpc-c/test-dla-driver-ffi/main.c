#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "dla_driver.h"

int main()
{
    printf("DLA FFI Test start!\n");
    dla_init();
    int8_t A[9] = {1,2,3,4,5,6,7,8,9};
    int8_t B[4] = {1,2,3,4};
    int16_t bias[1] = {20};
    char * input_order = "CHW";
    char * kernel_order = "KCHW";
    int8_t* C = malloc(4);
    dla_conv2d_relu(A, 1, 3, 3, input_order, B, 1, 1, 2, 2, kernel_order, 0, 0, 0, 0, 0, 1, 1, 0 ,0, C);
    for(int i = 0; i < 4; ++i) {
        printf("%d ", C[i]);
    }
    printf("Done!\n");
    for(;;){}
}
