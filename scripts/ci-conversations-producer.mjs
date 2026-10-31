/** The scoped npm workflow explicitly invokes the unchanged qualification graph. */
export function isConversationsProducer(run) {
  if (run.path !== ".github/workflows/npm-publish-jj-conversations.yml")
    return false;
  return (
    (run.event === "workflow_dispatch" && run.head_branch === "main") ||
    (run.event === "push" &&
      /^jj-conversations-v\d+\.\d+\.\d+(?:-[\w.-]+)?$/.test(
        run.head_branch ?? "",
      ))
  );
}
