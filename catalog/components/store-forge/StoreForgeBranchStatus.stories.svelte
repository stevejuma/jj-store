<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Branch status",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    args: { panelOpen: false },
    beforeEach: async () => {
      await views.stacks();
    },
  });
</script>

<Story
  name="Top branch shows compact PR and running checks below its title"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const row = await within(stack).findByRole("group", {
      name: "Branch status for feature/api",
    });
    const title = within(stack).getByRole("button", {
      name: "Show all changes in feature/api",
    });
    await expect(row.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      title.getBoundingClientRect().bottom,
    );
    await expect(within(row).getByText("Changes to push")).toBeVisible();
    const pr = within(row).getByRole("button", {
      name: "Pull request for feature/api",
    });
    await expect(pr).toHaveTextContent("PR #42");
    await expect(pr.getBoundingClientRect().height).toBe(
      within(stack)
        .getByRole("button", { name: "Submit stack feature/api" })
        .getBoundingClientRect().height,
    );
    const checks = within(row).getByRole("status", { name: "Checks running" });
    await expect(checks).toHaveTextContent("Running");
    await expect(
      checks.querySelector('[data-ui-component="spinner"]'),
    ).toBeInTheDocument();
    await userEvent.click(pr);
    await expect(
      await canvas.findByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(title).toHaveAttribute("aria-pressed", "false");
  }}
/>

<Story
  name="Published branch shows passed checks"
  args={{ pushState: "synced", apiCheckState: "passed" }}
  play={async ({ canvasElement }) => {
    const stack = await within(canvasElement).findByRole("region", {
      name: "Stack feature/api",
    });
    const row = within(stack).getByRole("group", {
      name: "Branch status for feature/api",
    });
    await expect(within(row).getByText("Nothing to push")).toBeVisible();
    const checks = within(row).getByRole("status", { name: "Checks passed" });
    await expect(checks).toHaveTextContent("Passed");
    await expect(
      checks.querySelector('[data-ui-component="spinner"]'),
    ).toBeNull();
  }}
/>
<Story
  name="Dependent branch shows compact PR and checks below its title"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const row = await within(stack).findByRole("group", {
      name: "Branch status for feature/model",
    });
    const title = within(stack).getByRole("button", {
      name: "Show all changes in feature/model",
    });
    await expect(row.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      title.getBoundingClientRect().bottom,
    );
    const pr = within(row).getByRole("button", {
      name: "Pull request for feature/model",
    });
    await expect(pr).toHaveTextContent("PR #41");
    await expect(within(row).getByText("Nothing to push")).toBeVisible();
    await expect(
      within(row).getByRole("status", { name: "Checks passed" }),
    ).toBeVisible();
    await userEvent.click(pr);
    await expect(
      await canvas.findByRole("region", { name: "PR 41" }),
    ).toBeVisible();
    await expect(title).toHaveAttribute("aria-pressed", "false");
  }}
/>
<Story
  name="Local changes and failed checks remain distinct"
  args={{ apiCheckState: "failed" }}
  play={async ({ canvasElement }) => {
    const stack = await within(canvasElement).findByRole("region", {
      name: "Stack feature/api",
    });
    const row = within(stack).getByRole("group", {
      name: "Branch status for feature/api",
    });
    await expect(within(row).getByText("Changes to push")).toBeVisible();
    await expect(
      within(row).getByRole("status", { name: "Checks failed" }),
    ).toHaveTextContent("Failed");
  }}
/>
<Story
  name="Unknown push and incomplete checks stay explicit"
  args={{
    pushState: "unknown",
    apiChecksComplete: false,
    apiCheckState: "passed",
  }}
  play={async ({ canvasElement }) => {
    const stack = await within(canvasElement).findByRole("region", {
      name: "Stack feature/api",
    });
    const row = within(stack).getByRole("group", {
      name: "Branch status for feature/api",
    });
    await expect(within(row).getByText("Push status unknown")).toBeVisible();
    await expect(
      within(row).getByRole("status", { name: "Checks unknown" }),
    ).toHaveTextContent("Unknown");
    await expect(
      within(row).queryByText("Passed", { exact: true }),
    ).not.toBeInTheDocument();
  }}
/>
