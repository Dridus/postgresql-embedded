use crate::traits::CommandBuilder;
use crate::Settings;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// vacuumlo removes unreferenced large objects from databases.
#[derive(Clone, Debug, Default)]
pub struct VacuumLoBuilder {
    program_dir: Option<PathBuf>,
    limit: Option<usize>,
    dry_run: bool,
    verbose: bool,
    version: bool,
    help: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    pg_password: Option<OsString>,
}

impl VacuumLoBuilder {
    /// Create a new [VacuumLoBuilder]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [VacuumLoBuilder] from [Settings]
    pub fn from(settings: &dyn Settings) -> Self {
        Self::new()
            .program_dir(settings.get_binary_dir())
            .host(settings.get_host())
            .port(settings.get_port())
            .username(settings.get_username())
            .pg_password(settings.get_password())
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// commit after removing each LIMIT large objects
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// don't remove large objects, just show what would be done
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// write a lot of progress messages
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }

    /// database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// user name to connect as
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// user password
    pub fn pg_password<S: AsRef<OsStr>>(mut self, pg_password: S) -> Self {
        self.pg_password = Some(pg_password.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for VacuumLoBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "vacuumlo".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(limit) = &self.limit {
            args.push("--limit".into());
            args.push(limit.to_string().into());
        }

        if self.dry_run {
            args.push("--dry-run".into());
        }

        if self.verbose {
            args.push("--verbose".into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        args
    }

    /// Get the environment variables for the command
    fn get_envs(&self) -> Vec<(OsString, OsString)> {
        let mut envs: Vec<(OsString, OsString)> = Vec::new();

        if let Some(password) = &self.pg_password {
            envs.push(("PGPASSWORD".into(), password.into()));
        }

        envs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::CommandToString;
    use crate::TestSettings;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = VacuumLoBuilder::new().program_dir(".").build();
        assert_eq!(
            PathBuf::from(".").join("vacuumlo"),
            PathBuf::from(command.to_command_string().replace('"', ""))
        );
    }

    #[test]
    fn test_builder_from() {
        let command = VacuumLoBuilder::from(&TestSettings).build();
        assert_eq!(
            r#"PGPASSWORD="password" "./vacuumlo" "--host" "localhost" "--port" "5432" "--username" "postgres""#,
            command.to_command_string()
        )
    }

    #[test]
    fn test_builder() {
        let command = VacuumLoBuilder::new()
            .limit(100)
            .dry_run()
            .verbose()
            .version()
            .help()
            .host("localhost")
            .port(5432)
            .username("postgres")
            .no_password()
            .password()
            .pg_password("password")
            .build();

        assert_eq!(
            r#"PGPASSWORD="password" "vacuumlo" "--limit" "100" "--dry-run" "--verbose" "--version" "--help" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password""#,
            command.to_command_string()
        );
    }
}
