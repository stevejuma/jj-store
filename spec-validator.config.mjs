import {
  defineConfig,
  singleIdVerification,
  tableRequirements,
} from "@lapismd/spec-validator";

export default defineConfig(tableRequirements(), {
  name: "jj-store",
  idPattern: /^JST-[A-Z]+-\d{3}$/,
  tableSection: "Requirements",
  ruleIds: {
    summary: "JST-GOV-001",
    governance: "JST-GOV-001",
    verification: "JST-GOV-001",
    book: "JST-GOV-001",
    bookIgnore: "JST-GOV-001",
    specFirst: "JST-GOV-002",
    internal: "JST-GOV-001",
  },
  validators: {
    summary: true,
    governance: {
      extras: ["AGENTS.md", "README.md"],
      normative: true,
      proseLimits: false,
      acceptance: false,
      references: true,
      changeMap: true,
    },
    verification: singleIdVerification({
      headers: {
        ids: ["ID"],
        status: ["Status"],
        evidence: ["Evidence"],
        required: [],
      },
      statuses: ["Implemented", "Planned"],
    }),
    book: true,
    specFirst: {
      mode: "mapped",
      canonicalPattern: "^spec/src/(?!SUMMARY|verification).+\\.md$",
      ignore: [
        "(^|/)node_modules/",
        "(^|/)(?:target|dist|build)/",
        "^spec/book/",
        "^spec/src/(?:SUMMARY|verification)\\.md$",
      ],
      rules: [
        {
          pattern: "^rustfmt\\.toml$",
          chapters: ["spec/src/spec-governance.md"],
        },
        {
          pattern: "^crates/jj-store-model/",
          chapters: ["spec/src/identity.md"],
        },
        {
          pattern: "^crates/jj-store-object/",
          chapters: ["spec/src/object-store.md"],
        },
        {
          pattern: "^crates/jj-store-wal/",
          chapters: ["spec/src/publication.md"],
        },
        {
          pattern: "^crates/jj-store-jj/",
          chapters: ["spec/src/jj-integration.md"],
        },
        {
          pattern: "^crates/jj-store-service/",
          chapters: ["spec/src/mutations.md"],
        },
        {
          pattern: "^crates/jj-store-rpc/",
          chapters: ["spec/src/jj-integration.md"],
        },
        {
          pattern: "^(?:crates/jj-store-http/|web/)",
          chapters: ["spec/src/api.md"],
        },
        {
          pattern: "^crates/jj-store-(?:adapters|tantivy)/",
          chapters: ["spec/src/adapters.md"],
        },
        {
          pattern: "^(?:crates/jj-store-cli/|bins/|docs/cli/)",
          chapters: ["spec/src/cli.md"],
        },
        {
          pattern:
            "^(?:crates/jj-store-testkit/|fixtures/|schemas/|proto/|docs/upstream-lock|docs/codec|docs/compatibility|docs/licenses/|docs/evidence/|scripts/.*(?:rustfs|spike|upstream)|deploy/)",
          chapters: ["spec/src/compatibility.md"],
        },
        {
          pattern:
            "^(?:scripts/|AGENTS\\.md$|spec-validator\\.config\\.mjs$|spec/book\\.toml$|\\.gitignore$|\\.prettier)",
          chapters: ["spec/src/spec-governance.md"],
        },
        {
          pattern:
            "^(?:Cargo\\.(?:toml|lock)$|rust-toolchain\\.toml$|package\\.json$|pnpm-lock\\.yaml$|README\\.md$|LICENSE\\.md$|docs/implementation-plan\\.md$|docs/reference/)",
          chapters: ["spec/src/architecture.md"],
        },
      ],
      protected: [
        "^(?:rustfmt\\.toml$|crates/|bins/|web/|schemas/|fixtures/|proto/|scripts/|deploy/|docs/|Cargo\\.(?:toml|lock)$|rust-toolchain\\.toml$|package\\.json$|pnpm-lock\\.yaml$|README\\.md$|LICENSE\\.md$|AGENTS\\.md$|spec-validator\\.config\\.mjs$|spec/book\\.toml$|\\.gitignore$|\\.prettier)",
      ],
    },
  },
  check: {
    lanes: [
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
});
