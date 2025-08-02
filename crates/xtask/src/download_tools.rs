use camino::Utf8Path;
use xshell::{Shell, cmd};

type ExtractFn = Box<dyn Fn(&Shell, &Tool) -> Result<(), xshell::Error>>;

struct Tool {
    download_link: &'static str,
    download_filename: &'static str,
    bin_name: &'static str,
    sha256sum: &'static str,
    extract_func: ExtractFn,
}

impl Tool {
    pub fn download(&self, sh: &Shell, bin_dir: &Utf8Path) -> Result<(), xshell::Error> {
        let Tool {
            download_link,
            download_filename,
            bin_name,
            extract_func,
            sha256sum,
        } = self;
        let bin_path = bin_dir.join(bin_name);
        if bin_path.exists() {
            return Ok(());
        }
        tracing::info!("Downloading {bin_name} to {bin_path}");

        let tmp_dir = sh.create_temp_dir()?;
        let dl_path = tmp_dir.path().join(download_filename);
        sh.change_dir(tmp_dir.path());

        // download
        cmd!(sh, "curl -sSfL {download_link} --output {dl_path}").run()?;
        sh.write_file("checksum", format!("{sha256sum} {download_filename}"))?;
        cmd!(sh, "sha256sum --check checksum").run()?;

        // extract
        extract_func(sh, self)?;

        // move to bin dir
        cmd!(sh, "mv {bin_name} {bin_path}").run()?;

        Ok(())
    }
}

fn tool_list() -> [Tool; 3] {
    [
        Tool {
            download_link: "https://github.com/EmbarkStudios/cargo-deny/releases/download/0.18.3/cargo-deny-0.18.3-x86_64-unknown-linux-musl.tar.gz",
            download_filename: "cargo-deny-0.18.3-x86_64-unknown-linux-musl.tar.gz",
            sha256sum: "5037f3c167a8da8cea04c34a89e74cef95c646f2a537750b2db58e54f6a788e7",
            bin_name: "cargo-deny",
            extract_func: Box::new(|sh, t| {
                tar_extract(sh, t, "cargo-deny-0.18.3-x86_64-unknown-linux-musl/cargo-deny")
            }),
        },
        Tool {
            download_link: "https://github.com/DioxusLabs/dioxus/releases/download/v0.6.3/dx-x86_64-unknown-linux-gnu-v0.6.3.tar.gz",
            download_filename: "dx-x86_64-unknown-linux-gnu-v0.6.3.tar.gz",
            sha256sum: "2d2e205bad9715141019ec558e19874d3922c7803656e98ba4518c18a0e22196",
            bin_name: "dx",
            extract_func: Box::new(|sh, t| {
                let download_filename = t.download_filename;
                cmd!(sh, "tar -zxvf {download_filename}").run()
            }),
        },
        Tool {
            download_link: "https://github.com/bnjbvr/cargo-machete/releases/download/v0.8.0/cargo-machete-v0.8.0-x86_64-unknown-linux-musl.tar.gz",
            download_filename: "cargo-machete-v0.8.0-x86_64-unknown-linux-musl.tar.gz",
            sha256sum: "020f6608f9be1562d1fb601e5808e541ccb8806e7e4dfde27d48bc22254a002c",
            bin_name: "cargo-machete",
            extract_func: Box::new(|sh, t| {
                tar_extract(sh, t, "cargo-machete-v0.8.0-x86_64-unknown-linux-musl/cargo-machete")
            }),
        },
    ]
}

fn tar_extract(sh: &Shell, t: &Tool, tar_path: &str) -> Result<(), xshell::Error> {
    let Tool {
        download_filename,
        bin_name,
        ..
    } = t;
    cmd!(sh, "tar -zxvf {download_filename}").run()?;
    cmd!(sh, "mv {tar_path} {bin_name}").run()?;
    Ok(())
}

pub fn download_all_tools(sh: &Shell) -> Result<(), xshell::Error> {
    let pwd = cmd!(sh, "pwd").read()?;
    let bin_dir = Utf8Path::new(&pwd).join("bin");
    if !bin_dir.exists() {
        sh.create_dir(&bin_dir)?;
    }

    for tool in tool_list() {
        tool.download(sh, &bin_dir)?;
        sh.change_dir(&pwd);
    }
    Ok(())
}
