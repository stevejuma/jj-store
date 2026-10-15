<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/LinkRenderingFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Appearance/Links",
    component: Fixture,
    tags: ["visual-pending"],
  });
  async function verify({
    canvasElement,
    args,
  }: {
    canvasElement: HTMLElement;
    args: { links?: string };
  }) {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const examples = await canvas.findByRole("region", {
      name: "Rendered link examples",
    });
    const pr = within(examples).getByRole("link", { name: "#42" });
    await expect(
      pr.querySelector('svg[data-link-site="GitHub"]'),
    ).not.toBeNull();
    await expect(pr.querySelector("strong")).toHaveTextContent("#42");
    await expect(pr).not.toHaveAttribute("data-layout");
    await expect(pr).toHaveAttribute("rel", "noopener noreferrer");
    await expect(pr).toHaveAttribute("target", "_blank");
    for (const [name, site] of [
      ["GitLab issue", "GitLab"],
      ["Forgejo PR", "Forgejo"],
    ]) {
      const link = within(examples).getByRole("link", { name });
      await expect(
        link.querySelector(`svg[data-link-site="${site}"]`),
      ).not.toBeNull();
      await expect(link).not.toHaveAttribute("data-layout");
    }
    await expect(getComputedStyle(pr).backgroundColor).toBe("rgba(0, 0, 0, 0)");
    await expect(getComputedStyle(pr.querySelector("svg")!).width).not.toBe(
      "0px",
    );
    for (const name of [
      "#41",
      "Slack discussion",
      "Unknown site",
      "https://github.com/team/repo/pull/43",
    ]) {
      const inline = within(examples).getByRole("link", { name });
      await expect(inline).not.toHaveAttribute("data-layout");
      await expect(inline.querySelector("svg")).not.toBeNull();
    }
    await expect(
      examples.querySelectorAll("blockquote .store-link-card"),
    ).toHaveLength(0);
    await expect(examples.querySelectorAll("code a")).toHaveLength(0);
    await expect(examples.querySelectorAll(".store-link-card")).toHaveLength(
      args.links === "compact" || args.links === "rich" ? 3 : 0,
    );
    for (const card of examples.querySelectorAll(".store-link-card"))
      await expect(card).toHaveAttribute("data-layout", args.links);
    await expect(examples.scrollWidth).toBeLessThanOrEqual(
      examples.clientWidth + 1,
    );
    pr.focus();
    await userEvent.tab();
    await userEvent.tab({ shift: true });
    await expect(pr).toHaveFocus();
    await waitFor(() =>
      expect(page.getByRole("tooltip")).toHaveTextContent(
        "https://github.com/team/repo/pull/42",
      ),
    );
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(page.queryByRole("tooltip")).toBeNull());
    const external = performance
      .getEntriesByType("resource")
      .filter(
        (entry) =>
          /^https?:/.test(entry.name) &&
          new URL(entry.name).origin !== location.origin,
      );
    await expect(external.map((entry) => entry.name)).toEqual([]);
  }
</script>

<Story name="Inline links with local site icons" play={verify} />
<Story
  name="Compact cards only for standalone content"
  args={{ links: "compact" }}
  play={verify}
/>
<Story
  name="Rich cards in a narrow dark column"
  globals={{ colorMode: "dark" }}
  args={{ links: "rich", mode: "dark", narrow: true }}
  play={verify}
/>
