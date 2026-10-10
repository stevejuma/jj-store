# Agent icon sources

Bundled on 29 September 2026 from the products' public SVG favicons:

- `codex.svg`: https://developers.openai.com/favicon.svg
- `cursor.svg`: https://cursor.com/favicon.svg
- `opencode.svg`: https://opencode.ai/favicon.svg

These images identify the captured integration in the local repository browser.

- `jj-store.svg`: the app's maintained `desktop/icons/logo-dark.svg`, sourced
  from the user's accepted jj-store artwork. See `desktop/icons/README.md` for
  attribution and tracing details. Used for the bundled jj-agent ACP harness.

- `favicon.svg`: a self-contained 64px RGBA export of the same accepted logo,
  rendered from `desktop/icons/icon.png` with macOS `sips`. The PNG is embedded
  in an SVG container for the existing browser SVG asset route. It retains the
  transparent surround and pale silhouette edge without loading the full traced
  vector or an external resource. Registered by `web/index.html`; the browser
  build manifest includes it through the existing agent-icons asset inventory.
