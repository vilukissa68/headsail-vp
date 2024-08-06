#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
#include <stdlib.h>


int main()
{
    printf("Hello, world! (from newlib)\n");   
    
    /** Generate two double arrays, one for data and one for results */
    double num[10];
    double res[10];

    for(int i = 0; i < 10; i++)
    {
        num[i] = (double)((double)rand() / (double)RAND_MAX);
    }

    /** 
     * Do stuff that use libm. Can be used both as a test to verify 
     * that libm works but also as a benchmark.
    */
    for(int i = 0; i < 1; i++)
    {
        for(int j = 0; j < 1; j++)
        {
            res[0] = sin(num[j]);
            res[1] = cos(num[j]);
            res[2] = tan(num[j]);
            res[3] = exp(num[j]);
            res[4] = exp2(num[j]);
            res[5] = log(num[j]);
            res[6] = log2(num[j]);
            res[7] = log10(num[j]);
            res[8] = sqrt(num[j]);
            res[9] = pow(M_PI, num[j]);
        }
    }

    /** Print those 10 numbers */
    for(int i = 0; i < 10; i++) printf("%f ", num[i]);

    /** Allocate a space from the heap */
    int alloc_size = 16384;
    void* buf = malloc(alloc_size);
    int element_size = sizeof(double);

    /** Copy the doubles in there */
    for(int i = 0; i < alloc_size / element_size; i++) 
    {
        ((double*)buf)[i] = res[i % 10];
    }

    /** Print the data saved in the buffer */
    for(int i = 0; i < alloc_size / element_size; i++) 
    {
        printf("%f ", ((double*)buf)[i]);
    }

    return 0;
}
