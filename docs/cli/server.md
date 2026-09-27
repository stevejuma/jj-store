# Native JJ server

Run an authenticated native repository service against a configured RustFS
bucket. The process stays in the foreground. No command installs a service or
starts RustFS. Use `jj-store config example` to create a configuration outside
repository checkouts, then `config check` before starting it.

Use `jj-store help -k commands` for the command guide. Repository creation and
native client operations use the custom `jj store` command family.

An optional `web_root` configuration field serves a prebuilt browser bundle from
the same origin. It is resolved relative to the configuration file. See
[Repository browser](../browser.md) for build, authentication and acceptance
instructions. Startup rejects missing, oversized or unsafe listed assets.
