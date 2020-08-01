use altair::Application;
use altair::AString;
use altair::env;

fn main() {
    let argc = env::argc();
    let argv = env::argv();

    let app = Application::new(argc, &argv);
    let url = String::from("qrc:/main.qml");
    app.load(url);

    app.exec();
}
