import { defineConfig } from "@lapismd/spec-validator";
import { changeConfirmations } from "./scripts/spec/change-confirmations.mjs";

export default defineConfig({
  requirementStyle: "table",
  minAcceptance: 0,
  maxAcceptance: null,
  validators: {
    governance: {
      extras: ["AGENTS.md", "README.md"],
      normative: true,
      proseLimits: false,
      acceptance: false,
      references: true,
      changeMap: true,
    },
    summary: {
      fragments: true,
    },
    fileLimits: {
      maxLines: 400,
      maxBytes: 32768,
    },
    qmd: {
      collection: "jj-store-current",
      defaultScope: "current",
      scopes: {
        current: {
          collection: "jj-store-current",
          path: "spec/src/current",
        },
        history: {
          collection: "jj-store-history",
          path: "spec/src/history",
        },
        all: {
          collection: "jj-store-all",
          path: "spec/src",
        },
      },
    },
    verification: {
      mode: "table",
      headers: {
        ids: ["ID"],
        status: ["Status"],
        evidence: ["Evidence"],
        required: [],
      },
      idMode: "single",
      statuses: ["Implemented", "Planned"],
      statusMatch: "exact",
      rowMultiplicity: "exactly-one",
      rejectOrphans: true,
      requireEvidence: true,
      files: [
        "^current/verification/(?!(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
      ],
    },
    book: true,
    specFirst: {
      mode: "mapped",
      confirmationProvider: changeConfirmations,
      canonicalPattern:
        "^spec/src/current/(?:contracts|governance)/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
      ignore: [
        "(^|/)node_modules/",
        "(^|/)(?:target|dist|build)/",
        "^spec/book/",
        "^spec/src/(?:SUMMARY|verification)\\.md$",
      ],
      rules: [
        {
          pattern:
            "^(?:\\.github/|\\.changeset/|\\.config/nextest|Dockerfile$|\\.dockerignore$|docs/releases\\.md$)",
          chapterPatterns: [
            "^spec/src/current/contracts/architecture/23-public-distribution\\.md$",
          ],
        },
        {
          pattern:
            "^spec/(?:archive/|relocations\\.json$|history-relations\\.yml$)",
          chapterPatterns: [
            "^spec/src/current/governance/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^\\.qmd/",
          chapterPatterns: [
            "^spec/src/current/governance/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern:
            "^(?:docs/(?:site/|maintenance/)|mkdocs\\.yml$|pyproject\\.toml$|uv\\.lock$)",
          chapterPatterns: [
            "^spec/src/current/contracts/documentation/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^docs/capture-preview\\.md$",
          chapterPatterns: [
            "^spec/src/current/contracts/cli/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/collaboration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^desktop/",
          chapterPatterns: [
            "^spec/src/current/contracts/desktop/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^docs/injectable-stores\\.md$",
          chapterPatterns: [
            "^spec/src/current/contracts/documentation/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/architecture/25-injectable-collaboration-stores\\.md$",
          ],
        },
        {
          pattern: "^docs/desktop-development\\.md$",
          chapterPatterns: [
            "^spec/src/current/contracts/documentation/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/desktop/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^docs/agent-execution\\.md$",
          chapterPatterns: [
            "^spec/src/current/contracts/documentation/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/collaboration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^docs/local-compute\\.md$",
          chapterPatterns: [
            "^spec/src/current/contracts/documentation/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/collaboration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/compatibility/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-jj/(?:src|tests)/local_",
          chapterPatterns: [
            "^spec/src/current/contracts/local-jj/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern:
            "^crates/jj-store-(?:cli|jj|wal|service|rpc)/(?:src|tests)/.*git[_-](?:bridge|interop)",
          chapterPatterns: [
            "^spec/src/current/contracts/git-interop/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^(?:catalog/|\\.storybook/|docs/catalog\\.md$)",
          chapterPatterns: [
            "^spec/src/current/contracts/ui/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^docs/ui-provenance\\.json$",
          chapterPatterns: [
            "^spec/src/current/governance/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
            "^spec/src/current/contracts/ui/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^rustfmt\\.toml$",
          chapterPatterns: [
            "^spec/src/current/governance/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-read-index/",
          chapterPatterns: [
            "^spec/src/current/contracts/adapters/(?!(?:index|SUMMARY).*).+\\.md$",
            "^spec/src/current/contracts/architecture/(?!(?:index|SUMMARY).*).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-model/",
          chapterPatterns: [
            "^spec/src/current/contracts/identity/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-object/",
          chapterPatterns: [
            "^spec/src/current/contracts/object-store/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-(?:core|git|native)/",
          chapterPatterns: [
            "^spec/src/current/contracts/architecture/25-injectable-collaboration-stores\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-wal/",
          chapterPatterns: [
            "^spec/src/current/contracts/publication/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-replication/",
          chapterPatterns: [
            "^spec/src/current/contracts/replication/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-vault/",
          chapterPatterns: [
            "^spec/src/current/contracts/local-jj/32-automatic-vault-backup\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-forge/",
          chapterPatterns: [
            "^spec/src/current/contracts/architecture/24-forge-stacks\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-workflow/",
          chapterPatterns: [
            "^spec/src/current/contracts/architecture/21-workflow-host\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-work/",
          chapterPatterns: [
            "^spec/src/current/contracts/projects/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-(?:agent|review|collab)/",
          chapterPatterns: [
            "^spec/src/current/contracts/collaboration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern:
            "^(?:crates/(?:jj-store-conversations|jj-history-service|tauri-plugin-jj-store)/|packages/jj-(?:conversations|history)/)",
          chapterPatterns: [
            "^spec/src/current/contracts/architecture/26-reusable-agent-workflows\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-capture/",
          chapterPatterns: [
            "^spec/src/current/contracts/collaboration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-jj/",
          chapterPatterns: [
            "^spec/src/current/contracts/jj-integration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-service/",
          chapterPatterns: [
            "^spec/src/current/contracts/mutations/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-rpc/",
          chapterPatterns: [
            "^spec/src/current/contracts/jj-integration/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-server/",
          chapterPatterns: [
            "^spec/src/current/contracts/cli/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^(?:crates/jj-store-http/|web/|docs/browser\\.md$)",
          chapterPatterns: [
            "^spec/src/current/contracts/api/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^crates/jj-store-(?:adapters|tantivy)/",
          chapterPatterns: [
            "^spec/src/current/contracts/adapters/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern: "^(?:crates/jj-store-cli/|bins/|docs/cli/)",
          chapterPatterns: [
            "^spec/src/current/contracts/cli/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern:
            "^(?:crates/jj-store-testkit/|runtime/|fixtures/|schemas/|proto/|docs/upstream-lock|docs/embedded-rustfs-qualification|docs/codec|docs/compatibility|docs/licenses/|docs/evidence/|scripts/.*(?:rustfs|spike|upstream)|deploy/)",
          chapterPatterns: [
            "^spec/src/current/contracts/compatibility/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern:
            "^(?:scripts/|AGENTS\\.md$|spec-validator\\.config\\.mjs$|spec/book\\.toml$|\\.gitignore$|\\.prettier)",
          chapterPatterns: [
            "^spec/src/current/governance/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
        {
          pattern:
            "^(?:Cargo\\.(?:toml|lock)$|rust-toolchain\\.toml$|package\\.json$|pnpm-(?:lock|workspace)\\.yaml$|README\\.md$|LICENSE\\.md$|docs/implementation-plan\\.md$|docs/reference/)",
          chapterPatterns: [
            "^spec/src/current/contracts/architecture/(?!(?:.*/)?(?:index|SUMMARY(?:\\.nav)?)\\.md$).+\\.md$",
          ],
        },
      ],
      protected: [
        "^(?:mkdocs\\.yml$|pyproject\\.toml$|uv\\.lock$|rustfmt\\.toml$|desktop/|catalog/|\\.storybook/|crates/|packages/|bins/|web/|schemas/|fixtures/|proto/|scripts/|deploy/|docs/|Cargo\\.(?:toml|lock)$|rust-toolchain\\.toml$|package\\.json$|pnpm-(?:lock|workspace)\\.yaml$|README\\.md$|LICENSE\\.md$|AGENTS\\.md$|spec-validator\\.config\\.mjs$|spec/book\\.toml$|\\.gitignore$|\\.prettier)",
      ],
    },
  },
  ruleIds: {
    summary: "JST-GOV-001",
    fileLimits: "JST-GOV-010",
    qmd: "JST-GOV-001",
    governance: "JST-GOV-001",
    verification: "JST-GOV-001",
    book: "JST-GOV-001",
    bookIgnore: "JST-GOV-001",
    specFirst: "JST-GOV-002",
    internal: "JST-GOV-001",
  },
  diagnostics: {},
  check: {
    lanes: [
      {
        name: "documentation",
        command: "pnpm",
        args: ["docs:check"],
      },
      {
        name: "catalog-cold-browser",
        command: "pnpm",
        args: ["catalog:test:browser"],
      },
      {
        name: "catalog-check",
        command: "pnpm",
        args: ["catalog:check"],
      },
      {
        name: "catalog-unit",
        command: "pnpm",
        args: ["catalog:test"],
      },
      {
        name: "catalog-interaction",
        command: "pnpm",
        args: ["test:storybook"],
      },
      {
        name: "catalog-build",
        command: "pnpm",
        args: ["build-storybook"],
      },
      {
        name: "browser-check",
        command: "pnpm",
        args: ["web:check"],
      },
      {
        name: "browser-unit",
        command: "pnpm",
        args: ["web:test"],
      },
      {
        name: "browser-interaction",
        command: "pnpm",
        args: ["web:test:browser"],
      },
      {
        name: "contracts",
        command: "pnpm",
        args: ["contracts:check"],
      },
      {
        name: "rustfmt",
        command: "cargo",
        args: ["fmt", "--all", "--", "--check"],
      },
      {
        name: "boundaries",
        command: "node",
        args: ["scripts/check-boundaries.mjs"],
      },
      {
        name: "progress",
        command: "node",
        args: ["scripts/check-progress.mjs"],
      },
      {
        name: "governance-tests",
        command: "node",
        args: ["--test", "scripts/governance.test.mjs"],
      },
      {
        name: "storage-harness",
        command: "python3",
        args: ["scripts/rustfs-contract.py", "--self-test"],
      },
      {
        name: "rust-tests",
        command: "cargo",
        args: ["test", "--workspace", "--locked"],
      },
    ],
    build: true,
    first: true,
  },
  name: "jj-store",
  plugins: ["scripts/check-ui-ownership.mjs"],
  idPattern: "^JST-[A-Z]+-\\d{3}$",
  tableSection: "Requirements",
  documentRoles: [
    {
      pattern: "(?:^|/)(?:index|SUMMARY(?:\\.nav)?)\\.md$",
      role: "navigation",
    },
    {
      pattern: "^current/(?:contracts|governance)/",
      role: "contract",
    },
    {
      pattern: "^current/verification/",
      role: "verification",
    },
    {
      pattern: "^current/progress/",
      role: "progress",
    },
    {
      pattern: "^history/",
      role: "history",
    },
    {
      pattern: "^[^/]+\\.md$",
      role: "navigation",
    },
  ],
});
