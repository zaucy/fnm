use crate::version_file_strategy::VersionFileStrategy;

use super::shell::Shell;
use indoc::formatdoc;
use std::path::Path;

#[derive(Debug)]
pub struct Nushell;

impl Shell for Nushell {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        panic!("Shell completion is not supported for Nushell (yet.) See https://github.com/clap-rs/clap/issues/2778 for updates.");
    }

    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let path = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Can't convert path to string"))?;

        if std::env::consts::OS == "windows" {
            Ok(format!("let-env Path = ($env.Path | prepend {:?})", path))
        } else {
            Ok(format!("let-env PATH = ($env.PATH | prepend {:?})", path))
        }
    }

    fn preferred_file_extension(&self) -> &'static str {
        ".nu"
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("let-env {} = {:?}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::FnmConfig) -> anyhow::Result<String> {
        Ok(match config.version_file_strategy() {
            VersionFileStrategy::Local => formatdoc!(
                r#"
                    let-env config = ($env.config | upsert hooks.env_change.PWD {{
                        [
                            {{
                                condition: {{|_, after| (($after | path join .node-version | path exists) || ($after | path join .nvmrc | path exists)) }}
                                code: "fnm use --silent-if-unchanged"
                            }}
                        ]
                    }})
                "#
            ),
            VersionFileStrategy::Recursive => formatdoc!(
                r#"
                    let-env config = ($env.config | upsert hooks.env_change.PWD {{
                        [
                            {{
                                code: "fnm use --silent-if-unchanged"
                            }}
                        ]
                    }})
                "#
            ),
        })
    }
}
