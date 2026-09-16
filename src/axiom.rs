//! Zed extension for Axiom. Resolves and launches the `axiom` CLI, whose
//! `lsp` subcommand starts the language server over stdin/stdout.
//!
//! The language server is *not* shipped inside the extension: Zed extensions
//! must not bundle language servers. The binary is looked up on the worktree
//! `$PATH`; `AXIOM_BIN` may point at a specific build (e.g.
//! `target/debug/axiom`) to avoid installing it.

use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct AxiomExtension;

impl zed::Extension for AxiomExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = std::env::var("AXIOM_BIN")
            .ok()
            .filter(|p| !p.is_empty())
            .or_else(|| worktree.which("axiom"))
            .ok_or_else(|| {
                "axiom not found on PATH. Install it with `cargo install --path crates/axiom-cli` \
                 or set the AXIOM_BIN environment variable to the binary path."
                    .to_string()
            })?;

        Ok(zed::Command {
            command: binary,
            args: vec!["lsp".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(AxiomExtension);