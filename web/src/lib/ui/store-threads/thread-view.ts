import type { StoreController } from "../../controller/contracts.js";
import { principalMember } from "../../collaboration/presentation.js";
import type { CommunityMessageModel } from "../../community/workspace-model.js";
export function replyRoot(
  messages: readonly CommunityMessageModel[],
  selected: string,
) {
  const byId = new Map(messages.map((message) => [message.id, message]));
  const visited = new Set<string>();
  let id = selected;
  while (!visited.has(id)) {
    visited.add(id);
    const parent = byId.get(id)?.parentMessageId;
    if (!parent || !byId.has(parent)) break;
    id = parent;
  }
  return id;
}
export function threadMembers(controller: StoreController) {
  const actor = controller.threads?.detail?.thread.value.actor;
  return [
    ...new Map(
      [
        ...(actor ? [principalMember(actor)] : []),
        ...(controller.threads?.projection.members ?? []),
        ...(
          controller.threads?.detail?.thread.directory?.participants ?? []
        ).map(principalMember),
      ].map((m) => [m.id, m]),
    ).values(),
  ];
}
export function threadWritable(controller: StoreController) {
  return ["write", "admin"].includes(
    controller.state.repository.permission ?? "",
  );
}
export function threadDisabled(controller: StoreController) {
  return (
    !threadWritable(controller) ||
    !!controller.threads?.pending ||
    !!controller.threads?.busy ||
    ["archived", "tombstoned"].includes(
      controller.threads?.detail?.thread.value.status ?? "",
    )
  );
}
