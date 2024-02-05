use crate::command::traits::CommandBuilder;
use std::convert::AsRef;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_restore restores a PostgreSQL database from an archive created by pg_dump.
#[derive(Clone, Debug, Default)]
pub struct PgRestoreBuilder {
    program_dir: Option<PathBuf>,
    dbname: Option<OsString>,
    file: Option<OsString>,
    format: Option<OsString>,
    list: bool,
    verbose: bool,
    version: bool,
    help: bool,
    data_only: bool,
    clean: bool,
    create: bool,
    exit_on_error: bool,
    index: Option<OsString>,
    jobs: Option<OsString>,
    use_list: Option<OsString>,
    schema: Option<OsString>,
    exclude_schema: Option<OsString>,
    no_owner: bool,
    function: Option<OsString>,
    schema_only: bool,
    superuser: Option<OsString>,
    table: Option<OsString>,
    trigger: Option<OsString>,
    no_privileges: bool,
    single_transaction: bool,
    disable_triggers: bool,
    enable_row_security: bool,
    if_exists: bool,
    no_comments: bool,
    no_data_for_failed_tables: bool,
    no_publications: bool,
    no_security_labels: bool,
    no_subscriptions: bool,
    no_table_access_method: bool,
    no_tablespaces: bool,
    section: Option<OsString>,
    strict_names: bool,
    use_set_session_authorization: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
    role: Option<OsString>,
}

impl PgRestoreBuilder {
    /// Create a new [`PgRestoreBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// connect to database name
    pub fn dbname<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.dbname = Some(name.as_ref().to_os_string());
        self
    }

    /// output file name (- for stdout)
    pub fn file<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.file = Some(filename.as_ref().to_os_string());
        self
    }

    /// backup file format (should be automatic)
    pub fn format<S: AsRef<OsStr>>(mut self, format: S) -> Self {
        self.format = Some(format.as_ref().to_os_string());
        self
    }

    /// print summarized TOC of the archive
    pub fn list(mut self) -> Self {
        self.list = true;
        self
    }

    /// verbose mode
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

    /// restore only the data, no schema
    pub fn data_only(mut self) -> Self {
        self.data_only = true;
        self
    }

    /// clean (drop) database objects before recreating
    pub fn clean(mut self) -> Self {
        self.clean = true;
        self
    }

    /// create the target database
    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }

    /// exit on error, default is to continue
    pub fn exit_on_error(mut self) -> Self {
        self.exit_on_error = true;
        self
    }

    /// restore named index
    pub fn index<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.index = Some(name.as_ref().to_os_string());
        self
    }

    /// use this many parallel jobs to restore
    pub fn jobs<S: AsRef<OsStr>>(mut self, num: S) -> Self {
        self.jobs = Some(num.as_ref().to_os_string());
        self
    }

    /// use table of contents from this file for selecting/ordering output
    pub fn use_list<S: AsRef<OsStr>>(mut self, filename: S) -> Self {
        self.use_list = Some(filename.as_ref().to_os_string());
        self
    }

    /// restore only objects in this schema
    pub fn schema<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.schema = Some(name.as_ref().to_os_string());
        self
    }

    /// do not restore objects in this schema
    pub fn exclude_schema<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.exclude_schema = Some(name.as_ref().to_os_string());
        self
    }

    /// skip restoration of object ownership
    pub fn no_owner(mut self) -> Self {
        self.no_owner = true;
        self
    }

    /// restore named function
    pub fn function<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.function = Some(name.as_ref().to_os_string());
        self
    }

    /// restore only the schema, no data
    pub fn schema_only(mut self) -> Self {
        self.schema_only = true;
        self
    }

    /// superuser user name to use for disabling triggers
    pub fn superuser<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.superuser = Some(name.as_ref().to_os_string());
        self
    }

    /// restore named relation (table, view, etc.)
    pub fn table<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.table = Some(name.as_ref().to_os_string());
        self
    }

    /// restore named trigger
    pub fn trigger<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.trigger = Some(name.as_ref().to_os_string());
        self
    }

    /// skip restoration of access privileges (grant/revoke)
    pub fn no_privileges(mut self) -> Self {
        self.no_privileges = true;
        self
    }

    /// restore as a single transaction
    pub fn single_transaction(mut self) -> Self {
        self.single_transaction = true;
        self
    }

    /// disable triggers during data-only restore
    pub fn disable_triggers(mut self) -> Self {
        self.disable_triggers = true;
        self
    }

    /// enable row security
    pub fn enable_row_security(mut self) -> Self {
        self.enable_row_security = true;
        self
    }

    /// use IF EXISTS when dropping objects
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
        self
    }

    /// do not restore comments
    pub fn no_comments(mut self) -> Self {
        self.no_comments = true;
        self
    }

    /// do not restore data of tables that could not be created
    pub fn no_data_for_failed_tables(mut self) -> Self {
        self.no_data_for_failed_tables = true;
        self
    }

    /// do not restore publications
    pub fn no_publications(mut self) -> Self {
        self.no_publications = true;
        self
    }

    /// do not restore security labels
    pub fn no_security_labels(mut self) -> Self {
        self.no_security_labels = true;
        self
    }

    /// do not restore subscriptions
    pub fn no_subscriptions(mut self) -> Self {
        self.no_subscriptions = true;
        self
    }

    /// do not restore table access methods
    pub fn no_table_access_method(mut self) -> Self {
        self.no_table_access_method = true;
        self
    }

    /// do not restore tablespace assignments
    pub fn no_tablespaces(mut self) -> Self {
        self.no_tablespaces = true;
        self
    }

    /// restore named section (pre-data, data, or post-data)
    pub fn section<S: AsRef<OsStr>>(mut self, section: S) -> Self {
        self.section = Some(section.as_ref().to_os_string());
        self
    }

    /// require table and/or schema include patterns to match at least one entity each
    pub fn strict_names(mut self) -> Self {
        self.strict_names = true;
        self
    }

    /// use SET SESSION AUTHORIZATION commands instead of ALTER OWNER commands to set ownership
    pub fn use_set_session_authorization(mut self) -> Self {
        self.use_set_session_authorization = true;
        self
    }

    /// database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, hostname: S) -> Self {
        self.host = Some(hostname.as_ref().to_os_string());
        self
    }

    /// database server port number
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// connect as specified database user
    pub fn username<S: AsRef<OsStr>>(mut self, name: S) -> Self {
        self.username = Some(name.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt (should happen automatically)
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }

    /// do SET ROLE before restore
    pub fn role<S: AsRef<OsStr>>(mut self, rolename: S) -> Self {
        self.role = Some(rolename.as_ref().to_os_string());
        self
    }
}

impl CommandBuilder for PgRestoreBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_restore".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(name) = &self.dbname {
            args.push("--dbname".into());
            args.push(name.into());
        }

        if let Some(filename) = &self.file {
            args.push("--file".into());
            args.push(filename.into());
        }

        if let Some(format) = &self.format {
            args.push("--format".into());
            args.push(format.into());
        }

        if self.list {
            args.push("--list".into());
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

        if self.data_only {
            args.push("--data-only".into());
        }

        if self.clean {
            args.push("--clean".into());
        }

        if self.create {
            args.push("--create".into());
        }

        if self.exit_on_error {
            args.push("--exit-on-error".into());
        }

        if let Some(name) = &self.index {
            args.push("--index".into());
            args.push(name.into());
        }

        if let Some(num) = &self.jobs {
            args.push("--jobs".into());
            args.push(num.into());
        }

        if let Some(filename) = &self.use_list {
            args.push("--use-list".into());
            args.push(filename.into());
        }

        if let Some(name) = &self.schema {
            args.push("--schema".into());
            args.push(name.into());
        }

        if let Some(name) = &self.exclude_schema {
            args.push("--exclude-schema".into());
            args.push(name.into());
        }

        if self.no_owner {
            args.push("--no-owner".into());
        }

        if let Some(name) = &self.function {
            args.push("--function".into());
            args.push(name.into());
        }

        if self.schema_only {
            args.push("--schema-only".into());
        }

        if let Some(name) = &self.superuser {
            args.push("--superuser".into());
            args.push(name.into());
        }

        if let Some(name) = &self.table {
            args.push("--table".into());
            args.push(name.into());
        }

        if let Some(name) = &self.trigger {
            args.push("--trigger".into());
            args.push(name.into());
        }

        if self.no_privileges {
            args.push("--no-privileges".into());
        }

        if self.single_transaction {
            args.push("--single-transaction".into());
        }

        if self.disable_triggers {
            args.push("--disable-triggers".into());
        }

        if self.enable_row_security {
            args.push("--enable-row-security".into());
        }

        if self.if_exists {
            args.push("--if-exists".into());
        }

        if self.no_comments {
            args.push("--no-comments".into());
        }

        if self.no_data_for_failed_tables {
            args.push("--no-data-for-failed-tables".into());
        }

        if self.no_publications {
            args.push("--no-publications".into());
        }

        if self.no_security_labels {
            args.push("--no-security-labels".into());
        }

        if self.no_subscriptions {
            args.push("--no-subscriptions".into());
        }

        if self.no_table_access_method {
            args.push("--no-table-access-method".into());
        }

        if self.no_tablespaces {
            args.push("--no-tablespaces".into());
        }

        if let Some(section) = &self.section {
            args.push("--section".into());
            args.push(section.into());
        }

        if self.strict_names {
            args.push("--strict-names".into());
        }

        if self.use_set_session_authorization {
            args.push("--use-set-session-authorization".into());
        }

        if let Some(hostname) = &self.host {
            args.push("--host".into());
            args.push(hostname.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(name) = &self.username {
            args.push("--username".into());
            args.push(name.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        if let Some(role) = &self.role {
            args.push("--role".into());
            args.push(role.into());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;

    #[test]
    fn test_builder_new() {
        let command = PgRestoreBuilder::new().build();

        assert_eq!(r#""pg_restore""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgRestoreBuilder::new()
            .program_dir("/usr/bin")
            .dbname("test")
            .file("test")
            .format("test")
            .list()
            .verbose()
            .version()
            .help()
            .data_only()
            .clean()
            .create()
            .exit_on_error()
            .index("test")
            .jobs("test")
            .use_list("test")
            .schema("test")
            .exclude_schema("test")
            .no_owner()
            .function("test")
            .schema_only()
            .superuser("test")
            .table("test")
            .trigger("test")
            .no_privileges()
            .single_transaction()
            .disable_triggers()
            .enable_row_security()
            .if_exists()
            .no_comments()
            .no_data_for_failed_tables()
            .no_publications()
            .no_security_labels()
            .no_subscriptions()
            .no_table_access_method()
            .no_tablespaces()
            .section("test")
            .strict_names()
            .use_set_session_authorization()
            .host("localhost")
            .port(5432)
            .username("test")
            .no_password()
            .password()
            .role("test")
            .build();

        assert_eq!(
            r#""/usr/bin/pg_restore" "--dbname" "test" "--file" "test" "--format" "test" "--list" "--verbose" "--version" "--help" "--data-only" "--clean" "--create" "--exit-on-error" "--index" "test" "--jobs" "test" "--use-list" "test" "--schema" "test" "--exclude-schema" "test" "--no-owner" "--function" "test" "--schema-only" "--superuser" "test" "--table" "test" "--trigger" "test" "--no-privileges" "--single-transaction" "--disable-triggers" "--enable-row-security" "--if-exists" "--no-comments" "--no-data-for-failed-tables" "--no-publications" "--no-security-labels" "--no-subscriptions" "--no-table-access-method" "--no-tablespaces" "--section" "test" "--strict-names" "--use-set-session-authorization" "--host" "localhost" "--port" "5432" "--username" "test" "--no-password" "--password" "--role" "test""#,
            command.to_command_string()
        );
    }
}
