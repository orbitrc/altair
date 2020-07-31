#include "altair.h"

extern "C" {

size_t AString__len(AString str)
{
    return str.ptr->length();
}

} // extern "C"
