# Bundled local agent

The vendored buzz-agent source and tests are pinned to Buzz revision
ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be (Apache-2.0). Original source:
crates/buzz-agent in https://github.com/block/buzz. Cargo workspace fields are
made standalone. src/lib.rs additionally advertises a single ask mode and
accepts selecting it for an existing session; the existing permission gate
remains mandatory for every tool call. Modified files carry an attribution
notice. The owned entrypoint supplies a stable product executable name and
avoids raw diagnostics. Provider defaults, standing instructions, secret custody
and coding tools belong to jj-store. Upstream LICENSE is retained with the
crate. Updating this pin requires protocol/tool/cancellation qualification.

The maintained fork additionally advertises and implements ACP mode config
options for permission-gated Ask mode, and performs bounded authenticated model
discovery against the configured OpenAI-compatible provider. Paired catalogs
therefore come from the selected inference host; warm initialization still
performs no discovery or prompting.

The pinned model capability manifest and normative test corpus from upstream
`scripts/` are retained under the owned `runtime/local-agent/scripts/`
directory. The subprocess fixture serves model discovery separately from
scripted completion responses, preserving the permission and cancellation
assertions. Catalog discovery is enabled by the owned local-agent entrypoint;
the vendored upstream entrypoint retains its configured-model behavior.
