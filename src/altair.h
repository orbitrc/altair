#ifndef _ALTAIR_H
#define _ALTAIR_H

#include <QtWidgets/QApplication>
#include <QtQml/QQmlApplicationEngine>

#include <QtCore/QString>

extern "C" {

struct AString;


//=====================
// Application
//=====================
struct Application {
    int argc;
    QApplication *app;
    QQmlApplicationEngine *engine;
};

Application Application__new(int argc, char *argv[]);
void Application__add_qml_import_path(Application application, AString path);
void Application__load(Application application, AString url);
int Application__exec(Application application);


//===================
// AByteArray
//===================
struct AByteArray {
    size_t len;
    unsigned char *data;
};

AByteArray AByteArray__new(const unsigned char *data, size_t len);
AString AByteArray__to_a_string(AByteArray arr);
void AByteArray__drop(AByteArray arr);


//==================
// AStringRef
//==================
struct AStringRef {
    QString *ptr;
};

AString AStringRef__to_a_string(AStringRef str_ref);


//==================
// AString
//==================
struct AString {
    QString *ptr;
};

size_t AString__len(AString str);
void AString__drop(AString str);

} // extern "C"

#endif /* _ALTAIR_H */
