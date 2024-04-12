/// Configuration of the program
pub struct Config {
    pub enable_cjk: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enable_cjk: false }
    }
}
