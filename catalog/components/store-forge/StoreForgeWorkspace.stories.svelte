<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Workspace",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
</script>

<Story
  name="Pull request details beside stacks"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await expect(
      canvas.getByRole("region", { name: "Stack feature/workspace" }),
    ).toBeInTheDocument();
    await expect(
      within(stack).getByRole("button", {
        name: "Pull request for feature/api",
      }),
    ).toHaveTextContent("#42");
    await expect(within(stack).getByText("Checks running")).toBeInTheDocument();
    const panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    panel.scrollIntoView({ block: "nearest", inline: "end" });
    await expect(
      within(panel).getByRole("textbox", { name: "Title" }),
    ).toHaveValue("Expose repository API");
    await expect(
      within(panel).getByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("running");
    await expect(panel.getBoundingClientRect().left).toBeGreaterThanOrEqual(
      stack.getBoundingClientRect().right - 1,
    );
  }}
/>

<Story
  name="Open another PR and preview stack submission"
  args={{ panelOpen: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/workspace",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Pull request for feature/workspace",
      }),
    );
    let panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    await expect(
      within(panel).getByRole("textbox", { name: "Title" }),
    ).toHaveValue("Add workspace toolbar");
    await userEvent.click(within(panel).getByRole("button", { name: "Close" }));
    const api = canvas.getByRole("region", { name: "Stack feature/api" });
    await userEvent.click(
      within(api).getByRole("button", { name: "Submit stack feature/api" }),
    );
    panel = await canvas.findByRole("region", { name: "Pull request details" });
    panel.scrollIntoView({ block: "nearest", inline: "end" });
    const preview = within(panel).getByRole("region", {
      name: "Stack submission preview",
    });
    await expect(preview).toHaveTextContent("feature/model → main");
    await expect(preview).toHaveTextContent("feature/api → feature/model");
    await expect(
      within(preview).getByRole("button", { name: "Push branches and submit" }),
    ).toBeEnabled();
  }}
/>
