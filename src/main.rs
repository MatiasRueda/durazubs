use durazubs::controller::app::App;

fn main() {
    let path_a = "a.ass";
    let path_b = "b.ass";
    let output_path = "final_result.ass";
    let mut app = App::new(path_a, path_b, output_path);
    app.run();
}
