<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import WorkspaceSettingsFixture from "../../fixtures/WorkspaceSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Local folders",
    component: WorkspaceSettingsFixture,
    tags: ["visual-pending"],
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
      canvas.getByText(
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
