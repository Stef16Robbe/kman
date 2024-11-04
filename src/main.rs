use anyhow::{bail, Result};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clap::{command, Parser, Subcommand};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

// Created manually (and adapted to fit OC) using: https://pkg.go.dev/k8s.io/client-go/tools/clientcmd/api/v1#Config
/// KubeConfig holds the information needed to build connect to remote Kubernetes clusters as a given user
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KubeConfig {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    /// Clusters is a map of referable names to cluster configs
    clusters: Vec<NamedCluster>,
    /// Contexts is a map of referable names to context configs
    contexts: Vec<NamedContext>,
    /// CurrentContext is the name of the context that you would like to use by default
    #[serde(rename = "current-context")]
    current_context: String,
    /// Users is a map of referable users with their tokens
    users: Vec<NamedUser>,
}

/// NamedUser relates nicknames to user information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NamedUser {
    /// Name is the nickname for this User
    name: String,
    /// User holds the user information
    user: User,
}

/// User contains information on the authenticated user
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    /// Token is the user's sha256 token
    token: String,
}

/// NamedCluster relates nicknames to cluster information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NamedCluster {
    /// Name is the nickname for this Cluster
    name: String,
    /// Cluster holds the cluster information
    cluster: Cluster,
}

/// Cluster contains information about how to communicate with a Kubernetes cluster
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Cluster {
    /// Server is the address of the Kubernetes cluster (https://hostname:port).
    server: String,
    /// TLSServerName is used to check server certificate. If TLSServerName is empty, the hostname used to contact the server is used.
    tls_server_name: Option<String>,
    /// InsecureSkipTLSVerify skips the validity check for the server's certificate. This will make your HTTPS connections insecure.
    insecure_skip_verify: Option<bool>,
    /// CertificateAuthority is the path to a cert file for the certificate authority.
    certificate_authority: Option<String>,
    /// CertificateAuthorityData contains PEM-encoded certificate authority certificates. Overrides CertificateAuthority
    certificate_authority_data: Option<Vec<u8>>,
    /// ProxyURL is the URL to the proxy to be used for all requests made by this client. URLs with "http", "https", and "socks5" schemes are supported. If this configuration is not provided or the empty string, the client attempts to construct a proxy configuration from http_proxy and https_proxy environment variables. If these environment variables are not set, the client does not attempt to proxy requests.
    ///
    /// socks5 proxying does not currently support spdy streaming endpoints (exec, attach, port forward).
    proxy_url: Option<String>,
    /// DisableCompression allows client to opt-out of response compression for all requests to the server. This is useful to speed up requests (specifically lists) when client-server network bandwidth is ample, by saving time on compression (server-side) and decompression (client-side): https://github.com/Kubernetes/Kubernetes/issues/112296.
    disable_compression: Option<bool>,
}

/// NamedContext relates nicknames to context information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NamedContext {
    /// Name is the nickname for this Context
    name: String,
    /// Context holds the context information
    context: Context,
}

/// Context is a tuple of references to a cluster (how do I communicate with a Kubernetes cluster), a user (how do I identify myself), and a namespace (what subset of resources do I want to work with)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Context {
    /// Cluster is the name of the cluster for this context
    cluster: String,
    /// User is the user info for this context
    user: String,
    /// Namespace is the default namespace to use on unspecified requests
    namespace: Option<String>,
}

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
        // The context name
        #[clap(short, long)]
        name: String,
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
                out.push_str("* ");
            }
            out.push_str(&ctx.name);
            out.push('\n');
        }

        Ok(out)
    }

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

        Ok(())
    }

    fn update_kubeconfig(&self, kubeconfig_location: &PathBuf) -> Result<()> {
        let yaml = serde_yml::to_string(&self.kubeconfig)?;
        let mut kubeconfig = File::create(kubeconfig_location)?;
        kubeconfig.write_all(yaml.as_bytes())?;
        Ok(())
    }

    fn load_kubeconfig(kubeconfig_location: &PathBuf) -> Result<KubeConfig> {
        let kubeconfig_str = std::fs::read_to_string(kubeconfig_location)?;
        let kubeconfig: KubeConfig = serde_yml::from_str(&kubeconfig_str)?;
        Ok(kubeconfig)
    }
}

fn main() -> Result<()> {
    // cluster > context > user
    // for now, we assume that clusters:contexts:users is a 1:1:1 ratio
    // just tell users to delete their kubeconfig lol?
    // assuming a "perfect" kubeconfig, all this CLI has to do is update the token
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
                print!("{}", kman.list_contexts().unwrap());
            }
            Commands::Select { name } => {
                kman.select_context(name).unwrap();
            }
        }
    }

    kman.update_kubeconfig(&kubeconfig_location)?;
    Ok(())
}