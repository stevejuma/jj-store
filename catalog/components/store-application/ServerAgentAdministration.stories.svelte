<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page } from "vitest/browser";
  import Fixture from "../../fixtures/ServerAgentAdministrationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agents/Server administration",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-agent-execution/StoreServerAgentAdministration.svelte",
      },
      layout: "fullscreen",
    },
  });
  async function close(dialog: ReturnType<typeof within>) {
    const button = dialog.getByRole("button", {
      name: "Close server agent access",
    });
    await waitFor(() => expect(button).toBeEnabled());
    await userEvent.click(button);
    await waitFor(() =>
      expect(
        within(document.body).queryByRole("dialog", {
          name: "Server agent access",
        }),
      ).not.toBeInTheDocument(),
    );
  }
  async function open(canvasElement: HTMLElement) {
    await userEvent.click(
      within(canvasElement).getByRole("button", {
        name: "Manage server access",
      }),
    );
    const dialog = await within(document.body).findByRole("dialog", {
      name: "Server agent access",
    });
    return within(dialog);
  }
</script>

<Story
  name="Sharing and configuration"
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await dialog.findByRole("checkbox", { name: "Noah Patel" });
    await userEvent.click(dialog.getByRole("checkbox", { name: "Noah Patel" }));
    await expect(
      dialog.getByRole("checkbox", { name: "Noah Patel" }),
    ).toBeChecked();
    await userEvent.clear(dialog.getByRole("textbox", { name: /^Name$/ }));
    await userEvent.type(
      dialog.getByRole("textbox", { name: /^Name$/ }),
      "Remote builder",
    );
    const access = dialog.getByRole("tab", { name: "Access onboarding" });
    access.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      dialog.getByRole("region", { name: "Access requirements" }),
    ).toBeVisible();
    await userEvent.click(
      dialog.getByRole("tab", { name: "Configuration and sharing" }),
    );
    await expect(dialog.getByRole("textbox", { name: /^Name$/ })).toHaveValue(
      "Remote builder",
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save configuration" }),
    );
    await waitFor(() =>
      expect(
        within(canvasElement).getByLabelText("Configuration save count"),
      ).toHaveTextContent("1"),
    );
    await close(dialog);
  }}
/>
<Story
  name="Original save recovery"
  args={{ loseSave: true }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await userEvent.click(
      await dialog.findByRole("checkbox", { name: "Noah Patel" }),
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save configuration" }),
    );
    await userEvent.click(
      await dialog.findByRole("button", { name: "Check original outcome" }),
    );
    await waitFor(() =>
      expect(
        within(canvasElement).getByLabelText("Configuration save count"),
      ).toHaveTextContent("1"),
    );
    await close(dialog);
  }}
/>
<Story
  name="Identity and repository onboarding"
  args={{ identityOperator: true }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await expect(
      dialog.queryByRole("tab", { name: "Configuration and sharing" }),
    ).not.toBeInTheDocument();
    const enrollment = within(
      dialog.getByRole("region", { name: "Enroll installation identity" }),
    );
    await userEvent.type(
      enrollment.getByRole("textbox", { name: "Principal ID" }),
      "11".repeat(16),
    );
    await userEvent.type(
      enrollment.getByLabelText("User access token"),
      "synthetic-user-credential",
    );
    await userEvent.click(
      enrollment.getByRole("button", { name: "Enroll token" }),
    );
    await waitFor(() =>
      expect(
        within(canvasElement).getByLabelText("Enrollment count"),
      ).toHaveTextContent("1"),
    );
    await waitFor(() =>
      expect(enrollment.getByLabelText("User access token")).toHaveValue(""),
    );
    const grant = within(
      dialog.getByRole("region", { name: "Grant repository Write access" }),
    );
    await userEvent.click(grant.getByRole("button", { name: "Repository" }));
    await expect(
      within(document.body).queryByRole("option", {
        name: "Read only repository",
      }),
    ).not.toBeInTheDocument();
    await expect(
      within(document.body).queryByRole("option", { name: "Local repository" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      within(document.body).getByRole("option", { name: "Code repository" }),
    );
    await userEvent.type(
      grant.getByLabelText("User access token"),
      "synthetic-user-credential",
    );
    await userEvent.click(
      grant.getByRole("button", { name: "Grant Write access" }),
    );
    await waitFor(() =>
      expect(
        within(canvasElement).getByLabelText("Write grant count"),
      ).toHaveTextContent("1"),
    );
    await close(dialog);
  }}
/>
<Story
  name="Mobile"
  play={async ({ canvasElement }) => {
    await page.viewport(390, 844);
    const dialog = await open(canvasElement);
    const control = await dialog.findByRole("textbox", {
      name: "ACP executable",
    });
    await expect(control.getBoundingClientRect().right).toBeLessThanOrEqual(
      390,
    );
    await expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(390);
    await close(dialog);
    await page.viewport(1280, 800);
  }}
/>
<Story
  name="Capability discovery in progress"
  args={{ discoveryPending: true }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await dialog.findByRole("checkbox", { name: "Noah Patel" });
    await expect(
      dialog.getByRole("tab", { name: "Configuration and sharing" }),
    ).toHaveAttribute("aria-selected", "true");
    await close(dialog);
  }}
/>
<Story
  name="Dark"
  globals={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => {
    const dialog = await open(canvasElement);
    await dialog.findByRole("checkbox", { name: "Noah Patel" });
    await close(dialog);
  }}
/>
