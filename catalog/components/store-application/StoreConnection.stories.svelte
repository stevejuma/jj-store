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
    search: "idle",
    agents: "idle",
    transcript: "idle",
    reviews: "idle",
    reviewAction: "idle",
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
  args={{ status: disconnected, storageReady: true, onConnect: fn() }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    await userEvent.type(canvas.getByLabelText("Access token"), "secret-token");
    const submit = canvas.getByRole("button", { name: "Continue" });
    await waitFor(() => expect(submit).toBeEnabled());
    await userEvent.click(submit);
    await waitFor(() =>
      expect(args.onConnect).toHaveBeenCalledWith({
        repository: "",
        token: "secret-token",
      }),
    );
    await expect(canvas.getByLabelText("Access token")).toHaveValue("");
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
    storageReady: true,
    onConnect: fn(),
  }}
/>

<Story
  name="Authorized repositories"
  args={{
    status: { ...disconnected, repositories: "ready" },
    storageReady: true,
    repositories: [
      { id: "12".repeat(16), name: "lapis/notes" },
      { id: "34".repeat(16), name: "lapis/community" },
    ],
    onConnect: fn(),
  }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: /lapis\/community/ }),
    );
    await waitFor(() =>
      expect(args.onConnect).toHaveBeenCalledWith({
        repository: "34".repeat(16),
        token: null,
      }),
    );
  }}
/>

<Story
  name="Bootstrapping"
  args={{
    status: { ...disconnected, connection: "connecting" },
    accessMode: "bootstrapping",
    onConnect: fn(),
  }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByRole("status", {
        name: /loading installation status/i,
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Local repositories"
  args={{
    status: { ...disconnected, repositories: "ready" },
    accessMode: "local_trusted",
    storageReady: true,
    repositories: [
      { id: "12".repeat(16), name: "Notes" },
      { id: "34".repeat(16), name: "Design system" },
    ],
    onConnect: fn(),
  }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /Notes/ }));
    await waitFor(() =>
      expect(args.onConnect).toHaveBeenCalledWith({
        repository: "12".repeat(16),
        token: null,
      }),
    );
  }}
/>

<Story
  name="Empty local catalog"
  args={{
    status: { ...disconnected, repositories: "empty" },
    accessMode: "local_trusted",
    storageReady: true,
    repositories: [],
    onConnect: fn(),
  }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByText("No registered repositories"),
    ).toBeVisible();
  }}
/>
