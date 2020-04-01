use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ShLvlConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
    pub base_value: &'a str,
}

impl<'a> RootModuleConfig<'a> for ShLvlConfig<'a> {
    fn new() -> Self {
        ShLvlConfig {
            symbol: SegmentConfig::new("🐚 "),
            prefix: "",
            suffix: "",
            style: Color::Black.bold().dimmed(),
            disabled: false,
            base_value: "0"
        }
    }
}
