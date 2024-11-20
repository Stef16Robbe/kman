use anyhow::{bail, Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use human_panic::{setup_panic, Metadata};
use kubeconfig::KubeConfig;
use regex::Regex;
use roxygen::roxygen;
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
    /// Select context to use
    Select {
        /// The context name
        name: Option<String>,
    },
    /// Refresh context token
    Refresh {
        /// The context name
        #[clap(short, long)]
        name: Option<String>,
        // TODO: add `--all` option
    },
}

/// Struct used for state management
struct Kman {
    /// The parsed kubeconfig from the user's home directory
    kubeconfig: KubeConfig,
}

impl Kman {
    #[roxygen]
    /// Create a new instance of [Kman]
    fn new(
        /// The kubeconfig loaded from disk
        kubeconfig: KubeConfig,
    ) -> Self {
        Self { kubeconfig }
    }

    fn get_all_contexts(&self) -> Vec<String> {
        self.kubeconfig
            .contexts
            .iter()
            .map(|c| c.name.clone())
            .collect()
    }

    /// This function prints out the current existing contexts,
    /// and shows which is active
    fn list_contexts(&self) -> Result<String> {
        if self.kubeconfig.contexts.is_empty() {
            bail!("Kubeconfig does not contain any contexts");
        }

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

    #[roxygen]
    /// Updates the kubeconfig's current-context to the given context name
    // TODO: add auto-check for expired credentials
    fn select_context(
        &mut self,
        /// The context name to use
        context_name: String,
    ) -> Result<()> {
        let mut found = false;
        for ctx in &self.kubeconfig.contexts {
            if ctx.name == context_name {
                self.kubeconfig.current_context = context_name.clone();
                found = true;
            }
        }

        if !found {
            bail!("Given context does not exist");
        }

        println!("Now using context: {}", context_name.green().bold());

        Ok(())
    }

    #[roxygen]
    /// Overwrite the user's kubeconfig with an updated one
    fn update_kubeconfig(
        &self,
        /// The location of the kubeconfig to override
        kubeconfig_location: &PathBuf,
    ) -> Result<()> {
        let yaml = serde_yml::to_string(&self.kubeconfig)?;
        let mut kubeconfig = File::create(kubeconfig_location)?;
        kubeconfig.write_all(yaml.as_bytes())?;

        Ok(())
    }

    #[roxygen]
    /// Get a "user" from the given context name
    fn get_user_from_context_name(
        &self,
        /// The context name to use
        context_name: String,
    ) -> Result<String> {
        Ok(self
            .kubeconfig
            .contexts
            .iter()
            .find(|c| c.name == context_name)
            .context("Given context does not exist")?
            .context
            .user
            .clone())
    }

    #[roxygen]
    /// Update a context token based on user input
    fn update_token(
        &mut self,
        /// The context name to use
        context_name: Option<String>,
    ) -> Result<()> {
        let context_to_update = context_name.unwrap_or(self.kubeconfig.current_context.clone());

        let user = self.get_user_from_context_name(context_to_update)?;

        let token: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Request a token (sha256~xxx...) in the console and paste it in here:")
            .interact_text()?;

        let token_regex =
            Regex::new(r"^sha256~[a-zA-Z0-9_-]{43}$").context("failed to compile regex")?;

        if token_regex.is_match(&token) {
            for u in &mut self.kubeconfig.users {
                if u.name == user {
                    u.user.token = token;
                    break;
                }
            }
        } else {
            bail!("Incorrect token given. A token looks like this: `sha256~re5x9PB4OYjn7BLUubSiWkHBYg6QdyflL1-4jcIJvmQ`");
        }

        println!("{}", "Token updated succesfully!".green().bold());

        Ok(())
    }

    #[roxygen]
    /// Load a kubeconfig from disk
    fn load_kubeconfig(
        /// The kubeconfig file location
        kubeconfig_location: &PathBuf,
    ) -> Result<KubeConfig> {
        let kubeconfig_str = std::fs::read_to_string(kubeconfig_location)
            .context("Could not read kubeconfig file")?;
        let kubeconfig: KubeConfig =
            serde_yml::from_str(&kubeconfig_str).context("Given file is not a valid Kubeconfig")?;
        Ok(kubeconfig)
    }
}

fn main() -> Result<()> {
    setup_panic!(
        Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors("Stef Robbe <stef.robbe@protonmail.com>")
            .homepage("https://github.com/stef16robbe/kman")
            .support("- Open an issue on the Github repo")
    );

    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    // TODO: remove `unwrap()`
    let base_dirs = BaseDirs::new().unwrap();
    let kubeconfig_location = std::env::var("KUBECONFIG")
        .map(|v| v.into())
        .unwrap_or_else(|_| base_dirs.home_dir().join(Path::new(".kube/config")));

    if !kubeconfig_location.exists() {
        bail!(
            "No file found at: {}\nYou can specify a custom location with the `KUBECONFIG` environment variable",
            // TODO: remove `unwrap()`
            kubeconfig_location.to_str().unwrap()
        );
    }

    let kubeconfig = Kman::load_kubeconfig(&kubeconfig_location)?;
    let mut kman = Kman::new(kubeconfig);

    if let Some(command) = cli.command {
        match command {
            Commands::List {} => {
                let contexts = kman.list_contexts()?;
                println!(
                    "{}\n\n{}",
                    "Here are your current contexts:".bold(),
                    contexts
                );
            }
            Commands::Select { name } => {
                let context_to_select = if let Some(name) = name {
                    name
                } else {
                    // TODO: highlight current context in this menu
                    let contexts = kman.get_all_contexts();
                    let selected_index = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Pick the context you want to use")
                        .default(0)
                        .items(&contexts)
                        .interact()?;

                    contexts[selected_index].clone()
                };

                kman.select_context(context_to_select)?;
            }
            Commands::Refresh { name } => kman.update_token(name)?,
        }

        kman.update_kubeconfig(&kubeconfig_location)?;
    }

    Ok(())
}
