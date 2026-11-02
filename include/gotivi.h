#ifndef GOTIVI_H
#define GOTIVI_H
#include <stdint.h>
#include <stdbool.h>
#ifdef __cplusplus
extern "C" {
#endif
/* The returned strings are borrowed: use or copy before the next session call. */
typedef struct FfiSession GotiviSession;
typedef struct GotiviAction {
    uint32_t kind;  /* 0 PASS, 1 REPLACE, 2 PREEDIT, 3 COMMIT, 4 BUSY */
    uint32_t revision;
    uint32_t delete_utf16;
    bool pass_key;
    const char *text;     /* UTF-8; value to insert */
    const char *expected; /* UTF-8; prior text that must match */
} GotiviAction;
GotiviSession *gotivi_new(uint32_t mode); /* 0 Telex 1 Simple 2 VNI 3 VIQR 4 English */
void gotivi_free(GotiviSession *session);
GotiviAction gotivi_feed(GotiviSession *session, const char *key);
GotiviAction gotivi_switch_mode(GotiviSession *session, uint32_t mode);
bool gotivi_ack(GotiviSession *session, uint32_t revision, bool accepted);
void gotivi_reset(GotiviSession *session);
#ifdef __cplusplus
}
#endif
#endif
