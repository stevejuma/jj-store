import type {
  RepositoryChoice,
  StoreControllerState,
} from "../../controller/contracts.js";

export function workspaceSnapshotChoices(
  view: StoreControllerState,
): readonly RepositoryChoice[] {
  const copy = view.workingCopies.find(
    (item) => item.workspace === view.route.workspace,
  );
  return copy
    ? [
        {
          value: `working-copy:${copy.workspace}`,
          label: `Working copy · ${copy.workspace}`,
          kind: "change",
          targets: [copy.commitId],
        },
        ...view.choices,
      ]
    : view.choices;
}

export function selectedWorkspaceSnapshot(view: StoreControllerState) {
  if (view.route.bookmark) return `bookmark:${view.route.bookmark}`;
  if (view.route.workspace && !view.route.commitId && !view.route.changeId)
    return `working-copy:${view.route.workspace}`;
  if (!view.route.changeId) return "";
  return (
    view.choices.find(
      (choice) =>
        choice.kind === "change" &&
        (choice.value === `change:${view.route.changeId}` ||
          (view.route.commitId !== null &&
            choice.targets.includes(view.route.commitId))),
    )?.value ?? `change:${view.route.changeId}`
  );
}
