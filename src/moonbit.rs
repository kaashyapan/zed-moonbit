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
        let shell_env = worktree.shell_env();

        let home = shell_env
            .iter()
            .find(|(k, _)| k == "HOME")
            .map(|(_, v)| v.clone())
            .ok_or("HOME not found in shell environment")?;

        let command = format!("{home}/.moon/bin/moon-lsp");

        let moon_bin = format!("{home}/.moon/bin");
        let env = prepend_to_path(shell_env, &moon_bin);

        Ok(zed::Command {
            command,
            args: vec!["--stdio".to_string()],
            env,
        })
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

fn prepend_to_path(mut env: Vec<(String, String)>, dir: &str) -> Vec<(String, String)> {
    if let Some((_, path)) = env.iter_mut().find(|(k, _)| k == "PATH") {
        *path = format!("{dir}:{path}");
    } else {
        env.push(("PATH".to_string(), dir.to_string()));
    }
    env
}

zed::register_extension!(MoonBitExtension);
