<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import StoreConnection from "../../../web/src/lib/ui/store-application/StoreConnection.svelte";
  import type { StoreControllerStatus } from "../../../web/src/lib/controller/contracts.js";

  const disconnected: StoreControllerStatus = {
    connection: "disconnected",
    navigation: "idle",
    repositories: "idle",
    bookmarks: "idle",
    graph: "idle",
    files: "idle",
    detail: "idle",
    live: "disconnected",
    refreshAvailable: false,
    error: "",
  };
  const { Story } = defineMeta({
    title: "JJ Store/Application/Connection",
    component: StoreConnection,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", a11y: { test: "error" } },
  });
</script>

<Story
  name="Connect"
  args={{ status: disconnected, onConnect: fn() }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    const repository = "12".repeat(16);
    await userEvent.type(canvas.getByLabelText("Repository ID"), repository);
    await userEvent.type(
      canvas.getByLabelText("Repository token"),
      "secret-token",
    );
    const submit = canvas.getByRole("button", { name: "Open repository" });
    await waitFor(() => expect(submit).toBeEnabled());
    await userEvent.click(submit);
    await waitFor(() =>
      expect(args.onConnect).toHaveBeenCalledWith({
        repository,
        token: "secret-token",
      }),
    );
    await expect(canvas.getByLabelText("Repository token")).toHaveValue("");
  }}
/>

<Story
  name="Failed"
  args={{
    status: {
      ...disconnected,
      connection: "failed",
      error: "Repository access was denied.",
    },
    onConnect: fn(),
  }}
/>

<Story
  name="Restored repository"
  args={{
    status: disconnected,
    initialRepository: "12".repeat(16),
    onConnect: fn(),
  }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByLabelText("Repository ID"),
    ).toHaveValue("12".repeat(16));
  }}
/>
