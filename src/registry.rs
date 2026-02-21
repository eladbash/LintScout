use crate::builtin;
use crate::config::Config;
use crate::error::Result;
use crate::scout::Scout;

pub struct ScoutRegistry {
    scouts: Vec<Scout>,
}

impl Default for ScoutRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ScoutRegistry {
    pub fn new() -> Self {
        ScoutRegistry { scouts: Vec::new() }
    }

    pub fn with_builtins(mut self) -> Result<Self> {
        self.scouts.extend(builtin::all()?);
        Ok(self)
    }

    pub fn with_config(mut self, config: &Config) -> Result<Self> {
        self.scouts.extend(config.build_custom_scouts()?);
        Ok(self)
    }

    pub fn filter(mut self, names: &[String]) -> Self {
        if !names.is_empty() {
            self.scouts.retain(|s| names.contains(&s.name));
        }
        self
    }

    pub fn exclude(mut self, names: &[String]) -> Self {
        if !names.is_empty() {
            self.scouts.retain(|s| !names.contains(&s.name));
        }
        self
    }

    pub fn into_scouts(self) -> Vec<Scout> {
        self.scouts
    }
}
