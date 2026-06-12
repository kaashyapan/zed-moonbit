use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result, Worktree};

struct MoonBitExtension;

impl zed::Extension for MoonBitExtension {
    fn new() -> Self {
        MoonBitExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which("moon-lsp");
        if let Some(command) = path {
            return Ok(zed::Command {
                command,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        } else {
            return Err(
                "The binary 'moon-lsp' was not found in your PATH.\n\
                Verify that the installation PATH is correctly exposed to Zed's startup environment.".to_string()
            );
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("moon-lsp", worktree)
            .ok()
            .and_then(|s| s.settings.clone())
            .unwrap_or(zed::serde_json::json!({}));

        Ok(Some(settings))
    }
}

zed::register_extension!(MoonBitExtension);
