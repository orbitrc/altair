use altair::Application;
use altair::AString;
use altair::env;

fn main() {
    let argc = env::argc();
    let argv = env::argv();

    let app = Application::new(argc, &argv);

    app.exec();
}
