use tdlist::configuration::get_configuration;
use tdlist::{app::App, cli};

fn main() {
    let settings = get_configuration();
    let app = App::new(settings);
    if std::env::args().len() > 1 {
        _ = cli::start_cli(app);
    }
}
