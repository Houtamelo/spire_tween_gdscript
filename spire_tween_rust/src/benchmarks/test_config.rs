use godot::prelude::*;
use godot::classes::Os;

/// Reads benchmark configuration from environment variables.
///
/// Mirrors `test_config.gd`: checks `TEST_NODE_AMOUNT`, `TEST_DURATION`,
/// and `TEST_IS_BUILTIN` env vars for overriding default benchmark settings.
pub struct TestConfig {
    pub duration: Option<f64>,
    pub amount: Option<i64>,
    pub is_builtin: Option<bool>,
}

impl TestConfig {
    pub fn from_env() -> Self {
        let os = Os::singleton();

        let amount = {
            let val = os.get_environment("TEST_NODE_AMOUNT");
            if val.is_empty() {
                None
            } else {
                Some(val.to_string().parse::<i64>().unwrap_or(0))
            }
        };

        let duration = {
            let val = os.get_environment("TEST_DURATION");
            if val.is_empty() {
                None
            } else {
                Some(val.to_string().parse::<f64>().unwrap_or(0.0))
            }
        };

        let is_builtin = {
            let val = os.get_environment("TEST_IS_BUILTIN");
            if val.is_empty() {
                None
            } else {
                Some(val.to_string().parse::<i64>().unwrap_or(0) != 0)
            }
        };

        Self {
            duration,
            amount,
            is_builtin,
        }
    }
}
