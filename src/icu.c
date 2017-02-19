#include <unicode/ucol.h>

UCollator * __rs_ucol_open(const char *loc, UErrorCode *status) {
  ucol_open(loc, status);
}

void __rs_ucol_close(UCollator *coll) {
  ucol_close(coll);
}

UCollationResult __rs_ucol_strcollUTF8(const UCollator *coll, const char *source, int32_t sourceLength, const char *target, int32_t targetLength, UErrorCode *status) {
  ucol_strcollUTF8(coll, source, sourceLength, target, targetLength, status);
}
