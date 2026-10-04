# Native repository tutorial

This walkthrough creates a private installation and a new native workspace. It
does not convert an existing Git repository. Use the explicit executables from
[installation](install.md), and choose empty paths outside the checkout. The
variables below are examples; set them to absolute paths on your device.

```sh
store_bin=/absolute/source/target/debug/jj-store
jj_bin=/absolute/source/target/debug/jj
rustfs_bin=/absolute/source/runtime/embedded-rustfs/target/debug/jj-store-rustfs
state_dir=/absolute/private/jj-store-demo
workspace_dir=/absolute/work/demo
```

## Initialize and configure

```sh
"$store_bin" init --local --state-dir "$state_dir" \
  --rustfs-executable "$rustfs_bin"
"$store_bin" config check --config "$state_dir/server.toml"
```

Initialization creates private installation identity, discovery catalog and
managed storage configuration. Existing state is refused. Inspect the generated
`server.toml`; its paths and listener are the source of truth. Set `web_root` to
the absolute built `web/dist` directory to serve the application and Help.
Configuration format 2 rejects unknown keys and older formats.

Start the server in a terminal you can keep open:

```sh
"$store_bin" run --config "$state_dir/server.toml"
```

Wait for the listening message. Startup verifies configured storage and the
installation sentinel before announcing readiness. Do not run a second serving
process against the same volume. LocalTrusted requires no user bearer token.

## Create the first workspace

In another terminal, use the socket path recorded in the generated
configuration:

```sh
"$jj_bin" store init "$workspace_dir" \
  --unix-socket "$state_dir/native.sock"
cd "$workspace_dir"
"$jj_bin" store status
"$jj_bin" store capabilities
```

The destination must be empty for the initial attempt. Keep the returned
repository identity. `store status` inspects authoritative heads without
snapshotting local files. `capabilities` distinguishes qualified command support
from unsupported or unqualified behavior.

The browser discovers registered repositories through the installation catalog.
If existing authority needs discovery registration, use:

```sh
"$store_bin" repository register --config "$state_dir/server.toml" \
  --repo-id REPOSITORY_ID --name demo
```

Registration obtains the storage lifetime lock, so stop the service before
running this offline operation and restart afterward. Do not use registration as
a substitute for creating authority.

## Browse and continue

Open the listener URL printed by the service. Select the repository, inspect
History, Files and Op Log, and open Help from the left sidebar. Attach a second
workspace with `jj store attach`, its unique workspace name and the same socket
and repository ID; it shares authority but has separate physical files.

Stop the foreground service with interrupt and restart using the same config.
Keep the persistent volume and installation files together. If creation or
attachment loses its response, repeat the same command with unchanged paths and
options: its private journal retains the original intent. Never delete the
journal to bypass an unknown outcome.
