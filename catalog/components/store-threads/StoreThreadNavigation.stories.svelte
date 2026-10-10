<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository thread navigation",
    component: Fixture,
    tags: ["visual-pending"],
    args: { otherRepository: true, multiRepository: true },
    parameters: { layout: "fullscreen" },
  });
  async function aliases({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const inbox = await c.findByRole("navigation", {
      name: "Threads by repository",
    });
    const rows = within(inbox).getAllByRole("button", {
      name: "Open Thread general",
    });
    await expect(rows).toHaveLength(2);
    await userEvent.click(rows[1]!);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    await expect(
      within(inbox)
        .getAllByRole("button", { name: "Open Thread general" })
        .every((row) => row.getAttribute("aria-pressed") === "true"),
    ).toBe(true);
    await expect(
      c.queryAllByRole("button", { name: "Browse threads" }),
    ).toHaveLength(0);
    await userEvent.click(c.getByRole("tab", { name: "All" }));
    await expect(c.getByRole("button", { name: /^Threads$/ })).toBeVisible();
  }
</script>

<Story
  name="Multiple repository aliases open one canonical Thread"
  play={aliases}
/>
<Story
  name="Loaded inbox and repository navigation reuse the same captures"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("navigation", { name: "Threads by repository" });
    const counts = () =>
      JSON.parse(c.getByTestId("sidebar-thread-reads").textContent ?? "{}");
    await waitFor(() => expect(counts()["99".repeat(16)]).toBe(1));
    await userEvent.click(
      c.getByRole("button", { name: "Collapse Other repository Threads" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Expand Other repository Threads" }),
    );
    await expect(counts()["99".repeat(16)]).toBe(1);
  }}
/>
<Story
  name="Mobile Thread aliases retain one conversation"
  args={{ displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Toggle thread conversations" }),
    );
    await userEvent.click(
      (await c.findAllByRole("button", { name: "Open Thread general" }))[1]!,
    );
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
  }}
/>
<Story
  name="Compact navigation keeps Threads beneath Agents"
  args={{ initiallyCollapsed: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByRole("button", { name: /^Agents$/ })).toBeVisible();
    await expect(c.getByRole("button", { name: /^Threads$/ })).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Browse threads" }),
    ).toBeNull();
  }}
/>
