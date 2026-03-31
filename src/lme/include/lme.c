#include "lme.h"
#include "lme_error.h"

#include <stdio.h>


void destroy_lme(lme* lme)
{
    for (size_t i = 0; i < lme->functions.len; i++)
    {
        destroy_lme_func_loc(lme->functions.data + i);
    }

    DESTROY_SLICE(lme->functions)
    DESTROY_SLICE(lme->functions)
    lme->code_entry = 0;
}

size_t get_file_size(FILE *file) {
    fseek(file, 0, SEEK_END); // Jump to the end of the file
    const size_t size = ftell(file);      // Get the current byte offset
    rewind(file);            // Jump back to the beginning
    return size;
}


void destroy_lme_func_loc(lme_function_location* lme)
{
    destroy_ustring(&lme->name);
    lme->loc = 0;
}

lme* lme_from_file(const char* path)
{
    FILE* f = fopen(path, "rb");
    char fmt[512];


    if (!f)
    {
        snprintf(fmt, 511, "failed to open file %s", path);
        set_err(fmt);
        return NULL;
    }

    size_t fsize = get_file_size(f);
    uint8_t* buf = (uint8_t*)malloc(fsize);
    if (!buf)
    {
        snprintf(fmt, 511, "out of memory!");
        set_err(fmt);
        return NULL;
    }

    fread(buf, 1, fsize, f);

    if (ferror(f)) {
        snprintf(fmt, 511, "failed to read file %s", path);
        set_err(fmt);
        free(buf);
        return NULL;
    }


    lme* result = lme_from_memory(buf, fsize);
    free(buf);
    return result;
}