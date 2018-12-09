use std::time::Duration;
use std::process::Command;
use chan::Sender;

use block::{Block, ConfigBlock};
use config::Config;
use de::deserialize_duration;
use errors::*;
use widgets::text::TextWidget;
use widget::I3BarWidget;
use input::I3BarEvent;
use scheduler::Task;

use uuid::Uuid;

pub struct Language {
    current_language: TextWidget,
    id: String,
    update_interval: Duration,

    //useful, but optional
    #[allow(dead_code)]
    config: Config,
    #[allow(dead_code)]
    tx_update_request: Sender<Task>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct LanguageConfig {
    /// Update interval in seconds
    #[serde(default = "LanguageConfig::default_interval", deserialize_with = "deserialize_duration")]
    pub interval: Duration,
}

impl LanguageConfig {
    fn default_interval() -> Duration {
        Duration::from_secs(1)
    }
}

impl ConfigBlock for Language {
    type Config = LanguageConfig;

    fn new(block_config: Self::Config, config: Config, tx_update_request: Sender<Task>) -> Result<Self> {
        Ok(Language {
            id: Uuid::new_v4().simple().to_string(),
            update_interval: block_config.interval,
            current_language: TextWidget::new(config.clone()).with_text("Language").with_icon("keyboard"),
            tx_update_request,
            config,
        })
    }
}

impl Block for Language {
    fn update(&mut self) -> Result<Option<Duration>> {
        let language = Command::new("sh")    
        .args(
            &[
                "-c"
                ,"/home/ed/.config/i3status-rust/input_layout.sh"
            ]
        )
        .output()
        .block_error("language","Error getting language from script")
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|e| e.description().to_owned());

        self.current_language.set_text(language);
        Ok(Some(self.update_interval))
    }

    fn view(&self) -> Vec<&I3BarWidget> {
        vec![&self.current_language]
    }

    fn click(&mut self, _: &I3BarEvent) -> Result<()> {
        Ok(())
    }

    fn id(&self) -> &str {
        &self.id
    }
}
