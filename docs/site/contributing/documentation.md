# Maintaining documentation

The manual is Markdown under `docs/site`; `mkdocs.yml` owns navigation. Its
format follows JJ's Material for MkDocs approach: tutorials, concepts, guides,
technical explanations and command reference. The canonical `spec/src` remains
separate and is not copied into the published Help tree.

## Authoring

Write about this application's current behavior. Name repository contexts and
qualification limits where they affect the reader. Explain a complete workflow
with expected results and recovery paths. Do not document a planned capability
as available or expand this into a general JJ/Git manual.

Keep diagrams in fenced `mermaid` blocks and explain their relationships in
text. The build renders them to local SVGs using pinned Mermaid and a
network-disabled browser. Relative Markdown links work both in source and in the
generated site. Avoid remote runtime assets and executable inline scripts.

## References and review acknowledgements

Run `pnpm docs:generate` after changing application command definitions, shared
CLI long help or the HTTP schema. The CLI generator walks the registered
parsers, including inherited flags, visible aliases and nested commands.
`docs/maintenance/arguments.json` supplies reviewed descriptions where parser
help is absent; new undocumented arguments fail generation. It does not load a
repository or execute a command. The configuration-default reference uses
production Rust defaults. Complete marked TOML examples are parsed by the
production configuration validator in the documentation regression lane.

`docs/maintenance/coverage.json` maps behavior sources to authored pages. A
source fingerprint change fails the review gate even if the generated CLI is
current. Update the mapped pages, then record a specific explanation:

```sh
pnpm docs:review publication "Updated the retry guidance for the revised receipt lookup behavior."
```

An acknowledgement proves someone reviewed a changed source surface; it cannot
prove the prose is correct. Do not refresh every fingerprint without reviewing
its mapped pages. Source directories include future files so additions
invalidate the owning topic. Changes to the coverage map itself require review.

## Build, preview and delivery

`pnpm docs:check` verifies generated references, source acknowledgements,
diagrams, navigation and the generated local link/anchor closure.
`pnpm docs:serve` builds and serves the site at `http://127.0.0.1:7480/`.
`pnpm docs:test` runs negative fixtures for the tooling.
`pnpm docs:test:browser` checks the built Help site and overlay under production
HTTP policy.

`pnpm web:build` stages the site at `web/dist/help`. Desktop staging includes
the same files. Assets use a separate bounded manifest, with path, symlink, size
and media checks. Scripts and search workers stay local; Mermaid does not run in
the reader's browser. Open Help from the sidebar to verify the embedded
experience.

Documentation styling lives in `assets/manual.css` and `assets/layout.css`. The
build bundles the application's existing palette sources and the released DM
Sans/Source Code Pro fonts, including their licenses; keep these sources shared
instead of maintaining a separate theme catalog. Embedded `theme.js` reads the
same-origin host's presentation attributes and follows changes without writing
preferences. Standalone mode uses Original/Original Dark through Material's
light/dark switch. Mermaid produces local light/dark image pairs; both variants
must pass the generated asset and link checks.

Record actual commands, failures and remaining qualification in the progress
ledger with the implementation change. Do not publish the site or packages as a
side effect of documentation maintenance.
