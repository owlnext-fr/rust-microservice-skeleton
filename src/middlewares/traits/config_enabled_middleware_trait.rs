use config::Config;

pub trait ConfigEnabledMiddlewareTrait {
    fn set_config(&mut self, config: &Config) -> Self;
    fn get_db_conn(&self) -> &Config;
}
