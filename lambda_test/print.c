#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

typedef __attribute__((packed)) struct lm_str {
  size_t size;
  uint8_t *str;
} lm_str;

void cprint(lm_str s) {
  for (size_t i = 0; i < s.size; i++) {
    putc(s.str[i], stdout);
  }
}
