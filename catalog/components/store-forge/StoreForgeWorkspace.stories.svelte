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
    await expect(
      within(
        within(stack).getByRole("button", {
          name: "Pull request for feature/api",
        }),
      ).getByRole("img", { name: "GitHub" }),
    ).toBeInTheDocument();
    const panel = await canvas.findByRole("region", {
      name: "Pull request details",
    });
    panel.scrollIntoView({ block: "nearest", inline: "end" });
    const stackHeader = stack.querySelector("header")!;
    const panelHeader = panel.querySelector("header")!;
    await expect(panelHeader).toHaveAttribute("data-ui-part", "column-header");
    await expect(panelHeader.getBoundingClientRect().height).toBe(
      stackHeader.getBoundingClientRect().height,
    );
    await expect(getComputedStyle(panelHeader).backgroundColor).toBe(
      getComputedStyle(stackHeader).backgroundColor,
    );
    const close = within(panelHeader).getByRole("button", {
      name: "Close pull requests",
    });
    await expect(close.textContent?.trim()).toBe("");
    await expect(close.querySelector("svg")).toBeInTheDocument();
    await expect(
      within(panel).getByRole("heading", { name: "Stack" }),
    ).toBeInTheDocument();
    await expect(
      within(panel).getByRole("link", { name: "#41" }),
    ).toHaveAttribute("href", "https://github.com/team/repo/pull/41");
    await expect(
      within(panel).getByRole("heading", { name: "Expose repository API" }),
    ).toBeVisible();
    await expect(
      within(panel).getByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("running");
    await expect(
      within(panel).getByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("2 successful · 1 in progress");
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
      within(panel).getByRole("heading", { name: "Add workspace toolbar" }),
    ).toBeVisible();
    await userEvent.click(
      within(panel).getByRole("button", { name: "Close pull requests" }),
    );
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
    const composer = within(preview).getByRole("group", {
      name: "PR description for feature/api",
    });
    const description = within(composer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(description);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(description, "## API review");
    await userEvent.click(
      within(composer.closest("fieldset")!).getByRole("button", {
        name: "Preview description",
      }),
    );
    await expect(
      within(preview).getByRole("heading", { name: "API review" }),
    ).toBeVisible();
    await expect(
      within(preview).getByRole("button", { name: "Push branches and submit" }),
    ).toBeEnabled();
  }}
/>
