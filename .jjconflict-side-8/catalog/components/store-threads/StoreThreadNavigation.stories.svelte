<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository thread navigation",
    component: Fixture,
    tags: ["visual-pending"],
    args: { otherRepository: true },
    parameters: { layout: "fullscreen" },
  });
  async function nested({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const mobile = c.queryByRole("button", { name: "Open navigation sidebar" });
    if (mobile) await userEvent.click(mobile);
    const navigation = c.getByRole("navigation", {
      name: "Repository navigation",
    });
    await waitFor(() =>
      expect(
        within(navigation).getByRole("button", { name: "general" }),
      ).toBeVisible(),
    );
    await expect(
      navigation.querySelector('[data-navigation-section-kind="channels"]'),
    ).toBeNull();
    const browse = within(navigation)
      .getAllByRole("button", { name: "Browse threads" })
      .find((button) =>
        button.closest(".lc-community-navigation__section-header"),
      )!;
    await expect(browse.textContent?.trim()).toBe("");
    await expect(
      browse.closest(".lc-community-navigation__section-header"),
    ).not.toBeNull();
    const home = within(navigation)
      .getByRole("button", { name: "Collaboration home" })
      .closest(".lc-community-navigation__item-tree")!;
    const thread = within(home as HTMLElement).getByRole("button", {
      name: "general",
    });
    const repo = within(home as HTMLElement).getByRole("button", {
      name: "Collaboration home",
    });
    await expect(thread).toBeVisible();
    await expect(
      thread.querySelector("svg")!.getBoundingClientRect().left -
        repo.querySelector("svg")!.getBoundingClientRect().left,
    ).toBeGreaterThan(16);
    await expect(
      within(home as HTMLElement).getByRole("button", {
        name: "Browse threads",
      }),
    ).toBeVisible();
    await userEvent.click(
      within(navigation).getByRole("button", {
        name: "Expand Other repository",
      }),
    );
    const other = within(navigation)
      .getByRole("button", { name: "Other repository" })
      .closest(".lc-community-navigation__item-tree")!;
    await userEvent.click(
      await within(other as HTMLElement).findByRole("button", {
        name: "general",
      }),
    );
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    const breadcrumb = await c.findByRole("navigation", { name: "breadcrumb" });
    if (mobile)
      await expect(
        breadcrumb.closest('[aria-label="Main toolbar"]'),
      ).not.toBeNull();
    await expect(
      within(breadcrumb).getByRole("link", { name: "Other repository" }),
    ).toBeVisible();
    await expect(breadcrumb).toHaveTextContent(
      /Other repository.*Threads.*general/,
    );
    await userEvent.click(
      within(breadcrumb).getByRole("link", { name: "Threads" }),
    );
    await waitFor(() =>
      expect(breadcrumb).toHaveTextContent(/^Other repository\s*Threads$/),
    );
    await expect(c.getByRole("heading", { name: "Threads" })).toBeVisible();
  }
</script>

<Story
  name="Threads nest under repositories with contextual breadcrumbs"
  play={nested}
/>
<Story
  name="Mobile thread context remains in the main toolbar"
  args={{ displayMode: "mobile" }}
  play={nested}
/>
<Story
  name="Collapsed repositories retain thread and browse actions"
  args={{ initiallyCollapsed: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "Repositories" }));
    const option = await body.findByRole("option", {
      name: /general Other repository/,
    });
    await userEvent.click(option);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    await expect(
      c.getByRole("navigation", { name: "breadcrumb" }),
    ).toHaveTextContent(/Other repository.*Threads.*general/);
    await userEvent.click(c.getByRole("button", { name: "Repositories" }));
    await userEvent.click(
      await body.findByRole("option", { name: "Browse threads" }),
    );
    await expect(
      c.getByRole("navigation", { name: "breadcrumb" }),
    ).toHaveTextContent(/^Other repository\s*Threads$/);
  }}
/>
