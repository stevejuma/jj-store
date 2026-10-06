<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProjectsFeedbackFixture from "../../fixtures/ProjectsFeedbackFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects feedback",
    component: ProjectsFeedbackFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function feedback(
    canvasElement: HTMLElement,
    read: string,
    retry: string,
  ) {
    const c = within(canvasElement),
      body = within(document.body);
    const toast = await body.findByRole("alert");
    await expect(toast).toHaveTextContent(`${read} temporarily unavailable`);
    expect(toast.closest("[data-notification-toasts]")).not.toBeNull();
    const content = c.getByRole("region", { name: "Projects and tasks" });
    await expect(within(content).queryByRole("alert")).toBeNull();
    await expect(body.getByRole("button", { name: retry })).toBeVisible();
    const id = toast.getAttribute("data-notification-id");
    await userEvent.click(body.getByRole("button", { name: retry }));
    await waitFor(() => expect(body.getAllByRole("alert")).toHaveLength(1));
    await expect(body.getByRole("alert")).toHaveAttribute(
      "data-notification-id",
      id!,
    );
    await userEvent.click(c.getByRole("button", { name: "Recover read" }));
    await waitFor(() => expect(body.queryByRole("alert")).toBeNull());
  }
</script>

<Story
  name="Selected project task failures use retry toasts"
  args={{ selectedProject: true }}
  play={async ({ canvasElement }) =>
    feedback(canvasElement, "browse", "Retry home")}
/>
<Story
  name="Activity association failures use warning toasts"
  args={{ tab: "activity", activityContextError: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await c.findByRole("list", { name: "Repository activity" });
    await userEvent.click(
      c.getByRole("button", { name: "Filters for activity" }),
    );
    await userEvent.click(
      body.getByRole("option", { name: /With agent context/ }),
    );
    await userEvent.click(body.getByRole("button", { name: "Close filters" }));
    await waitFor(() =>
      expect(
        document.querySelector(
          '[data-notification-toasts] [data-severity="warning"]',
        ),
      ).toBeVisible(),
    );
    const toast = document.querySelector<HTMLElement>(
      '[data-notification-toasts] [data-severity="warning"]',
    )!;
    await expect(toast).toHaveAttribute("role", "status");
    await expect(toast).toHaveAttribute("data-severity", "warning");
    await expect(toast).toHaveTextContent(
      "Agent context unavailable for 1 commits",
    );
    await expect(
      within(c.getByRole("region", { name: "Projects and tasks" })).queryByRole(
        "alert",
      ),
    ).toBeNull();
    await expect(
      within(toast).getByRole("button", { name: "Retry activity" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Projects failures use retry toasts"
  play={async ({ canvasElement }) => {
    await feedback(canvasElement, "browse", "Retry home");
    await expect(
      within(canvasElement).getByRole("button", {
        name: "Open project Native release",
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Tasks failures use retry toasts"
  args={{ tab: "tasks" }}
  play={async ({ canvasElement }) => {
    await feedback(canvasElement, "browse", "Retry home");
    await expect(
      within(canvasElement).getAllByRole("button", {
        name: "Open task Browser work",
      }).length,
    ).toBeGreaterThan(0);
  }}
/>
<Story
  name="Reviews failures use retry toasts"
  args={{ tab: "reviews", failedRead: "reviews" }}
  play={async ({ canvasElement }) =>
    feedback(canvasElement, "reviews", "Retry reviews")}
/>
<Story
  name="Agents failures use retry toasts"
  args={{ tab: "agents", failedRead: "agents" }}
  play={async ({ canvasElement }) =>
    feedback(canvasElement, "agents", "Retry agent sessions")}
/>
<Story
  name="Agent scope failures use retry toasts"
  args={{ tab: "agents" }}
  play={async ({ canvasElement }) =>
    feedback(canvasElement, "browse", "Retry project scope")}
/>
<Story
  name="Activity failures use retry toasts"
  args={{ tab: "activity", failedRead: "activity" }}
  play={async ({ canvasElement }) =>
    feedback(canvasElement, "activity", "Retry activity")}
/>
<Story
  name="Repository failures use retry toasts"
  args={{ tab: "repositories", failedRead: "directory" }}
  play={async ({ canvasElement }) => {
    const body = within(document.body),
      c = within(canvasElement);
    const toast = await body.findByRole("alert");
    expect(toast.closest("[data-notification-toasts]")).not.toBeNull();
    await expect(toast).toHaveTextContent("directory temporarily unavailable");
    await expect(
      within(c.getByRole("region", { name: "Projects and tasks" })).queryByRole(
        "alert",
      ),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Recover read" }));
    await waitFor(() => expect(body.queryByRole("alert")).toBeNull());
  }}
/>
<Story
  name="Dismissal survives retry and tabs until hidden recovery"
  args={{ tab: "reviews", failedRead: "reviews" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const toast = await body.findByRole("alert");
    expect(toast.closest("[data-notification-toasts]")).not.toBeNull();
    await userEvent.click(
      within(toast).getByRole("button", { name: /^Dismiss/ }),
    );
    await userEvent.click(c.getByRole("button", { name: "Pause read" }));
    await expect(body.queryByRole("alert")).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Finish read" }));
    await userEvent.click(c.getByRole("tab", { name: "Projects" }));
    await userEvent.click(c.getByRole("tab", { name: "Reviews" }));
    await expect(body.queryByRole("alert")).toBeNull();
    await userEvent.click(c.getByRole("tab", { name: "Projects" }));
    await userEvent.click(
      c.getByRole("button", { name: "Recover hidden reviews" }),
    );
    await waitFor(() =>
      expect(c.getByLabelText("Hidden review result")).toHaveTextContent(
        "recovered",
      ),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Fail hidden reviews" }),
    );
    await waitFor(() =>
      expect(c.getByLabelText("Hidden review result")).toHaveTextContent(
        "failed",
      ),
    );
    await userEvent.click(c.getByRole("tab", { name: "Reviews" }));
    await expect(await body.findByRole("alert")).toHaveTextContent(
      "reviews temporarily unavailable",
    );
  }}
/>
