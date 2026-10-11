<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositorySettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Repository",
    component: Fixture,
    tags: ["visual-pending"],
  });
  async function expectEmpty(canvasElement: HTMLElement) {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("Choose a repository", { exact: true }),
    ).toBeVisible();
    await expect(canvas.queryByRole("tab")).not.toBeInTheDocument();
    await expect(
      canvas.queryByRole("heading", { name: "Identity and snapshot" }),
    ).not.toBeInTheDocument();
    await expect(canvas.queryAllByText("Unavailable")).toHaveLength(0);
    await expect(
      canvas.getAllByRole("combobox", { name: "Settings repository" }),
    ).toHaveLength(1);
    await expect(
      canvasElement.querySelector(
        '[data-ui-component="empty"] svg[aria-hidden="true"]',
      ),
    ).toBeVisible();
  }
</script>

<Story
  name="No selection hides unavailable rows and tabs and supports keyboard selection"
  play={async ({ canvasElement }) => {
    await expectEmpty(canvasElement);
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Settings repository" }),
    );
    await userEvent.type(
      body.getByRole("combobox", { name: "Select settings repository" }),
      "Managed",
    );
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(
      canvas.getByRole("heading", { name: "Identity and snapshot" }),
    ).toBeVisible();
    await expect(canvas.getByRole("tab", { name: "General" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      canvas.getByText("Server Hosted", { exact: true }),
    ).toBeVisible();
    await expect(
      canvas.queryByText("Choose a repository", { exact: true }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Projects context ignores retained repository details"
  args={{ retainedRepository: true }}
  play={async ({ canvasElement }) => await expectEmpty(canvasElement)}
/>
<Story
  name="Local JJ identity has a distinct icon and tone"
  args={{ initialRepository: "local-jj" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const chip = canvas.getByText("Local JJ", { exact: true });
    await expect(chip).toHaveAttribute("data-chip-tone", "success");
    await expect(chip.querySelector('svg[aria-hidden="true"]')).toBeVisible();
    await expect(
      canvas.getByText("Folder repository", { exact: true }),
    ).toBeVisible();
  }}
/>
<Story
  name="Server Hosted identity has a distinct icon and tone"
  args={{ initialRepository: "jj-store" }}
  play={async ({ canvasElement }) => {
    const chip = within(canvasElement).getByText("Server Hosted", {
      exact: true,
    });
    await expect(chip).toHaveAttribute("data-chip-tone", "accent");
    await expect(chip.querySelector('svg[aria-hidden="true"]')).toBeVisible();
  }}
/>
<Story
  name="Pending folder draft prevents repository selection"
  args={{ blocked: true }}
  play={async ({ canvasElement }) => {
    await expectEmpty(canvasElement);
    await expect(
      within(canvasElement).getByRole("combobox", {
        name: "Settings repository",
      }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Failed repository selection retains the empty state"
  args={{ failSelection: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Settings repository" }),
    );
    await userEvent.click(
      body.getByRole("option", { name: /Managed repository/ }),
    );
    await expect(
      await body.findByText("The repository request could not be completed."),
    ).toBeVisible();
    await expectEmpty(canvasElement);
  }}
/>
