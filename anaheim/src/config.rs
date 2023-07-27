// TODO: Support new from Default or config, and then Builder for each field
// TODO: Also support configuration provider including remote server configuration provider
pub struct Config {}

pub trait ConfigTrait {
    fn key() -> &'static str;
}

/*


#[config(key = "path.to.config"]
pub struct Config {
    // opt
    #[default(value = "..")
    param: (),
}

// expands to

// Want to derive builder, but also add a From<Config> initializer
// TODO: re: Config_rs, can maybe do some sort of merge between the loaded config,
// and builder config, to get final result. That would make the error returned
// by builder handle both.
// The fact that there will also be Default derived might help with config_rs
#[derive(Clone, Debug, Default, Builder)
pub struct Config {
    param: (),
}

impl From<::Config> for Config {
    ...
}

// TODO: Clap config / derive
 */
