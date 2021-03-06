#include "altair.h"

#include <cstring>
#include <iostream>
#include <cstdio>

extern "C" {

//======================
// Application
//======================
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

void Application__add_qml_import_path(Application application, AString path)
{
    std::cout << "Application__add_qml_import_path - " << path.ptr->toStdString() << std::endl;
    application.engine->addImportPath(*(path.ptr));
}

void Application__load(Application application, AString url)
{
    std::cout << "load: " << url.ptr->toUtf8().toStdString() << std::endl;
    application.engine->load(*(url.ptr));
}

int Application__exec(Application application)
{
    std::cout << "exec" << std::endl;
    return application.app->exec();
}


//================
// AByteArray
//================
AByteArray AByteArray__new(const unsigned char *data, size_t len)
{
    AByteArray byte_array = { len, NULL };
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


//===================
// AStringRef
//===================
AString AStringRef__to_a_string(AStringRef str_ref)
{
    AString a_string = { NULL };
    a_string.ptr = new QString(*str_ref.ptr);

    return a_string;
}


//===================
// AString
//===================
size_t AString__len(AString str)
{
    return str.ptr->length();
}

void AString__drop(AString str)
{
    delete str.ptr;
    std::cout << "Dropped in C++" << std::endl;
}

} // extern "C"
