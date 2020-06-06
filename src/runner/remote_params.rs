
use clap::ArgMatches;

#[derive(Debug, Default)]
pub(super) struct RemoteParams {
    pub(super) hostname: String,
    pub(super) username: Option<String>,
    pub(super) port: Option<u16>,
    pub(super) syncroot: String,
}

impl RemoteParams {
    pub(super) fn from_args(args: &ArgMatches) -> Self {
        RemoteParams::new()
            .with_hostname(args.value_of("host"))
            .with_username(args.value_of("user"))
            .with_port(args.value_of("port"))
            .with_syncroot(args.value_of("syncroot"))
    }
    fn new() -> Self {
        RemoteParams {
            ..Default::default()
        }
    }
    fn with_hostname(mut self, hostname: Option<&str>) -> Self {
        let hostname = hostname.unwrap_or("localhost").to_string();
        self.hostname = hostname;
        self
    }
    fn with_username(mut self, username: Option<&str>) -> Self {
        self.username = username.and_then(|x| Some(String::from(x)));
        self
    }
    fn with_port(mut self, port: Option<&str>) -> Self {
        self.port = port.and_then(|x| x.parse().ok());
        self
    }
    fn with_syncroot(mut self, syncroot: Option<&str>) -> Self {
        self.syncroot = syncroot.unwrap_or("~/.yaggy").to_string();
        self
    }

}
