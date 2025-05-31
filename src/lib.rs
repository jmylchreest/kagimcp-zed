use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

#[derive(Debug, Deserialize, JsonSchema)]
struct KagiContextServerSettings {
    kagi_api_key: String,
    #[serde(default)]
    kagi_summarizer_engine: Option<String>,
}

struct KagiModelContextExtension;

impl zed::Extension for KagiModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("kagimcp", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `kagi_api_key` setting".into());
        };
        let settings: KagiContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        let mut env = vec![("KAGI_API_KEY".into(), settings.kagi_api_key)];
        
        if let Some(engine) = settings.kagi_summarizer_engine {
            env.push(("KAGI_SUMMARIZER_ENGINE".into(), engine));
        }

        Ok(Command {
            command: "uvx".to_string(),
            args: vec!["kagimcp".to_string()],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(KagiContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(KagiModelContextExtension);