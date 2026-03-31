#ifndef LME_LME_ERROR_H
#define LME_LME_ERROR_H

extern char* last_err_message;

extern const char* get_last_err();
extern void set_err(char* msg);

#endif