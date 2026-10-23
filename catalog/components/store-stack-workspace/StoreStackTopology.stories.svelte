<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackTopologyFixture from "../../fixtures/StackTopologyFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Topology",
    component: StackTopologyFixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
</script>

<Story
  name="Native lanes keep dependent aliases together and wait for complete graph pages"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const upper = await canvas.findByRole("region", {
      name: "Stack cloud-observability",
    });
    await within(upper).findByRole("status", {
      name: "Loading history for cloud-observability",
    });
    expect(
      canvas.queryByRole("region", { name: "Stack cloud-runner" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("region", { name: "Stack workspace/experiment" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("region", { name: "Stack bootstrap-cleanup" }),
    ).toBeNull();
    expect(
      canvas.queryByRole("region", { name: "Stack readme-polish" }),
    ).toBeNull();
    expect(
      within(upper).queryByRole("button", {
        name: "add deployment health summary",
      }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish loading fixture history" }),
    );
    await within(upper).findByRole("button", {
      name: "add deployment health summary",
    });
    await within(upper).findByRole("button", {
      name: "add deployment preview command",
    });
    await waitFor(() =>
      expect(
        within(upper)
          .getAllByText("workspace/experiment")
          .filter((element) => element.closest('[data-presentation="inline"]')),
      ).toHaveLength(1),
    );
    await canvas.findByRole("region", { name: "Stack export-json" });
    await canvas.findByRole("region", { name: "Stack query-filtering" });
    await userEvent.click(
      within(upper).getByRole("button", {
        name: "Actions for cloud-observability",
      }),
    );
    await userEvent.click(
      within(document.body).getByRole("menuitem", { name: "Unapply stack" }),
    );
    await waitFor(() =>
      expect(
        canvas.getByLabelText("Executed membership command"),
      ).toHaveTextContent('"branches":["cloud-observability"]'),
    );
  }}
/>

<Story
  name="Stack discovery hides dependent entries and keeps partial application explicit"
  args={{ partial: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("region", { name: "Stack cloud-runner" });
    expect(
      canvas.queryByRole("region", { name: "Stack cloud-observability" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish loading fixture history" }),
    );
    await userEvent.click(canvas.getByRole("tab", { name: "Branches" }));
    const branches = await canvas.findByRole("region", {
      name: "Branches",
    });
    const list = within(branches);
    expect(list.queryByRole("option", { name: "cloud-runner" })).toBeNull();
    expect(
      list.queryByRole("option", { name: "workspace/experiment" }),
    ).toBeNull();
    await list.findByText("Partially applied");
    await userEvent.type(
      list.getByRole("combobox", { name: "Bookmarks" }),
      "workspace/experiment",
    );
    await userEvent.click(
      list.getByRole("option", { name: "cloud-observability" }),
    );
    const upper = await canvas.findByRole("region", {
      name: "Stack cloud-observability",
    });
    await within(upper).findByRole("button", {
      name: "add deployment health summary",
    });
    await within(upper).findByRole("button", {
      name: "add deployment preview command",
    });
    await within(upper).findByRole("button", {
      name: "prepare cloud deployment config",
    });
    await waitFor(() =>
      expect(
        within(upper)
          .getAllByText("workspace/experiment")
          .filter((element) => element.closest('[data-presentation="inline"]')),
      ).toHaveLength(1),
    );
    expect(
      within(upper).getByRole("button", {
        name: "Apply cloud-observability",
      }),
    ).toBeEnabled();
  }}
/>

<Story
  name="Shared lower cards keep one branch draft across sibling stacks"
  args={{ shared: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Finish loading fixture history",
      }),
    );
    const upper = await canvas.findByRole("region", {
      name: "Stack cloud-observability",
    });
    const sibling = await canvas.findByRole("region", {
      name: "Stack export-json",
    });
    await waitFor(() =>
      expect(
        upper.querySelector(
          '[data-presentation="inline"] [data-tag-id="shared"]',
        ),
      ).toBeVisible(),
    );
    await waitFor(() =>
      expect(
        sibling.querySelector(
          '[data-presentation="inline"] [data-tag-id="shared"]',
        ),
      ).toBeVisible(),
    );
    await within(sibling).findByRole("button", {
      name: "prepare cloud deployment config",
    });
    await userEvent.click(
      within(upper).getByRole("button", {
        name: "Add empty commit to cloud-runner",
      }),
    );
    const composer = await within(upper).findByRole("form", {
      name: /^Create commit in cloud-runner/,
    });
    await userEvent.type(
      within(composer).getByRole("textbox", { name: "Commit title" }),
      "Shared lower draft",
    );
    const other = await within(sibling).findByRole("form", {
      name: /^Create commit in cloud-runner/,
    });
    expect(
      within(other).getByRole("textbox", { name: "Commit title" }),
    ).toHaveValue("Shared lower draft");
  }}
/>
