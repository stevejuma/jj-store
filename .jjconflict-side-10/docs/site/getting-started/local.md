# Existing JJ folder tutorial

Use this path for an existing Git-backed JJ repository. Code remains in that
repository, and the application accesses it only after explicit device folder
registration. The folder must be a supported JJ workspace; a plain unrelated
directory is not enough.

## Start a filesystem-only installation

After building the paired executables, initialize a new private state directory:

```sh
target/debug/jj-store init --local --storage filesystem \
  --state-dir /absolute/private/jj-store-folders
```

Set the generated configuration's `web_root` to the absolute built `web/dist`.
Validate it and run the foreground server:

```sh
target/debug/jj-store config check \
  --config /absolute/private/jj-store-folders/server.toml
target/debug/jj-store run \
  --config /absolute/private/jj-store-folders/server.toml
```

Filesystem-only mode uses loopback LocalTrusted Admin admission and does not
launch RustFS. Native search/replication/backup configurations are not accepted
in that mode. Open the printed URL and use the sidebar's **Add local
repository** action to choose the actual JJ folder. The folder picker is
device-local.

## Understand what registration changes

Registration records an explicit device binding and verifies the repository
profile. It does not snapshot working files, rewrite commits, convert storage,
or initialize all collaboration metadata. Multiple JJ workspaces of one
repository share companion metadata, while each physical folder remains a
distinct device location.

Browse files and immutable history using the operation selected by the browser.
A filesystem Save changes the admitted working file only; it does not itself
publish a JJ snapshot or collaboration metadata. Use the foreground JJ workflow
when you need to snapshot those files.

## Use local collaboration

From the registered repository, the custom `jj project`, `jj task`, `jj review`
and supported `jj agent` commands select the local companion provider. They do
not need a native server connection. The first supported metadata write can
initialize the companion identity; list/status reads leave an absent identity
absent.

For example, create a repository discussion with an explicit request ID:

```sh
/absolute/source/target/debug/jj review repository \
  --request-id REQUEST_ID --body 'Notes for this repository'
```

Use a valid full request identifier in place of `REQUEST_ID`; see the generated
[review reference](../cli/jj-review.md). Preserve it if the response is unknown.
Native `--cursor` and local `--after`/`--at-metadata` selectors have different
contracts and cannot be mixed.

Local portable code/agent review anchors, indexed search, recoverable code
commands have separate support gates. Eligible repositories can be copied into
independent server storage through
[repository conversion](../guides/conversion.md). Repository discussions working
does not imply every native capability is available. Consult the
[capability table](../reference/capabilities.md) before extending the workflow.
