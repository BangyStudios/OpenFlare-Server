mod configfactory;

fn main() {
    let config = configfactory::nginx::get_config_https("bangy.sites.icbix.com", "us1-1.edge.icbix.com", "1g",  "1h");
    println!("{config}");
}
