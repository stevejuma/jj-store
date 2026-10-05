import { createHash } from "node:crypto";

export function workSeedId(repository, kind, key) {
  return createHash("sha256")
    .update(`jj-store/dev-demo-work/v1/${repository}/${kind}/${key}`)
    .digest("hex")
    .slice(0, 32);
}

export function sampleWork(
  repository,
  repositoryLinks = [repository, repository, repository],
) {
  const projects = [
    {
      key: "browser",
      title: "Native repository browser",
      body: "Bring code, projects and discussion together in the native browser.\n\nFocus on **clear navigation**, useful metadata and accessible interactions.",
      tasks: [
        {
          key: "browse",
          title: "Ship repository browsing",
          state: "in_progress",
          body: "Review repository rows and cards, activity counts and snapshot refresh.\n\n- [x] Show native identities\n- [ ] Check narrow-screen layouts",
          comments: [
            {
              key: "layout",
              body: "The list and card layouts are ready for a visual pass. Keep the native snapshot visible when switching views.",
            },
            {
              key: "filters",
              reply: "layout",
              body: "I will check the filters in the right rail and the centered project breadcrumb.",
            },
            {
              key: "keyboard",
              reply: "layout",
              body: "Please include keyboard navigation through the repository picker in that pass.",
            },
          ],
        },
        {
          key: "navigation",
          title: "Align project navigation",
          state: "open",
          parent: "browse",
          body: "Check sidebar expansion, linked repositories and Back navigation from a task.",
        },
        {
          key: "discussion",
          title: "Review task discussion",
          state: "blocked",
          parent: "browse",
          unassigned: true,
          body: "Check replies and the original-message preview while editing. Waiting for the shared composer release.",
          comments: [
            {
              key: "release",
              body: "The local composer preview is available. Registry release qualification is the remaining dependency.",
            },
          ],
        },
      ],
    },
    {
      key: "storage",
      title: "Recovery and reliability",
      body: "Exercise durable native state across cold starts and interrupted requests. Keep recovery evidence separate from presentation checks.",
      tasks: [
        {
          key: "recovery",
          title: "Verify cold recovery",
          state: "in_progress",
          body: "Reopen the sample repository and verify that projects, task state and threaded comments survive.",
          comments: [
            {
              key: "evidence",
              body: "Capture the selected WorkStore root before shutdown so the cold read can compare exact identities.",
            },
            {
              key: "receipts",
              reply: "evidence",
              body: "Also check the original command receipts after reconstruction.",
            },
          ],
        },
        {
          key: "replay",
          title: "Exercise receipt replay",
          state: "done",
          parent: "recovery",
          body: "Sample completed task: replay an identical request without creating a second publication.",
        },
        {
          key: "replication",
          title: "Plan WorkStore replication",
          state: "blocked",
          body: "WorkStore homes currently freeze the origin protocol. Define and qualify the reducer before enabling replication.",
        },
      ],
    },
    {
      key: "tools",
      title: "Developer experience",
      body: "Make the local development demo easy to explore, document and verify.",
      tasks: [
        {
          key: "setup",
          title: "Document local setup",
          state: "done",
          body: "Sample completed task: explain startup, persistent demo data and explicit reset.",
        },
        {
          key: "picker",
          title: "Check task picker keyboard navigation",
          state: "open",
          body: "Search by title, inspect task state and select a parent without leaving the keyboard.",
        },
        {
          key: "demo",
          title: "Prepare the demo walkthrough",
          state: "in_progress",
          unassigned: true,
          body: "Walk through projects, task filters, subtasks and threaded comments using the sample data.",
        },
      ],
    },
  ];
  return projects.map((project, index) => ({
    ...project,
    id: workSeedId(repository, "project", project.key),
    repository: repositoryLinks[index],
    tasks: project.tasks.map((task) => ({
      ...task,
      id: workSeedId(repository, "task", `${project.key}/${task.key}`),
      repository: task.unassigned ? null : repositoryLinks[index],
      parent: task.parent
        ? workSeedId(repository, "task", `${project.key}/${task.parent}`)
        : null,
      comments: (task.comments ?? []).map((comment) => ({
        ...comment,
        id: workSeedId(
          repository,
          "comment",
          `${project.key}/${task.key}/${comment.key}`,
        ),
        reply_to: comment.reply
          ? workSeedId(
              repository,
              "comment",
              `${project.key}/${task.key}/${comment.reply}`,
            )
          : null,
      })),
    })),
  }));
}
