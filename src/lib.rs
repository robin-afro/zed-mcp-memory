use const_format::formatcp;
use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed_extension_api::{
    Command, ContextServerConfiguration, ContextServerId, Project, Result, serde_json,
    settings::ContextServerSettings,
};

const NODE_PACKAGE_NAME: &str = "@modelcontextprotocol/server-memory";
const NODE_PACKAGE_VERSION: &str = "2026.1.26";
const NODE_SERVER_PATH: &str = formatcp!("node_modules/{}/dist/index.js", NODE_PACKAGE_NAME);
const CONTEXT_SERVER_ID: &str = "mcp-server-memory";

const CONTEXT_SERVER_DEFAULT_SETTINGS: &str = r#"{"memory_file_path": ""}"#;

#[derive(Debug, Deserialize, JsonSchema)]
struct MemoryContextServerSettings {
    /// Path to the memory file. Leave empty to use the default location
    /// (next to the server's index.js). Supports absolute paths on any OS.
    memory_file_path: String,
}

struct MemoryContextServerExtension;

impl zed_extension_api::Extension for MemoryContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        assert_eq!(
            context_server_id.as_ref(),
            CONTEXT_SERVER_ID,
            "Unexpected context server ID"
        );

        let version = zed_extension_api::npm_package_installed_version(NODE_PACKAGE_NAME)?;
        if version.as_deref() != Some(NODE_PACKAGE_VERSION) {
            zed_extension_api::npm_install_package(NODE_PACKAGE_NAME, NODE_PACKAGE_VERSION)?;
        }

        let mut env_vars: Vec<(String, String)> = Vec::new();

        if let Ok(settings) = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project) {
            if let Some(settings) = settings.settings {
                if let Ok(settings) =
                    serde_json::from_value::<MemoryContextServerSettings>(settings)
                {
                    if !settings.memory_file_path.is_empty() {
                        env_vars.push(("MEMORY_FILE_PATH".into(), settings.memory_file_path));
                    }
                }
            }
        }

        Ok(Command {
            command: zed_extension_api::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(NODE_SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
            ],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        assert_eq!(
            context_server_id.as_ref(),
            CONTEXT_SERVER_ID,
            "Unexpected context server ID"
        );

        let installation_instructions = include_str!("installation_instructions.md").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(MemoryContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            settings_schema,
            default_settings: CONTEXT_SERVER_DEFAULT_SETTINGS.to_string(),
        }))
    }
}

zed_extension_api::register_extension!(MemoryContextServerExtension);
