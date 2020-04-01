use super::{Context, Module, SegmentConfig};

use crate::config::RootModuleConfig;
use crate::configs::shlvl::ShLvlConfig;
use crate::modules::env_var::get_env_value;

/// Creates a module with the value of $SHLVL
///
/// Will display the environment variable's value if all of the following criteria are met:
///     - shlvl.disabled is absent or false
///     - $SHLVL - shlvl.base_value (default 0) is > 0
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("shlvl");
    let config: ShLvlConfig = ShLvlConfig::try_load(module.config);

    let shlvl = get_env_value("SHLVL", Some("0"))?;
    let shlvl_int = match shlvl.parse::<i8>() {
        Ok(value) => value,
        Err(_e) => 0
    };

    let base_value_int = match config.base_value.parse::<i8>() {
        Ok(value) => value,
        Err(_e) => 0
    };

    if shlvl_int <= base_value_int {
        return None
    }
    
    module.set_style(config.style);
    module.get_prefix().set_value("with ");

    module.create_segment("symbol", &config.symbol);

    // TODO: Use native prefix and suffix instead of stacking custom ones together with env_value.
    let shlvl_stacked = format!("{}{}{}", config.prefix, shlvl, config.suffix);
    module.create_segment("shlvl", &SegmentConfig::new(&shlvl_stacked));

    Some(module)
}