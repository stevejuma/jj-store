<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/RepositoryEntryControlsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository entry controls",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
</script>

<Story
  name="One repository menu opens server and local forms"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const add = await c.findByRole("button", { name: "Add repository" });
    await expect(
      c.queryByRole("button", { name: "Add local repository" }),
    ).toBeNull();
    await expect(
      c.queryByRole("button", { name: "Create server repository" }),
    ).toBeNull();
    await add.focus();
    await userEvent.keyboard("{Enter}");
    const server = await body.findByRole("menuitem", {
      name: "Add Server Repository",
    });
    const local = body.getByRole("menuitem", { name: "Add Local Repository" });
    await expect(
      server.querySelector('svg[aria-hidden="true"]'),
    ).not.toBeNull();
    await expect(local.querySelector('svg[aria-hidden="true"]')).not.toBeNull();
    for (const item of [server, local]) {
      const label = document.createRange();
      label.selectNodeContents(
        [...item.childNodes].find((node) =>
          node.textContent?.includes("Repository"),
        )!,
      );
      await expect(label.getClientRects().length).toBe(1);
      await expect(item.scrollWidth).toBeLessThanOrEqual(item.clientWidth);
    }
    await userEvent.click(server);
    const dialog = await body.findByRole("dialog", {
      name: "Create server repository",
    });
    await waitFor(() => expect(dialog).toBeVisible());
    await waitFor(() =>
      expect(dialog.contains(document.activeElement)).toBe(true),
    );
    const name = within(dialog).getByRole("textbox", { name: "Name" });
    const card = name.closest<HTMLElement>(".store-settings-card")!;
    await expect(card).not.toBeNull();
    await expect(
      within(card).getByRole("heading", { name: "Repository details" }),
    ).toBeVisible();
    const label = name
      .closest('[data-slot="field"]')!
      .querySelector('[data-ui-part="field-label"]')!;
    if (matchMedia("(max-width: 40rem)").matches)
      await expect(label.getBoundingClientRect().bottom).toBeLessThan(
        name.getBoundingClientRect().top,
      );
    else
      await expect(label.getBoundingClientRect().right).toBeLessThan(
        name.getBoundingClientRect().left,
      );
    await expect(card.querySelector(".store-settings-icon svg")).not.toBeNull();
    await expect(getComputedStyle(name).borderTopWidth).toBe("1px");
    await expect(getComputedStyle(card).boxShadow).toContain("0px 0px 0px 1px");
    await userEvent.type(name, "Review repository");
    await expect(
      within(dialog).getByRole("button", { name: "Create repository" }),
    ).toBeEnabled();
    await userEvent.click(
      within(dialog).getByText("Close", { selector: "button" }),
    );
    await waitFor(() => expect(add).toHaveFocus());
    await userEvent.click(add);
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Add Local Repository" }),
    );
    const folder = await body.findByRole("dialog", {
      name: "Add local JJ folder",
    });
    await waitFor(() => expect(folder).toBeVisible());
    await expect(
      within(folder).getByRole("textbox", { name: "Folder" }),
    ).toBeVisible();
    await userEvent.click(
      within(folder).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() => expect(add).toHaveFocus());
    await userEvent.click(add);
    await userEvent.keyboard("{Escape}");
    await expect(add).toHaveFocus();
  }}
/>

<Story
  name="Header spacing and bordered repository action row"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const header = canvasElement.querySelector(
      ".community-app-shell__header-end",
    )!;
    await expect(
      parseFloat(getComputedStyle(header).columnGap),
    ).toBeGreaterThanOrEqual(4);
    await userEvent.click(
      await c.findByRole("button", { name: "Repository settings" }),
    );
    const button = await c.findByRole("button", {
      name: "Use local Git-backed JJ",
    });
    const row = button.closest(".store-settings__action-row")!;
    const card = row.closest(".store-settings-card")!;
    await expect(getComputedStyle(card).boxShadow).toContain("0px 0px 0px 1px");
    await expect(
      getComputedStyle(card.querySelector(".store-settings-card__header")!)
        .borderBottomWidth,
    ).toBe("1px");
    const description = row.querySelector("p")!;
    await expect(description.getBoundingClientRect().right).toBeLessThan(
      button.getBoundingClientRect().left,
    );
    await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth + 1);
  }}
/>
