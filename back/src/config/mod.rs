use colored::Colorize;
use envconfig::Envconfig;
use rand::distributions::Alphanumeric;
use rand::Rng;

#[derive(envconfig_derive::Envconfig)]
pub struct Config {
    #[envconfig(from = "ADDRESS", default = "localhost:8080")]
    pub address: String,
    #[envconfig(from = "DATABASE_URL", default = "postgres://localhost/demiboard")]
    pub db_url: String,
    #[envconfig(from = "PRIVATE_KEY", default = "")]
    pub private_key: String,
    #[envconfig(from = "HTTPS", default = "false")]
    pub https: bool,
    #[envconfig(from = "STATIC_DIR", default = "./tmp")]
    pub static_dir: String,
}

impl Config {
    pub fn print(&self) {
        println!(
            "{}",
            "===================================================".cyan()
        );
        println!("{}: http://{}", "Server address".cyan(), self.address);
        println!("{}: {}", "Postgres address".cyan(), self.db_url);
        println!(
            "{}",
            "===================================================".cyan()
        );
    }

    pub fn create() -> Self {
        let mut config = Config::init().unwrap();

        if !config.private_key.len() > 0 {
            let mut rng = rand::thread_rng();
            config.private_key = std::iter::repeat(())
                .map(|_| rng.sample(Alphanumeric))
                .take(32)
                .collect();
            eprintln!(
                "{}: Private key missing, using a randomly generated one",
                "Warning".yellow()
            );
        }
        config
    }
}
