#ifndef GOTIVI_H
#define GOTIVI_H
#include <stdint.h>
#include <stdbool.h>
#ifdef __cplusplus
extern "C" {
#endif
typedef struct Session GotiviSession;
typedef struct { uint32_t kind,delete_utf16,cursor_utf16; bool pass_key; const char *text; } GotiviAction;
GotiviSession* gotivi_new(uint32_t mode);
void gotivi_free(GotiviSession* session);
GotiviAction gotivi_feed(GotiviSession* session,const char* key);
#ifdef __cplusplus
}
#endif
#endif
