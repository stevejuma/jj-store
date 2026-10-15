<script lang="ts">
  import { untrack } from "svelte";
  import CommunityMarkdownContent from "../../web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import type { LinkStyle } from "../../web/src/lib/appearance/preferences.js";
  let {
    links = "inline",
    mode = "light",
    narrow = false,
  }: {
    links?: LinkStyle;
    mode?: "light" | "dark";
    narrow?: boolean;
  } = $props();
  const appearance = createAppearanceStore();
  appearance.update(untrack(() => ({ links, mode })));
  ownAppearance(appearance);
  const content = `## Release review

Review [**#42**](https://github.com/team/repo/pull/42) and [the design](https://figma.com/file/demo) before merging.

See the [GitLab issue](https://gitlab.com/team/repo/-/issues/7) and [Forgejo PR](https://code.forgejo.org/team/repo/pulls/9).

1. [#41](https://github.com/team/repo/pull/41)
2. [Slack discussion](https://team.slack.com/archives/C42)
3. [Unknown site](https://example.com/details)

https://github.com/team/repo/pull/42

[https://github.com/team/repo/pull/43](https://github.com/team/repo/pull/43)

> https://github.com/team/repo/issues/7

https://linear.app/team/issue/APP-12/fix

https://docs.google.com/document/d/example/edit

https://example.com/this/is/a/very/long/address/that/should/wrap/within/the/available/column

Code stays untouched: \`https://github.com/team/repo/pull/44\`.
`;
</script>

<section
  aria-label="Rendered link examples"
  style:max-width={narrow ? "18rem" : "42rem"}
>
  <CommunityMarkdownContent {content} fullWidth />
</section>

<style>
  section {
    margin: 1rem;
    font-family: var(--font-sans);
  }
</style>
