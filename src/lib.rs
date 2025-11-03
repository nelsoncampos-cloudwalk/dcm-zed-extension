mod commands;
mod config;
mod language_server;
mod utils;

use config::ResolvedSettings;
use zed_extension_api::{self as zed, LanguageServerId, SlashCommand, Worktree};

const LANGUAGE_SERVER_ID: &str = "dcm";

struct DcmExtension;

impl zed::Extension for DcmExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        ensure_supported_language_server(language_server_id)?;

        let settings = ResolvedSettings::from_worktree(worktree)?;
        language_server::build_command(&settings)
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        ensure_supported_language_server(language_server_id)?;

        let settings = ResolvedSettings::from_worktree(worktree)?;
        Ok(Some(language_server::initialization_options(&settings)))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        ensure_supported_language_server(language_server_id)?;

        let settings = ResolvedSettings::from_worktree(worktree)?;
        Ok(Some(language_server::workspace_configuration(&settings)?))
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> zed::Result<Vec<zed::SlashCommandArgumentCompletion>> {
        if command.name == commands::DCM_SLASH_COMMAND {
            Ok(commands::complete(&args))
        } else {
            Ok(Vec::new())
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> zed::Result<zed::SlashCommandOutput> {
        if command.name != commands::DCM_SLASH_COMMAND {
            return Err(format!(
                "Unsupported slash command `{}` for DCM extension",
                command.name
            ));
        }

        commands::run(args, worktree)
    }
}

fn ensure_supported_language_server(language_server_id: &LanguageServerId) -> zed::Result<()> {
    if language_server_id.as_ref() == LANGUAGE_SERVER_ID {
        Ok(())
    } else {
        Err(format!(
            "Unsupported language server id `{language_server_id}` for DCM extension"
        ))
    }
}

zed::register_extension!(DcmExtension);
