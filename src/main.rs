pub mod config;

fn main() {
    let config_str = r#"
name = "example"
interval = 5

[[http]]
name = "web server"
url = "http://localhost:8080"
timeout = 5

[[handlers]]
name = "restart web server docker container"
state = "failed"
command = "docker restart example"
    "#;

    let config: config::Configuration = toml::from_str(config_str).unwrap();

    println!("{:#?}", config);
}
