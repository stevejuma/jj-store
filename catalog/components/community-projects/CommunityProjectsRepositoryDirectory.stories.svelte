<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import CommunityProjectsRepositoryDirectory from "../../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryDirectory.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Components/Repository directory",
    component: CommunityProjectsRepositoryDirectory,
    tags: ["visual-pending"],
    args: {
      now: 0,
      currentPath: "",
      entries: [
        { type: "blob", name: "README.md", path: "README.md", size: 42 },
      ],
      pathCommits: {},
      latestCommit: {
        eventId:
          "912c8df13998f422355759647d11b9143ed1fdfd1a9bd8aa3059d69f1f170cd6",
        changeId: "5d0e9e4c650e8afe31009778e5f85040",
        parents: [],
        message:
          "Describe the repository landing page with a deliberately long summary",
        reason: "checkpoint" as const,
        createdAt: 1_700_000_000_000,
        root: "root",
        author: "Maya Chen",
        authorEmail: "maya@example.invalid",
        empty: false,
        bookmarks: [],
        workspaces: [],
        conflicted: false,
        divergent: false,
      },
      branch: "main",
      branches: [],
      previewDocuments: [
        { path: "SECURITY.md", contents: "# Security\n\nReport privately" },
        { path: "LICENCE", contents: "# License\n\nApache-2.0" },
        {
          path: "CONTRIBUTING.md",
          contents: "# Contributing\n\nOpen a change",
        },
        {
          path: "CODE_OF_CONDUCT.md",
          contents: "# Code of conduct\n\nBe considerate",
        },
        { path: "README.md", contents: "# Repository\n\nNative history" },
      ],
      onOpenDirectory: fn(),
      onOpenFile: fn(),
      onEditFile: fn(),
      onSelectCommit: fn(),
      onSelectBranch: fn(),
    },
  });
</script>

<Story
  name="Repository document tabs"
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    const commitDetails = canvas.getByRole("button", {
      name: "Open commit details",
    });
    const commitMessage = canvas.getByRole("button", {
      name: /View commit Describe the repository landing page/,
    });
    const revisionId = canvas.getByRole("button", {
      name: /Copy full Git revision/,
    });
    await expect(commitDetails).toBeVisible();
    await userEvent.click(commitDetails);
    await expect(args.onSelectCommit).toHaveBeenCalledWith(args.latestCommit);
    const tabs = canvas.getAllByRole("tab");
    await expect(tabs.map((tab) => tab.textContent?.trim())).toEqual([
      "README",
      "Code of conduct",
      "Contributing",
      "License",
      "Security",
    ]);
    const readme = canvas.getByRole("tab", { name: "README" });
    const license = canvas.getByRole("tab", { name: "License" });
    const security = canvas.getByRole("tab", { name: "Security" });
    await expect(readme).toHaveAttribute("aria-selected", "true");
    await expect(getComputedStyle(readme).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(
      Number.parseFloat(getComputedStyle(readme).borderRadius),
    ).toBeGreaterThan(0);
    const indicator = readme.querySelector<HTMLElement>(
      ".community-projects__repository-document-tab-indicator",
    )!;
    await expect(getComputedStyle(indicator).height).toBe("3px");
    await waitFor(() => expect(getComputedStyle(indicator).opacity).toBe("1"));
    await expect(getComputedStyle(license).backgroundColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
    await userEvent.hover(license);
    await expect(license).toBeVisible();
    await userEvent.click(license);
    await expect(license).toHaveAttribute("aria-selected", "true");
    await expect(
      canvas.getByRole("region", { name: "License preview" }),
    ).toHaveTextContent("Apache-2.0");
    const edit = canvas.getByRole("button", { name: "Edit License" });
    await expect(edit).toHaveTextContent("");
    await userEvent.click(edit);
    await expect(args.onEditFile).toHaveBeenCalledWith("LICENCE");
    await userEvent.click(security);
    await expect(
      canvas.getByRole("region", { name: "Security preview" }),
    ).toHaveTextContent("Report privately");

    canvasElement.style.width = "31rem";
    await waitFor(() => expect(commitMessage).not.toBeVisible());
    await expect(revisionId).not.toBeVisible();
    await expect(commitDetails).toBeVisible();
    const summary = canvasElement.querySelector<HTMLElement>(
      ".community-projects__file-browser-summary",
    )!;
    await expect(summary.scrollWidth).toBeLessThanOrEqual(summary.clientWidth);
    await waitFor(() =>
      expect(
        canvas.getByRole("button", { name: "More repository documents" }),
      ).toBeVisible(),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "More repository documents" }),
    );
    await userEvent.click(body.getByRole("menuitem", { name: "Security" }));
    await expect(
      canvas.getByRole("region", { name: "Security preview" }),
    ).toBeVisible();
    const more = canvas.getByRole("button", {
      name: "More repository documents",
    });
    await expect(more).toHaveAttribute("data-selected");
    await expect(getComputedStyle(more).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    const moreIndicator = more.querySelector<HTMLElement>(
      ".community-projects__repository-document-tab-indicator",
    )!;
    await waitFor(() =>
      expect(getComputedStyle(moreIndicator).opacity).toBe("1"),
    );
    await userEvent.click(more);
    const selectedMenuItem = body.getByRole("menuitem", { name: "Security" });
    await expect(selectedMenuItem).toHaveAttribute("data-selected");
    await expect(getComputedStyle(selectedMenuItem).fontWeight).toBe("600");
    await expect(selectedMenuItem.querySelector("svg")).toBeNull();
    await userEvent.keyboard("{Escape}");
    canvasElement.style.width = "";
  }}
/>
