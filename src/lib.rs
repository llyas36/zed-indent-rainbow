use zed_extension_api::{
    register_extension, Extension, LanguageServerId, Worktree, Result, Command,
};

struct IndentRainbowExtension;

impl Extension for IndentRainbowExtension {
    fn new() -> Self {
        IndentRainbowExtension
    }

    fn language_server_command(
        &mut self,
        _id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command, String> {
        // We don’t need a language server, so just return an error.
        Err("IndentRainbow doesn’t provide a language server".into())
    }
}

register_extension!(IndentRainbowExtension);
