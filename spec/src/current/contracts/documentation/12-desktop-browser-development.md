# Desktop browser development documentation

JST-DOC-001/002 and JST-DESK-004 require the desktop development guide to
describe the existing production renderer's browser debugging workflow. Document
`pnpm dev:desktop:ui`, its one-time fixture preparation, ordinary browser mode,
port and binary overrides, HMR, backend-only restart and non-destructive Stop.
Keep native-only capability limits explicit: desktop geometry in a browser
provides no terminal, native glass, dragging or sleep prevention. Browser
acceptance cannot qualify WKWebView or native distribution.

The application source review covers explicit host injection and shared
host/connection/appearance bundling without changing the published manual's
service-backed browsing, editing or Settings contracts. Documentation checks may
reuse an explicitly selected existing `JJ_STORE_DOC_REFERENCE` executable for a
UI-only slice; this does not qualify new CLI/parser behavior or justify a Rust
rebuild. Immutable committed evidence and unrelated history navigation remain
preserved when generating a new development record.
