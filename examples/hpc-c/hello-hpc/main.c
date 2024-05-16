#include <stdint.h>
#include <string.h>

#include "boot.h"

const uintptr_t UART0_ADDR = 0xFFF00000;
uint8_t *const UART0_PTR = (uint8_t *)UART0_ADDR;

void putc(char ch)
{
    *UART0_PTR |= ch;
}

int main()
{
    const char *str = "Hello world!\r\n";

    for (int i = 0; i < strlen(str); i++)
    {
        putc(str[i]);
    }

    return 0;
}
