use crate::config;

use picodata_plugin::plugin::prelude::*;

#[derive(Debug)]
pub struct PluginService {}

impl Default for PluginService {
    fn default() -> Self {
        todo!()
    }
}

impl Service for PluginService {
    type Config = Option<config::Config>;

    fn on_config_change(
        &mut self,
        ctx: &PicoContext,
        new_config: Self::Config,
        old_config: Self::Config,
    ) -> CallbackResult<()> {
        _ = ctx;
        _ = new_config.unwrap_or_default();
        _ = old_config.unwrap_or_default();
        todo!()
    }

    fn on_start(&mut self, context: &PicoContext, config: Self::Config) -> CallbackResult<()> {
        _ = context;
        _ = config.unwrap_or_default();
        todo!()
    }

    fn on_stop(&mut self, context: &PicoContext) -> CallbackResult<()> {
        _ = context;
        todo!()
    }

    fn on_leader_change(&mut self, context: &PicoContext) -> CallbackResult<()> {
        _ = context;
        todo!()
    }

    fn on_health_check(&self, context: &PicoContext) -> CallbackResult<()> {
        _ = context;
        todo!()
    }
}
