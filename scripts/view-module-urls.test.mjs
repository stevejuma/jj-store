import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import test from "node:test";
import { viewModuleUrls } from "./view-module-urls.mjs";

const root = resolve("web");
const id = `${root}/src/lib/ui/store-application/application-views.ts`;
const source = `viewModuleURL(
  "../community-projects/CommunityProjectsRepositoryFiles.svelte",
)`;

test("development prepares multiline view URLs with trailing commas and HMR queries", () => {
  const plugin = viewModuleUrls(root);
  plugin.configResolved({ command: "serve" });
  for (const moduleId of [id, `${id}?t=12345`]) {
    assert.equal(
      plugin.transform(source, moduleId),
      '"/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte"',
    );
  }
  assert.equal(plugin.transform(source, `${root}/unrelated.ts`), undefined);
});

test("every application view receives a compiler URL before development evaluation", async () => {
  const plugin = viewModuleUrls(root);
  plugin.configResolved({ command: "serve" });
  const transformed = plugin.transform(await readFile(id, "utf8"), id);
  assert.equal(/viewModuleURL\s*\(/.test(transformed), false);
  assert.equal((transformed.match(/lazyView\(/g) ?? []).length, 24);
  assert.equal((transformed.match(/"\/src\/lib\/ui\//g) ?? []).length, 24);
});

test("catalog retry URLs use the catalog serving root and the same owned component", () => {
  const plugin = viewModuleUrls(root);
  plugin.configResolved({ command: "serve", root: resolve(".") });
  assert.equal(
    plugin.transform(source, `${id}?t=98765`),
    '"/web/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte"',
  );
});

test("built retry URLs use emitted entries and resolve relative to their importing chunk", () => {
  const plugin = viewModuleUrls(root);
  plugin.configResolved({ command: "build" });
  const chunks = [];
  const transformed = plugin.transform.call(
    {
      emitFile(chunk) {
        chunks.push(chunk);
        return "VIEW_1";
      },
    },
    source,
    id,
  );
  assert.deepEqual(chunks, [
    {
      type: "chunk",
      id: `${root}/src/lib/ui/community-projects/CommunityProjectsRepositoryFiles.svelte`,
    },
  ]);
  assert.equal(transformed, "import.meta.ROLLUP_FILE_URL_VIEW_1");
  assert.equal(
    plugin.resolveFileUrl({ relativePath: "files-hash.js" }),
    'new URL("files-hash.js", import.meta.url).href',
  );
});

test("view URL preparation rejects paths outside the application or non-component entries", () => {
  const plugin = viewModuleUrls(root);
  for (const path of ["../../../../../../outside.svelte", "../lazy-view.ts"]) {
    assert.throws(
      () => plugin.transform(`viewModuleURL(${JSON.stringify(path)})`, id),
      /Unexpected view module path/,
    );
  }
});
