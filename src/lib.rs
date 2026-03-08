use const_format::formatcp;
use std::env;

use zed_extension_api::{Command, ContextServerId, Project, Result};

const NODE_PACKAGE_NAME: &str = "@modelcontextprotocol/server-memory";
const NODE_PACKAGE_VERSION: &str = "2026.1.26";
const NODE_SERVER_PATH: &str = formatcp!("node_modules/{}/dist/index.js", NODE_PACKAGE_NAME);
const CONTEXT_SERVER_ID: &str = "mcp-server-memory";

struct MemoryContextServerExtension;

impl zed_extension_api::Extension for MemoryContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
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

        Ok(Command {
            command: zed_extension_api::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(NODE_SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed_extension_api::register_extension!(MemoryContextServerExtension);
