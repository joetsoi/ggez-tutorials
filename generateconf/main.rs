use std::fs::File;

use ggez;

pub fn main() -> ggez::GameResult {
    let conf = ggez::conf::Conf::new();
    let mut config_file = File::create("conf.toml")?;
    conf.to_toml_file(&mut config_file);
    println!("Generated conf.toml");
    Ok(())
}
