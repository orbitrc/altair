#ifndef MY_EXTENSION_H_
#define MY_EXTENSION_H_

#include <QtCore/QObject>

#include <altair.h>

class MyExtension : public QObject
{
    Q_OBJECT

    Q_PROPERTY(QString name READ name WRITE set_name NOTIFY name_changed)
    Q_PROPERTY(int age READ age WRITE set_age NOTIFY age_changed)

public:
    MyExtension(QObject *parent = nullptr);

    QString name();
    void set_name(QString val);
    int age();
    void set_age(int val);

    void get_older();

signals:
    void name_changed();
    void age_changed();

public:
    QString m_name;
    int m_age;
};

extern "C" {

void MyExtension__get_older(MyExtension *self);

} // extern "C"

MyExtension::MyExtension(QObject *parent)
    : QObject(parent)
{
}

QString MyExtension::name()
{
    return this->m_name;
}

void MyExtension::set_name(QString val)
{
    if (this->m_name != val) {
        this->m_name = val;
        emit this->name_changed();
    }
}

int MyExtension::age()
{
    return this->m_age;
}

void MyExtension::set_age(int val)
{
    if (this->m_age != val) {
        this->m_age = val;
        emit this->age_changed();
    }
}

void MyExtension::get_older()
{
    MyExtension__get_older(this);
}

extern "C" {

AStringRef MyExtension__name(MyExtension *self)
{
    AStringRef a_string = { &self->m_name };
    return a_string;
}

void MyExtension__set_name(MyExtension *self, AString val)
{
    self->set_name(*val.ptr);
}

int MyExtension__age(MyExtension *self)
{
    return self->age();
}

void MyExtension__set_age(MyExtension *self, int val)
{
    self->set_age(val);
}

} // extern "C"

#endif // MY_EXTNSION_H_