<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositoryPreparationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository feedback",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Preparation uses a toast and skeleton"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await expect(
      canvas.queryByText("Preparing the graph index…"),
    ).not.toBeInTheDocument();
    const toast = await page.findByRole("status", { name: "" });
    await expect(toast).toHaveTextContent("Preparing the graph index…");
    await expect(
      canvas.getByRole("status", { name: "Loading JJ log" }),
    ).toBeVisible();
    await userEvent.click(
      within(toast).getByRole("button", { name: "Cancel preparation" }),
    );
    const warning = await page.findByRole("alert");
    await expect(warning).toHaveTextContent("Cancellation is unconfirmed");
    await expect(
      canvas.queryByText(/Cancellation is unconfirmed/),
    ).not.toBeInTheDocument();
    await userEvent.click(
      within(warning).getByRole("button", { name: "Retry cancellation" }),
    );
    await expect(
      await page.findByText("Graph index preparation canceled."),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish preparation" }),
    );
    await expect(
      await page.findByText("Graph index preparation completed."),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("status", { name: "Loading JJ log" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Preparation remains available in Settings"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await page.findByText("Preparing the graph index…");
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Settings" }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    await expect(page.getByText("Preparing the graph index…")).toBeVisible();
    await expect(
      canvas.queryByText("Preparing the graph index…"),
    ).not.toBeInTheDocument();
    await expect(
      page.getByRole("button", { name: "Cancel preparation" }),
    ).toBeEnabled();
    await userEvent.click(canvas.getByRole("button", { name: "Repository" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await expect(page.getAllByText("Preparing the graph index…")).toHaveLength(
      1,
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish preparation" }),
    );
    await expect(
      await page.findByText("Graph index preparation completed."),
    ).toBeVisible();
    await expect(
      canvas.queryByText("Graph index preparation completed."),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Dismissed preparation stays quiet across pages"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await page.findByText("Preparing the graph index…");
    await userEvent.click(
      page.getByRole("button", { name: "Dismiss Graph index preparation" }),
    );
    await userEvent.click(canvas.getByRole("tab", { name: "Op Log" }));
    await expect(
      page.queryByText("Preparing the graph index…"),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("status", { name: "Loading operation log" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Commits" }));
    await expect(
      page.queryByText("Preparing the graph index…"),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Finish preparation" }),
    );
    await expect(
      await page.findByText("Graph index preparation completed."),
    ).toBeVisible();
  }}
/>
