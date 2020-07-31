#ifndef _ALTAIR_H
#define _ALTAIR_H

#include <QtCore/QString>

extern "C" {

struct AString {
    QString *ptr;
};

size_t AString__len(AString str);

} // extern "C"

#endif /* _ALTAIR_H */
