use durazubs::controller::App;
use durazubs::model::repository::file::FileRepository;
use durazubs::view::console::Console;

fn main() {
    let view = Console::new();
    let repository = FileRepository::new();
    let mut app = App::new(view, repository);
    app.run();
}
