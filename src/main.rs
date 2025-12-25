use durazubs::controller::App;
use durazubs::view::console::Console;

fn main() {
    let view = Console::new();
    let mut app = App::new(view);
    app.run();
}
