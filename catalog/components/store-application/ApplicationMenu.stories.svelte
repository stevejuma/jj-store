<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { page } from "vitest/browser";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/RuntimeReloadFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Application menu",
    component: Fixture,
    args: { application: true },
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Mobile repository settings retain the toolbar above their body"
  args={{ displayMode: "mobile", initialPage: "settings" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const menu = await canvas.findByRole("button", {
      name: "Application menu",
    });
    const toolbar = menu.closest('[data-ui-part="toolbar"]')!;
    const settings = canvasElement.querySelector(
      '[data-ui-part="body"][aria-label="Repository settings"]',
    )!;
    await expect(settings).toBeVisible();
    await expect(
      toolbar.compareDocumentPosition(settings) &
        Node.DOCUMENT_POSITION_FOLLOWING,
    ).toBeTruthy();
    await expect(
      canvas.queryByRole("navigation", { name: "Application navigation" }),
    ).toBeNull();
  }}
/>

<Story
  name="Denied reload keeps native commands without exposing activation"
  args={{ denied: true, initialPage: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Application menu" }),
    );
    await waitFor(async () => {
      await expect(
        await body.findByRole("menuitem", { name: "Create change" }),
      ).toBeVisible();
    });
    await expect(
      body.queryByRole("menuitem", { name: "Reload saved settings" }),
    ).toBeNull();
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Unknown reload retains original intent when the trigger relocates"
  args={{ lost: true, displayMode: "auto" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Application menu" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Reload saved settings" }),
    );
    const dialog = await body.findByRole("dialog", {
      name: "Reload saved settings",
    });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Reload saved settings" }),
    );
    const check = await within(dialog).findByRole("button", {
      name: "Check original outcome",
    });
    await waitFor(async () => {
      await expect(
        await body.findByText("Configuration reload", { selector: "strong" }),
      ).toBeVisible();
    });
    try {
      await page.viewport(390, 844);
      await waitFor(() =>
        expect(
          canvas.queryByRole("navigation", { name: "Application navigation" }),
        ).toBeNull(),
      );
      await expect(check).toBeVisible();
      await userEvent.click(check);
      await waitFor(() =>
        expect(canvas.getByText("Submitted 1 · Checked 1")).toBeInTheDocument(),
      );
      await userEvent.keyboard("{Escape}");
      await waitFor(() =>
        expect(
          canvas.getByRole("button", { name: "Application menu" }),
        ).toHaveFocus(),
      );
    } finally {
      await page.viewport(1280, 800);
    }
  }}
/>

<Story
  name="One desktop menu includes reload alongside repository actions"
  args={{ initialPage: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const menu = await canvas.findByRole("button", {
      name: "Application menu",
    });
    await expect(
      canvas.queryByRole("button", { name: "Repository actions" }),
    ).toBeNull();
    await expect(
      menu.closest('[aria-label="Application navigation"]'),
    ).not.toBeNull();
    await userEvent.click(menu);
    await waitFor(async () => {
      await expect(
        within(document.body).getByRole("menuitem", { name: "Create change" }),
      ).toBeVisible();
    });
    await userEvent.click(
      await within(document.body).findByRole("menuitem", {
        name: "Reload saved settings",
      }),
    );
    const dialog = await within(document.body).findByRole("dialog", {
      name: "Reload saved settings",
    });
    await waitFor(async () => {
      await expect(
        within(dialog).getByRole("button", { name: "Reload saved settings" }),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(menu).toHaveFocus());
  }}
/>
<Story
  name="Mobile hides top navigation and locates its one menu in the body"
  args={{ displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const menu = await canvas.findByRole("button", {
      name: "Application menu",
    });
    await expect(
      canvas.queryByRole("navigation", { name: "Application navigation" }),
    ).toBeNull();
    await expect(menu.closest('[data-ui-part="main"]')).not.toBeNull();
    await expect(menu.closest('[data-ui-part="toolbar"]')).not.toBeNull();
    await expect(
      canvas.getByRole("button", { name: "Open navigation sidebar" }),
    ).toBeVisible();
    await expect(
      canvas.getAllByRole("button", { name: "Application menu" }),
    ).toHaveLength(1);
    await userEvent.click(menu);
    await userEvent.click(
      await within(document.body).findByRole("menuitem", {
        name: "Reload saved settings",
      }),
    );
    await waitFor(async () => {
      await expect(
        await within(document.body).findByRole("dialog", {
          name: "Reload saved settings",
        }),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(menu).toHaveFocus());
  }}
/>

<Story
  name="Reload dialog and focus survive desktop to mobile relocation"
  args={{ displayMode: "auto" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Application menu" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Reload saved settings" }),
    );
    const dialog = await body.findByRole("dialog", {
      name: "Reload saved settings",
    });
    try {
      await page.viewport(390, 844);
      await waitFor(() =>
        expect(
          canvas.queryByRole("navigation", { name: "Application navigation" }),
        ).toBeNull(),
      );
      await expect(body.getAllByRole("dialog")).toHaveLength(1);
      await waitFor(async () => {
        await expect(dialog).toBeVisible();
      });
      await userEvent.keyboard("{Escape}");
      await waitFor(() =>
        expect(
          canvas.getByRole("button", { name: "Application menu" }),
        ).toHaveFocus(),
      );
      await expect(
        canvasElement.ownerDocument.documentElement.scrollWidth,
      ).toBeLessThanOrEqual(390);
    } finally {
      await page.viewport(1280, 800);
    }
  }}
/>

<Story
  name="Native change draft survives mobile to desktop relocation"
  args={{ displayMode: "auto", initialPage: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    try {
      await page.viewport(390, 844);
      await userEvent.click(
        await canvas.findByRole("button", { name: "Application menu" }),
      );
      await userEvent.click(
        await body.findByRole("menuitem", { name: "Create change" }),
      );
      const dialog = await body.findByRole("dialog", { name: "Create change" });
      const description = within(dialog).getByRole("textbox", {
        name: "Markdown editor",
      });
      await userEvent.type(
        description,
        "Keep this draft across the breakpoint",
      );
      await page.viewport(1280, 800);
      await waitFor(() =>
        expect(
          canvas.getByRole("navigation", {
            name: "Application navigation",
            hidden: true,
          }),
        ).toBeInTheDocument(),
      );
      await expect(description).toHaveTextContent(
        "Keep this draft across the breakpoint",
      );
      await expect(body.getAllByRole("dialog")).toHaveLength(1);
      await userEvent.keyboard("{Escape}");
      await waitFor(() =>
        expect(
          canvas.getByRole("button", { name: "Application menu" }),
        ).toHaveFocus(),
      );
    } finally {
      await page.viewport(1280, 800);
    }
  }}
/>
