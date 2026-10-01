<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import NativeActivityFixture from "../../fixtures/NativeActivityFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Native activity",
    component: NativeActivityFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Virtual rows lazy context and automatic continuation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const region = c.getByRole("region", { name: "Virtual activity fixture" });
    const feed = await c.findByRole("list", { name: "Repository activity" });
    const viewport = region.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await expect(viewport).not.toBeNull();
    await waitFor(() =>
      expect(Number(region.dataset.discoveries)).toBeGreaterThan(0),
    );
    await expect(Number(region.dataset.discoveries)).toBeLessThan(40);
    await expect(
      feed.querySelectorAll('[data-ui-part="virtual-row"]').length,
    ).toBeLessThan(40);
    await expect(region.dataset.sessions).toBe("0");
    await expect(
      c.queryByRole("log", { name: "Agent conversation activity" }),
    ).toBeNull();
    await userEvent.click(
      c.getAllByRole("button", { name: "Agent context" })[0]!,
    );
    await expect(
      await c.findByRole("log", { name: "Agent conversation activity" }),
    ).toHaveTextContent("Updated the parser");
    await expect(region.dataset.sessions).toBe("1");
    await waitFor(() => {
      const rows = feed.querySelectorAll<HTMLElement>(
        ":scope > .community-projects-virtual-rows > .community-projects-virtual-rows__content > [data-ui-part='virtual-row']",
      );
      expect(rows.length).toBeGreaterThan(1);
      expect(rows[1]!.getBoundingClientRect().top).toBeGreaterThanOrEqual(
        rows[0]!.getBoundingClientRect().bottom - 1,
      );
    });
    await userEvent.click(
      c.getAllByRole("button", { name: "Agent context" })[0]!,
    );
    await userEvent.click(
      c.getAllByRole("button", { name: "Agent context" })[0]!,
    );
    await expect(region.dataset.sessions).toBe("1");
    viewport.scrollTo(0, viewport.scrollHeight);
    await waitFor(() => expect(region.dataset.loaded).toBe("160"));
    await expect(region.dataset.continuations).toBe("1");
    await expect(
      feed.querySelectorAll('[data-ui-part="virtual-row"]').length,
    ).toBeLessThan(40);
    await expect(Number(region.dataset.discoveries)).toBeLessThan(100);
    viewport.scrollTo(0, 0);
    await expect(
      await c.findByRole("link", { name: "Open commit Native activity 0" }),
    ).toBeVisible();
    await userEvent.click(
      c.getAllByRole("button", { name: "Agent context" })[0]!,
    );
    await expect(
      await c.findByRole("log", { name: "Agent conversation activity" }),
    ).toBeVisible();
    await expect(region.dataset.sessions).toBe("1");
  }}
/>
