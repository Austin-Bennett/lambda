#ifndef LME_RUST_FFI_H
#define LME_RUST_FFI_H

#include "lme_slice.h"
#include "lme_ustring.h"
#include "lme_error.h"

typedef struct lme_function_location {
  ustring name;
  uint64_t loc;
} lme_function_location;

typedef struct lme
{
  uint64_t code_entry;
  SLICE(lme_function_location) functions;
  SLICE(uint8_t) code;
} lme;

extern void destroy_lme(lme* lme);
extern void destroy_lme_func_loc(lme_function_location* lme);

extern lme* lme_from_file(const char* path);

// * RUST CODE *
extern lme* lme_from_memory(const uint8_t* data, size_t len);


#endif
