import test from "node:test";
import assert from "node:assert/strict";
import { ownershipFindings, moduleReferences } from "./check-ui-ownership.mjs";

const local = "web/src/lib/ui/Message.svelte";
const provenance = {
  version: 1,
  copies: [
    {
      local,
      sources: [
        {
          path: "src/components/community-message/CommunityMessage.svelte",
          revision: "a".repeat(40),
          license: "Apache-2.0",
        },
      ],
    },
  ],
};
function fixture(
  source = "<script>import { Button } from '@lapismd/design-core/shadcn/button';</script>",
  extra = {},
  manifest = provenance,
) {
  const contents = {
    [local]: source,
    "docs/ui-provenance.json": JSON.stringify(manifest),
    ...extra,
  };
  return ownershipFindings({
    root: "/repo",
    files: Object.keys(contents),
    read: (file) => contents[file],
    realpath: (file) => `/repo/${file}`,
  });
}
test("accepts local presentation and permitted foundations with attribution", () => {
  assert.deepEqual(
    fixture(
      "<script>import Mira from '@lapismd/mira'; // copied from Community</script>",
      { "docs/reference.md": "@lapismd/lapis-community" },
    ),
    [],
  );
});
test("rejects runtime, type, reexport and lazy Community module dependencies", () => {
  for (const statement of [
    "import X from '@lapismd/lapis-community/components';",
    "import type { X } from '@lapismd/lapis-community';",
    "export { X } from '@lapismd/lapis-community';",
    "const x = import('@lapismd/lapis-community');",
    "const x = require('@lapismd/lapis-community');",
    "type X = import('@lapismd/lapis-community').X;",
  ])
    assert.equal(
      fixture(`<script lang='ts'>${statement}</script>`).length,
      1,
      statement,
    );
});
test("rejects CSS, package aliases, lock resolutions and checkout configuration", () => {
  for (const [file, source] of Object.entries({
    "web/src/a.css": '@import "@lapismd/lapis-community/styles.css";',
    "package.json":
      '{"dependencies":{"alias":"npm:@lapismd/lapis-community@1"}}',
    "pnpm-lock.yaml": "resolution: ../lapis-community/",
    "web/vite.config.ts": "const aliases = { ui: '../lapis-community/src' };",
    "scripts/local-package-pnpmfile.cjs":
      "const x = '@lapismd/lapis-community';",
  }))
    assert.ok(fixture(undefined, { [file]: source }).length, file);
});
test("does not mistake negative fixtures or comments for executable imports", () => {
  assert.deepEqual(
    fixture(
      "<script>// import X from '@lapismd/lapis-community';\nconst example = \"import X from '@lapismd/lapis-community'\";</script>",
    ),
    [],
  );
  assert.equal(
    moduleReferences("type X = import('./local').X", "a.ts")[0].value,
    "./local",
  );
});
test("rejects external production symlinks and invalid or missing provenance", () => {
  const rows = ownershipFindings({
    root: "/repo",
    files: [local, "docs/ui-provenance.json"],
    read: (file) => (file === local ? "" : JSON.stringify(provenance)),
    realpath: () => "/other/component.svelte",
  });
  assert.match(rows[0].message, /inside/);
  for (const manifest of [
    null,
    { version: 2, copies: [] },
    {
      version: 1,
      copies: [{ ...provenance.copies[0], local: "web/src/lib/../outside" }],
    },
    { version: 1, copies: [{ local, sources: [] }] },
  ])
    assert.ok(fixture(undefined, {}, manifest).length);
});

test("requires complete recognized and explicitly marked copy coverage", () => {
  for (const [file, source] of [
    ["web/src/lib/ui/community-message/NewMessage.svelte", ""],
    ["web/src/lib/ui/renamed/Message.svelte", "<!-- @community-copy -->"],
  ]) {
    assert.match(
      fixture(undefined, { [file]: source })[0].message,
      /provenance entry/,
    );
  }
  const native = "web/src/lib/ui/community-projects/native.ts";
  assert.deepEqual(
    fixture(
      undefined,
      { [native]: "export const id = 1;" },
      {
        ...provenance,
        owned: [
          {
            local: native,
            reason: "Native jj-store identity helper, locally authored.",
          },
        ],
      },
    ),
    [],
  );
  assert.ok(
    fixture(
      undefined,
      {},
      { ...provenance, owned: [{ local, reason: "bypass" }] },
    ).length,
  );
});
test("rejects Community CSS imports inside Svelte styles", () => {
  assert.equal(
    fixture('<style>@import "@lapismd/lapis-community/styles.css";</style>')
      .length,
    1,
  );
});
