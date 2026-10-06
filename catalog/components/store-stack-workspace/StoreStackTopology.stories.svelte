<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackTopologyFixture from "../../fixtures/StackTopologyFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Topology",
    component: StackTopologyFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
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
        name: "Commit cloud-observability",
      }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish loading fixture history" }),
    );
    await within(upper).findByRole("button", {
      name: "Commit cloud-observability",
    });
    await within(upper).findByRole("button", { name: "Commit cloud-runner" });
    expect(within(upper).getAllByText("workspace/experiment")).toHaveLength(1);
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
        within(document.body).getByText(/"unapply_branches"/),
      ).toHaveTextContent(
        '"branches":["cloud-observability","cloud-runner","workspace/experiment"]',
      ),
    );
  }}
/>
