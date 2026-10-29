<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import WorkspaceSettingsFixture from "../../fixtures/WorkspaceSettingsFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-settings/StoreSettingsPage.svelte",
      },
    },
    title: "JJ Store/Settings/Local folders",
    component: WorkspaceSettingsFixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story
  name="Link and retain failed draft"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const input = canvas.getByRole("textbox", { name: "Folder path" });
    await expect(
      canvas.getByText("Unavailable", { exact: true }),
    ).toBeVisible();
    await expect(input.closest(".store-settings__rows")).not.toBeNull();
    await expect(
      canvas.getByRole("button", { name: "Refresh folders" }),
    ).toHaveTextContent("");
    const link = canvas.getByRole("button", { name: "Link folder" });
    const cancel = canvas.getByRole("button", { name: /^Cancel$/ });
    await expect(cancel.getBoundingClientRect().top).toBeCloseTo(
      link.getBoundingClientRect().top,
      0,
    );
    const group = input.closest('[role="group"]')!;
    const row = input.closest('[data-slot="field"]')!;
    await expect(group.getBoundingClientRect().width).toBeGreaterThan(
      row.getBoundingClientRect().width * 0.58,
    );
    await userEvent.type(input, "/wrong");
    await userEvent.keyboard("{Enter}");
    await expect(
      canvas.getByText("Invalid workspace binding. Check the folder path."),
    ).toBeVisible();
    await expect(input).toHaveValue("/wrong");
    await userEvent.click(canvas.getByRole("button", { name: /^Cancel$/ }));
    await expect(input).toHaveValue("");
    await userEvent.type(input, "/workspaces/example");
    await userEvent.keyboard("{Enter}");
    await expect(
      canvas.getByText("/workspaces/example", { exact: true }),
    ).toBeVisible();
    await expect(canvas.getByText("Ready", { exact: true })).toBeVisible();
    await expect(input).toHaveValue("");
  }}
/>
<Story
  name="Joined folder form and shared input focus ring"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = c.getByRole("textbox", { name: "Folder path" });
    const browse = c.getByRole("button", { name: "Browse…" });
    const group = c.getByRole("group", { name: "Choose local folder" });
    await userEvent.click(input);
    await waitFor(() =>
      expect(getComputedStyle(group).boxShadow).toContain("3px"),
    );
    await expect(group.contains(browse)).toBe(true);
    await userEvent.type(input, "/retained draft");
    await userEvent.tab();
    await expect(browse).toHaveFocus();
    await expect(input).toHaveValue("/retained draft");
    const link = c.getByRole("button", { name: "Link folder" });
    const cancel = c.getByRole("button", { name: /^Cancel$/ });
    const card = input.closest(".store-settings__rows")!;
    await expect(link.closest(".store-settings__rows")).toBe(card);
    await expect(
      card.querySelector('[data-ui-component="separator"]'),
    ).not.toBeNull();
    const footer = link.closest("footer")!;
    await expect(link.getBoundingClientRect().right).toBeCloseTo(
      group.getBoundingClientRect().right,
      0,
    );
    await expect(cancel.getBoundingClientRect().top).toBeCloseTo(
      link.getBoundingClientRect().top,
      0,
    );
    await expect(link.getBoundingClientRect().top).toBeGreaterThan(
      footer.querySelector("p")!.getBoundingClientRect().bottom,
    );
  }}
/>
<Story
  name="Unsupported connection"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText(
        "Local folder management is not available for this connection.",
      ),
    ).toBeVisible();
    await expect(
      canvasElement.querySelector('[data-ui-part="empty-media"] svg'),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Link folder" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Browse fills draft before linking"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const input = canvas.getByRole("textbox", { name: "Folder path" });
    await userEvent.click(canvas.getByRole("button", { name: "Browse…" }));
    await expect(input).toHaveValue("/workspaces/picked folder");
    await expect(
      canvas.queryByText("Ready", { exact: true }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("button", { name: "Link folder" }));
    await expect(
      canvas.getByText("/workspaces/picked folder", { exact: true }),
    ).toBeVisible();
  }}
/>
<Story
  name="Picker cancellation preserves draft"
  args={{ selection: "cancelled" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const input = canvas.getByRole("textbox", { name: "Folder path" });
    await userEvent.type(input, "/existing draft");
    await userEvent.click(canvas.getByRole("button", { name: "Browse…" }));
    await expect(input).toHaveValue("/existing draft");
  }}
/>
<Story
  name="Picker failure keeps manual entry"
  args={{ selection: "failed" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Browse…" }));
    await expect(
      within(canvasElement.ownerDocument.body).getByText(
        "Folder picker unavailable. Enter the folder path manually.",
      ),
    ).toBeVisible();
    const input = canvas.getByRole("textbox", { name: "Folder path" });
    await userEvent.type(input, "/manual");
    await expect(input).toHaveValue("/manual");
  }}
/>
<Story
  name="Manual entry without native picker"
  args={{ pickerSupported: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.queryByRole("button", { name: "Browse…" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("textbox", { name: "Folder path" }),
    ).toBeEnabled();
  }}
/>

<Story
  name="Repository folder loading retains form and refresh retains links"
  args={{ embedded: true, loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("status", { name: "Loading local folders" }),
    ).toBeVisible();
    const input = c.getByRole("textbox", { name: "Folder path" });
    await expect(input).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    await userEvent.type(input, "/retained/workspace");
    await expect(c.getByText("/workspaces/moved-project")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Start background refresh" }),
    );
    await expect(c.getByText("/workspaces/moved-project")).toBeVisible();
    await expect(input).toHaveValue("/retained/workspace");
    await expect(
      c.queryByRole("status", { name: "Loading local folders" }),
    ).not.toBeInTheDocument();
  }}
/>
