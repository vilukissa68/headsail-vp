#include "headsail_bsp.h"
#include "boot.h"

int main() {
  int a = 0;
  char * str = "Hello world";

  for (int i = 0; i <= 11; i++) {
    putc(str[i]);
  }
  return 0;
}
