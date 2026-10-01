import type { Model } from "../../api/client.js";
import type { StoreController } from "../../controller/contracts.js";
type Comment = Model<"ReviewComment">;
export interface ReviewDraft {
  body: string;
  reply: Comment | null;
  edit: Comment | null;
  pendingRequestId: string | null;
}
const drafts = new WeakMap<StoreController, Map<string, ReviewDraft>>();
export function reviewDraft(
  controller: StoreController,
  threadId: string,
): ReviewDraft {
  let threads = drafts.get(controller);
  if (!threads) {
    threads = new Map();
    drafts.set(controller, threads);
  }
  let draft = threads.get(threadId);
  if (!draft) {
    const created = $state<ReviewDraft>({
      body: "",
      reply: null,
      edit: null,
      pendingRequestId: null,
    });
    draft = created;
    threads.set(threadId, draft);
  }
  return draft;
}
