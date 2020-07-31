#include "altair.h"

#include <cstring>

extern "C" {

Application Application__new(int argc, char *argv[])
{
    QApplication *app = new QApplication(argc, argv);
    QQmlApplicationEngine *engine = new QQmlApplicationEngine;
    Application application = { app, engine };

    return application;
}

void Application__load(Application application, AString url)
{
    application.engine->load(*(url.ptr));
    AString__drop(url);
}

int Application__exec(Application application)
{
    return application.app->exec();
}


AByteArray AByteArray__new(const unsigned char *data, size_t len)
{
    AByteArray byte_array = { 0, NULL };
    byte_array.data = new unsigned char[len];
    memcpy(byte_array.data, data, len);

    return byte_array;
}

AString AByteArray__to_a_string(AByteArray arr)
{
    QByteArray qba = QByteArray(arr.data, arr.len);
    QString *qstr = new QString(qba);
    AString str = { &qstr };

    return str;
}

void AByteArray__drop(AByteArray arr)
{
    delete[] arr.data;
}


size_t AString__len(AString str)
{
    return str.ptr->length();
}

void AString__drop(AString str)
{
    delete str.ptr;
}

} // extern "C"
