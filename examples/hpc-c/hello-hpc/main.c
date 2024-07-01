#include <stdio.h>

int main()
{
    setbuf(stdin, NULL);
    setbuf(stdout, NULL);
    sprintf("Hello world", stdin);
    printf("Hello world");
    for(;;){}

    return 0;
}
