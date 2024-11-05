use anyhow::{bail, Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input};
use kubeconfig::KubeConfig;
use regex::Regex;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clap::{command, Parser, Subcommand};
use directories::BaseDirs;

mod kubeconfig;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists all contexts
    List {},
    /// Select context (cluster) to use
    Select {
        /// The context name
        #[clap(short, long)]
        name: String,
    },
    /// Refresh context (cluster) credentials
    Refresh {
        /// The context name
        #[clap(short, long)]
        name: Option<String>,
        // TODO: add `--all` option
    },
}

struct Kman {
    kubeconfig: KubeConfig,
}

impl Kman {
    fn new(kubeconfig: KubeConfig) -> Self {
        Self { kubeconfig }
    }

    fn list_contexts(&self) -> Result<String> {
        let mut out = String::new();

        for ctx in &self.kubeconfig.contexts {
            if ctx.name == self.kubeconfig.current_context {
                out.push_str(&format!("{}", "* ".green().bold()));
                out.push_str(&format!("{}", ctx.name.green()));
            } else {
                out.push_str(&ctx.name);
            }
            out.push('\n');
        }

        Ok(out)
    }

    // TODO: add auto-check for expired credentials
    fn select_context(&mut self, name: String) -> Result<()> {
        let mut found = false;
        for ctx in &self.kubeconfig.contexts {
            if ctx.name == name {
                self.kubeconfig.current_context = name.clone();
                found = true;
            }
        }

        if !found {
            bail!("Context does not exist");
        }

        println!("Now using context: {}", name.green().bold());

        Ok(())
    }

    fn update_kubeconfig(&self, kubeconfig_location: &PathBuf) -> Result<()> {
        let yaml = serde_yml::to_string(&self.kubeconfig)?;
        let mut kubeconfig = File::create(kubeconfig_location)?;
        kubeconfig.write_all(yaml.as_bytes())?;

        Ok(())
    }

    fn get_user_from_context_name(&self, context_name: String) -> String {
        self.kubeconfig
            .contexts
            .iter()
            .find(|c| c.name == context_name)
            .unwrap()
            .context
            .user
            .clone()
    }

    fn update_token(&mut self, name: Option<String>) -> Result<()> {
        let context_to_update = if let Some(name) = name {
            name
        } else {
            self.kubeconfig.current_context.clone()
        };

        let user = self.get_user_from_context_name(context_to_update);

        let token: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Request a token (sha256~xxx...) in the console and paste it in here:")
            .interact_text()?;

        let pattern = r"sha256~[A-Za-z0-9\-\_+\/=]{43}";
        let re = Regex::new(pattern).context("failed to compile regex")?;

        if re.is_match(&token) {
            for u in &mut self.kubeconfig.users {
                if u.name == user {
                    u.user.token = token;
                    break;
                }
            }
        } else {
            bail!("Incorrect token given");
        }

        println!("{}", "Token updated succesfully!".green().bold());

        Ok(())
    }

    fn load_kubeconfig(kubeconfig_location: &PathBuf) -> Result<KubeConfig> {
        let kubeconfig_str = std::fs::read_to_string(kubeconfig_location)?;
        let kubeconfig: KubeConfig = serde_yml::from_str(&kubeconfig_str)?;
        Ok(kubeconfig)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let base_dirs = BaseDirs::new().unwrap();
    // for now assuming we always use $HOME/.kube/
    let kubeconfig_location = base_dirs.home_dir().join(Path::new(".kube/config"));
    let kubeconfig = Kman::load_kubeconfig(&kubeconfig_location).unwrap();
    let mut kman = Kman::new(kubeconfig);

    if let Some(command) = cli.command {
        match command {
            Commands::List {} => {
                let contexts = kman.list_contexts().unwrap();
                println!(
                    "{}\n\n{}",
                    "Here are your current contexts:".bold(),
                    contexts
                );
            }
            Commands::Select { name } => {
                kman.select_context(name).unwrap();
            }
            Commands::Refresh { name } => kman.update_token(name).unwrap(),
        }

        kman.update_kubeconfig(&kubeconfig_location)?;
    }

    Ok(())
}
