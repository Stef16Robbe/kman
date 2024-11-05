use serde::{Deserialize, Serialize};

// Created manually (and adapted to fit OC) using: https://pkg.go.dev/k8s.io/client-go/tools/clientcmd/api/v1#Config
/// KubeConfig holds the information needed to build connect to remote Kubernetes clusters as a given user
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KubeConfig {
    /// The api version
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// The kind
    pub kind: String,
    /// Clusters is a map of referable names to cluster configs
    pub clusters: Vec<NamedCluster>,
    /// Contexts is a map of referable names to context configs
    pub contexts: Vec<NamedContext>,
    /// CurrentContext is the name of the context that you would like to use by default
    #[serde(rename = "current-context")]
    pub current_context: String,
    /// Users is a map of referable users with their tokens
    pub users: Vec<NamedUser>,
}

/// NamedUser relates nicknames to user information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedUser {
    /// Name is the nickname for this User
    pub name: String,
    /// User holds the user information
    pub user: User,
}

/// User contains information on the authenticated user
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Token is the user's sha256 token
    pub token: String,
}

/// NamedCluster relates nicknames to cluster information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedCluster {
    /// Name is the nickname for this Cluster
    pub name: String,
    /// Cluster holds the cluster information
    pub cluster: Cluster,
}

/// Cluster contains information about how to communicate with a Kubernetes cluster
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    /// Server is the address of the Kubernetes cluster (https://hostname:port).
    pub server: String,
    /// TLSServerName is used to check server certificate. If TLSServerName is empty, the hostname used to contact the server is used.
    pub tls_server_name: Option<String>,
    /// InsecureSkipTLSVerify skips the validity check for the server's certificate. This will make your HTTPS connections insecure.
    pub insecure_skip_verify: Option<bool>,
    /// CertificateAuthority is the path to a cert file for the certificate authority.
    pub certificate_authority: Option<String>,
    /// CertificateAuthorityData contains PEM-encoded certificate authority certificates. Overrides CertificateAuthority
    pub certificate_authority_data: Option<Vec<u8>>,
    /// ProxyURL is the URL to the proxy to be used for all requests made by this client. URLs with "http", "https", and "socks5" schemes are supported. If this configuration is not provided or the empty string, the client attempts to construct a proxy configuration from http_proxy and https_proxy environment variables. If these environment variables are not set, the client does not attempt to proxy requests.
    ///
    /// socks5 proxying does not currently support spdy streaming endpoints (exec, attach, port forward).
    pub proxy_url: Option<String>,
    /// DisableCompression allows client to opt-out of response compression for all requests to the server. This is useful to speed up requests (specifically lists) when client-server network bandwidth is ample, by saving time on compression (server-side) and decompression (client-side): https://github.com/Kubernetes/Kubernetes/issues/112296.
    pub disable_compression: Option<bool>,
}

/// NamedContext relates nicknames to context information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedContext {
    /// Name is the nickname for this Context
    pub name: String,
    /// Context holds the context information
    pub context: ClusterContext,
}

/// Context is a tuple of references to a cluster (how do I communicate with a Kubernetes cluster), a user (how do I identify myself), and a namespace (what subset of resources do I want to work with)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterContext {
    /// Cluster is the name of the cluster for this context
    pub cluster: String,
    /// User is the user info for this context
    pub user: String,
    /// Namespace is the default namespace to use on unspecified requests
    pub namespace: Option<String>,
}
