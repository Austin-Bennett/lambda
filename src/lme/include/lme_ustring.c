#include "lme_ustring.h"

#include <stdio.h>


ustring us_from_bytes(const uint8_t *s, const size_t len)
{


    const ustring ret = (ustring){
        .len = len,
        .str = (uint8_t*)malloc(len)
    };

    memcpy(ret.str, s, len);

    return ret;
}

void debug_print_ustring(const ustring* s)
{
    for (int i = 0; i < s->len; i++)
    {
        putc(s->str[i], stdout);
    }
}


void destroy_ustring(ustring *s)
{
    free(s->str);
    s->str = NULL;
    s->len = 0;


}