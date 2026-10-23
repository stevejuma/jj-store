<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/RuntimeReloadFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Runtime reload",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-settings/StoreRuntimeReload.svelte",
      },
      layout: "fullscreen",
    },
  });
  async function open(canvasElement: HTMLElement) {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Application menu" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("menuitem", {
        name: "Reload saved settings",
      }),
    );
    return within(
      await within(document.body).findByRole("dialog", {
        name: "Reload saved settings",
      }),
    );
  }
</script>

<Story
  name="Apply saved settings"
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await userEvent.click(
      dialog.getByRole("button", { name: "Reload saved settings" }),
    );
    await waitFor(() => expect(dialog.getAllByText("Applied")).toHaveLength(6));
    expect(
      within(canvasElement).getByText("Submitted 1 · Checked 0"),
    ).toBeInTheDocument();
    await userEvent.click(
      dialog.getByRole("button", { name: "Close configuration reload" }),
    );
  }}
/>
<Story
  name="Original outcome recovery"
  args={{ lost: true }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await userEvent.click(
      dialog.getByRole("button", { name: "Reload saved settings" }),
    );
    await userEvent.click(
      await dialog.findByRole("button", { name: "Check original outcome" }),
    );
    await waitFor(() =>
      expect(
        within(canvasElement).getByText("Submitted 1 · Checked 1"),
      ).toBeInTheDocument(),
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Close configuration reload" }),
    );
  }}
/>
<Story
  name="Active session deferral"
  args={{ deferred: true }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await userEvent.click(
      dialog.getByRole("button", { name: "Reload saved settings" }),
    );
    expect(
      await dialog.findByText(/Active sessions retain/),
    ).toBeInTheDocument();
    await userEvent.click(
      dialog.getByRole("button", { name: "Close configuration reload" }),
    );
  }}
/>
<Story
  name="No operator controls"
  args={{ denied: true }}
  play={async ({ canvasElement }) => {
    await waitFor(() =>
      expect(
        within(canvasElement).queryByRole("button", {
          name: "Application menu",
        }),
      ).not.toBeInTheDocument(),
    );
  }}
/>
<Story
  name="Mobile"
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(
      window.innerWidth,
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Close configuration reload" }),
    );
  }}
/>

<Story
  name="Keyboard menu and focus restoration"
  play={async ({ canvasElement }) => {
    const trigger = await within(canvasElement).findByRole("button", {
      name: "Application menu",
    });
    trigger.focus();
    await userEvent.keyboard("{Enter}");
    const item = await within(document.body).findByRole("menuitem", {
      name: "Reload saved settings",
    });
    await waitFor(() => expect(item).toHaveFocus());
    await userEvent.keyboard("{Enter}");
    const dialog = await within(document.body).findByRole("dialog", {
      name: "Reload saved settings",
    });
    expect(
      within(dialog).getByRole("button", { name: "Reload saved settings" }),
    ).toBeInTheDocument();
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(trigger).toHaveFocus());
  }}
/>
