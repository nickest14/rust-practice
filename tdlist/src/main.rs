use tdlist::configuration::get_configuration;

fn main() {
    let settings = get_configuration();
    println!("To do list {:?}", settings);
}
