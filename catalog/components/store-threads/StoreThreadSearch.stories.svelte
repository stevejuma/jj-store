<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Thread inbox search",
    component: Fixture,
    tags: ["visual-pending"],
    args: { searchable: true },
    parameters: { layout: "fullscreen" },
  });
  async function search({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Open Thread general" });
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    const input = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await userEvent.type(input, "release checklist");
    await expect(
      c.getByRole("button", { name: "Open Thread planning" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Open Thread general" }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Open Thread planning" }),
    );
    await expect(
      await c.findByText("Release planning discussion"),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Close search" }));
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    const nextInput = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await userEvent.clear(nextInput);
    await userEvent.type(nextInput, "no match");
    await expect(c.getByText("No matching conversations")).toBeVisible();
  }
</script>

<Story
  name="Search loaded Thread titles and previews without requests"
  play={search}
/>
<Story
  name="Mobile search keeps the repository groups accessible"
  args={{ displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Toggle thread conversations" }),
    );
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    await userEvent.type(
      c.getByRole("searchbox", { name: "Search loaded conversations" }),
      "release",
    );
    await expect(
      c.getByRole("button", { name: "Open Thread planning" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Filtered pagination remains in the inbox list"
  args={{ pagedSearch: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Open Thread general" });
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    const input = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await userEvent.type(input, "release");
    await expect(c.getByText("No matching conversations")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: /More Threads/ }));
    await expect(
      await c.findByRole("button", { name: "Open Thread planning" }),
    ).toBeVisible();
    await expect(input).toHaveValue("release");
  }}
/>
