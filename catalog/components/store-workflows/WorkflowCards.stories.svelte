<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StoreWorkflows from "../../../web/src/lib/ui/store-workflows/StoreWorkflows.svelte";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow cards",
    component: StoreWorkflows,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Description stays in Form and YAML and survives save"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: /Build parser Bookmark|View Build parser/,
      }),
    );
    let dialog = within(
      await body.findByRole("dialog", { name: "Edit workflow" }),
    );
    const description = dialog.getByRole("textbox", { name: "Description" });
    await expect(description).toHaveValue(
      "Build and test the recorded revision.",
    );
    await userEvent.clear(description);
    await userEvent.type(description, "Build the parser and report results.");
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("Build the parser and report results.");
    await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
    await expect(
      dialog.getByRole("textbox", { name: "Description" }),
    ).toHaveValue("Build the parser and report results.");
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    await waitFor(() =>
      expect(
        getComputedStyle(
          canvas.getByRole("button", { name: "View Build parser" }),
        ).pointerEvents,
      ).not.toBe("none"),
    );
    await userEvent.click(
      await canvas.findByRole("button", { name: "View Build parser" }),
    );
    dialog = within(await body.findByRole("dialog", { name: "Edit workflow" }));
    await expect(
      dialog.getByRole("textbox", { name: "Description" }),
    ).toHaveValue("Build the parser and report results.");
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(
        getComputedStyle(
          canvas.getByRole("button", { name: "Create workflow" }),
        ).pointerEvents,
      ).not.toBe("none"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Create workflow" }),
    );
    dialog = within(
      await body.findByRole("dialog", { name: "Create workflow" }),
    );
    await expect(
      dialog.getByRole("textbox", { name: "Description" }),
    ).toHaveValue("");
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>

<Story
  name="Fixed height cards summarize actions and keep controls reachable"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("button", {
      name: /View Build parser|Build parser Bookmark/,
    });
    const cards = [...canvasElement.querySelectorAll(".workflow-card")];
    const create = canvas.getByRole("button", { name: "Create workflow" });
    for (const card of cards)
      await expect(card.getBoundingClientRect().height).toBe(
        create.getBoundingClientRect().height,
      );
    await expect(
      within(canvas.getByRole("article", { name: "Build parser" })).getByRole(
        "heading",
        {
          name: "When bookmark “main” changes, run “build”",
        },
      ),
    ).toBeVisible();
    await expect(
      canvas.getByRole("switch", { name: "Enable Agent review" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Actions for Build parser" }),
    );
    await expect(
      await within(document.body).findByRole("menuitem", {
        name: "Edit workflow",
      }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>

<Story
  name="Ordered action stack and fixed cards at mobile width"
  parameters={{ viewport: { defaultViewport: "mobile390" } }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const card = await canvas.findByRole("article", { name: "Build parser" });
    await expect(
      canvas.getByRole("heading", {
        name: "When bookmark “main” changes, run “build”, then 2 more steps",
      }),
    ).toBeVisible();
    const stack = card.querySelector(".workflow-card__action-stack")!;
    await expect(stack.querySelectorAll("[data-layer]")).toHaveLength(3);
    const tiles = [...stack.querySelectorAll("[data-layer]")];
    await expect(tiles[0]!.getBoundingClientRect().right).toBeGreaterThan(
      tiles[1]!.getBoundingClientRect().left,
    );
    await expect(Number(getComputedStyle(tiles[0]!).zIndex)).toBeGreaterThan(
      Number(getComputedStyle(tiles[1]!).zIndex),
    );
    const create = canvas.getByRole("button", { name: "Create workflow" });
    await expect(card.getBoundingClientRect().height).toBe(
      create.getBoundingClientRect().height,
    );
    await expect(card.getBoundingClientRect().width).toBe(
      create.getBoundingClientRect().width,
    );
    await expect(canvasElement.scrollWidth).toBeLessThanOrEqual(
      window.innerWidth,
    );
  }}>{#snippet template()}<WorkflowsFixture multiStep />{/snippet}</Story
>
