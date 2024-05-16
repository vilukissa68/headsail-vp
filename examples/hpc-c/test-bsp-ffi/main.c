#include <stdint.h>
#include <string.h>

#include "boot.h"
#include "headsail_bsp.h"

int main()
{
    const char *hello = "Hello world!\r\n";
    const char *str2 = "This is the BSP printing.\r\n";

    for (int i = 0; i < strlen(hello); i++)
    {
        putc(hello[i]);
    }
    for (int i = 0; i < strlen(str2); i++)
    {
        putc(str2[i]);
    }

    return 0;
}
