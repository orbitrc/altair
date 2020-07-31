#ifndef _ALTAIR_H
#define _ALTAIR_H

#include <QtWidgets/QApplication>
#include <QtQml/QQmlApplicationEngine>

#include <QtCore/QString>

extern "C" {

struct AString;


struct Application {
    QApplication *app;
    QQmlApplicationEngine *engine;
};

Application Application__new(int argc, char *argv[]);
void Application__load(Application application, AString url);
int Application__exec(Application application);


struct AByteArray {
    size_t len;
    unsigned char *data;
};

AByteArray AByteArray__new(const unsigned char *data, size_t len);
AString AByteArray__to_a_string(AByteArray arr);
void AByteArray__drop(AByteArray arr);


struct AString {
    QString *ptr;
};

size_t AString__len(AString str);
void AString__drop(AString str);

} // extern "C"

#endif /* _ALTAIR_H */
