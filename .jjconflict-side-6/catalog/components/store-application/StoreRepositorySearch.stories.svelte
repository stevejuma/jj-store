<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Connected application/Search",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: { story: { inline: false, iframeHeight: 700 } },
    },
  });
</script>

<Story
  name="Command search navigation"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("button", { name: "Search repository" }),
    );
    const dialog = page.getByRole("dialog", { name: "Search workspace" });
    const input = within(dialog).getByRole("combobox", {
      name: "Search workspace",
    });
    await userEvent.type(input, "immutable");
    const result = await within(dialog).findByRole("option", {
      name: /Stream immutable native objects/u,
    });
    await userEvent.click(result);
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Search workspace" }),
      ).not.toBeInTheDocument(),
    );
    await expect(
      canvas.queryByRole("list", { name: "Repository breadcrumb" }),
    ).not.toBeInTheDocument();
    await waitFor(() =>
      expect(canvas.getByRole("tab", { name: "Commits" })).toHaveAttribute(
        "data-state",
        "active",
      ),
    );
    await expect(
      canvas.getByRole("button", { name: "Close Diff column" }),
    ).toBeVisible();

    await userEvent.keyboard("{Meta>}k{/Meta}");
    await expect(
      page.getByRole("dialog", { name: "Search workspace" }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
  }}
/>
