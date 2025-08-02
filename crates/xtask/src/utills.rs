use camino::Utf8PathBuf;
use xshell::{Shell, cmd};

pub fn initialize_shell() -> Result<Shell, xshell::Error> {
    let sh = Shell::new()?;
    let workspace_manifest =
        Utf8PathBuf::from(cmd!(sh, "cargo locate-project --workspace --message-format=plain").read()?);
    let workspace_dir = workspace_manifest.parent().unwrap();
    sh.change_dir(workspace_dir);
    Ok(sh)
}
