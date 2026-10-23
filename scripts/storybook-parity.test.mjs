import { test } from "node:test";
import assert from "node:assert/strict";
import { parityFindings } from "./check-storybook-parity.mjs";
function check(fixture, kind = "application", additional = {}) {
  const story = `<script module>import {defineMeta} from '@storybook/addon-svelte-csf'; import Fixture from '../../fixtures/Fixture.svelte';const {Story}=defineMeta({component:Fixture,parameters:{jjStoreStory:{kind:'${kind}',subject:'web/src/lib/Leaf.svelte',requirement:'JST-UI-001',reason:'An explicitly governed component or prototype scenario.'}}});</script><Story name="Example" />`;
  const sources = {
    "catalog/components/test/Test.stories.svelte": story,
    "catalog/fixtures/Fixture.svelte": fixture,
    ...additional,
  };
  return parityFindings({
    root: ".",
    files: Object.keys(sources),
    read: (file) => {
      if (!(file in sources)) throw Error(file);
      return sources[file];
    },
  });
}
test("unused root import cannot qualify an application story", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><p>Fake app</p>`,
    ).length,
  ));
test("thin application fixture renders the approved root", () =>
  assert.deepEqual(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><App controller={controller} />`,
    ),
    [],
  ));
test("conditional reduced shells fail closed", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte'; let {shell=false}=$props();</script>{#if shell}<App />{:else}<p>Reduced shell</p>{/if}`,
    ).length,
  ));
test("application content cannot be replaced through snippets", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><App><p>Fake content</p></App>`,
    ).length,
  ));
test("application geometry overrides are rejected", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><div style="width:900px"><App /></div>`,
    ).length,
  ));
test("explicit component examples do not claim application parity", () =>
  assert.deepEqual(
    check(
      `<script>import Leaf from '../../web/src/lib/Leaf.svelte';</script><Leaf />`,
      "component",
      { "web/src/lib/Leaf.svelte": "<p>Leaf</p>" },
    ),
    [],
  ));
test("application aliases and reexports are traced", () =>
  assert.deepEqual(
    check(
      `<script>import App from './alias.js';</script><App />`,
      "application",
      {
        "catalog/fixtures/alias.ts": `export {default} from './ApplicationStory.svelte';`,
      },
    ),
    [],
  ));
test("replacing the trusted adapter fails validation", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><App />`,
      "application",
      { "catalog/fixtures/ApplicationStory.svelte": "<p>Fake shared root</p>" },
    ).length,
  ));
test("a temporarily unmounted application does not qualify a replacement shell", () =>
  assert.deepEqual(
    check(
      `<script>import App from './ApplicationStory.svelte';let mounted=$state(true);</script>{#if mounted}<App />{/if}`,
    ),
    [],
  ));

test("rendering a root does not qualify a parallel fixture shell", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><App /><nav data-story-controls>Replacement navigation</nav>`,
    ).length,
  ));

test("trusted adapter cannot hide its production root behind a branch", () =>
  assert.ok(
    check(
      `<script>import App from './ApplicationStory.svelte';</script><App />`,
      "application",
      {
        "catalog/fixtures/ApplicationStory.svelte": `<script>import Root from '../../web/src/lib/ui/store-application/StoreApplicationRoot.svelte';</script>{#if false}<Root />{/if}`,
        "web/src/lib/ui/store-application/StoreApplicationRoot.svelte":
          "<main>Actual app</main>",
      },
    ).length,
  ));
