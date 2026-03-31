#include "lme_error.h"

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* last_err_message = NULL;
const char* get_last_err()
{
    return last_err_message;
}
void set_err(char* msg)
{
    if (last_err_message != NULL)
    {
        free(last_err_message);
    }
    size_t len = strlen(msg);
    last_err_message = (char*)malloc(len + 1);
    
    if (last_err_message == NULL)
    {
        fprintf(stderr, "out of memory\n");
        abort();
    }
    
    memcpy(last_err_message, msg, len);
    last_err_message[len] = '\0';
    
}