#include "altair.h"

#include <cstring>
#include <iostream>
#include <cstdio>

extern "C" {

Application Application__new(int argc, char *argv[])
{
    std::cout << "Application__new - argc: " << argc << std::endl;
    printf("%p\n", argv);
    printf("%p\n", argv[0]);

    Application application = { argc, NULL, NULL };
    QApplication *app = new QApplication(application.argc, argv);
    QQmlApplicationEngine *engine = new QQmlApplicationEngine;
    application.app = app;
    application.engine = engine;

    return application;
}

void Application__load(Application application, AString url)
{
    application.engine->load(*(url.ptr));
    AString__drop(url);
}

int Application__exec(Application application)
{
    std::cout << "exec" << std::endl;
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
    QByteArray qba = QByteArray((const char*)(arr.data), arr.len);
    QString *qstr = new QString(qba);
    AString str = { qstr };

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
