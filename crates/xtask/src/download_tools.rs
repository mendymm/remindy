use camino::Utf8Path;
use xshell::{Shell, cmd};

const DIOXUS_VERSION: &str = "0.6.3";
const CARGO_MACHETE_VERSION: &str = "0.8.0";
const CARGO_DENY_VERSION: &str = "0.18.3";
const CARGO_BINSTALL_VERSION: &str = "1.14.2";

pub fn download_all_tools(sh: &Shell) -> Result<(), xshell::Error> {
    // install dioxus cli
    let pwd = cmd!(sh, "pwd").read()?;
    install_cargo_binstall(sh, &pwd)?;
    install_dioxus_cli(sh, &pwd)?;
    install_cargo_machete(sh, &pwd)?;
    install_cargo_deny(sh, &pwd)?;

    Ok(())
}

fn install_cargo_deny(sh: &Shell, pwd: &str) -> Result<(), xshell::Error> {
    if sh.path_exists("./bin/cargo-deny") {
        let version = cmd!(sh, "./bin/cargo-deny --version").read().unwrap();
        if version == format!("cargo-deny {CARGO_DENY_VERSION}") {
            tracing::debug!("Found installed cargo-deny cli, version output: {version}");
            return Ok(());
        }
    }
    binstall_cli_tool(sh, pwd, "cargo-deny", "cargo-deny", CARGO_DENY_VERSION)?;
    Ok(())
}

fn install_cargo_machete(sh: &Shell, pwd: &str) -> Result<(), xshell::Error> {
    if sh.path_exists("./bin/cargo-machete") {
        let version = cmd!(sh, "./bin/cargo-machete --version").read().unwrap();
        if version == CARGO_MACHETE_VERSION {
            tracing::debug!("Found installed cargo-machete cli, version output: {version}");
            return Ok(());
        }
    }
    binstall_cli_tool(
        sh,
        pwd,
        "cargo-machete",
        "cargo-machete",
        CARGO_MACHETE_VERSION,
    )?;
    Ok(())
}

fn install_dioxus_cli(sh: &Shell, pwd: &str) -> Result<(), xshell::Error> {
    if sh.path_exists("./bin/dx") {
        let version = cmd!(sh, "./bin/dx -V").read().unwrap();
        if version.starts_with(&format!("dioxus {DIOXUS_VERSION}")) {
            tracing::debug!("Found installed dioxus cli, version output: {version}");
            return Ok(());
        }
    }
    binstall_cli_tool(sh, pwd, "dioxus-cli", "dx", DIOXUS_VERSION)?;

    Ok(())
}

fn binstall_cli_tool(
    sh: &Shell,
    pwd: &str,
    package: &str,
    bin: &str,
    version: &str,
) -> Result<(), xshell::Error> {
    tracing::info!("Downloading {package}@{version} to ./bin/{bin}");

    cmd!(sh, "./bin/cargo-binstall --no-confirm --no-discover-github-token --disable-telemetry --no-track --root {pwd} {package}@{version}").run()?;

    Ok(())
}

fn install_cargo_binstall(sh: &Shell, pwd: &str) -> Result<(), xshell::Error> {
    if sh.path_exists("./bin/cargo-binstall") {
        let version = cmd!(sh, "./bin/cargo-binstall -V").read().unwrap();
        if version == CARGO_BINSTALL_VERSION {
            tracing::debug!("Found installed cargo-binstall cli, version output: {version}");
            return Ok(());
        }
    }
    let orig_cwd = Utf8Path::new(pwd);
    if !orig_cwd.join("bin").exists() {
        sh.create_dir(orig_cwd.join("bin"))?;
    }
    let binstall_path = orig_cwd.join("bin").join("cargo-binstall");
    let download_link = "https://github.com/cargo-bins/cargo-binstall/releases/download/v1.14.2/cargo-binstall-x86_64-unknown-linux-gnu.tgz";
    let sha256sum = "921d826fcce861a8d986e7c0d5fbf5e5fed8707d8fa574f98486e7537c37cf16";

    let tmp = sh.create_temp_dir()?;
    sh.change_dir(tmp.path());
    cmd!(
        sh,
        "curl --location --output cargo-binstall-x86_64-unknown-linux-gnu.tgz {download_link}"
    )
    .run()?;
    cmd!(
        sh,
        "echo {sha256sum}  cargo-binstall-x86_64-unknown-linux-gnu.tgz | sha256sum -c"
    )
    .run()?;
    cmd!(sh, "tar -xf cargo-binstall-x86_64-unknown-linux-gnu.tgz").run()?;

    cmd!(sh, "mv cargo-binstall {binstall_path}").run()?;

    sh.change_dir(orig_cwd);
    Ok(())
}
