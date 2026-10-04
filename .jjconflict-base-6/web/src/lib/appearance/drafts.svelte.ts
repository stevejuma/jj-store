import { getContext, setContext } from "svelte";
export interface ConversationDraft {
  value: string;
  pendingRequestId?: string | null;
  editing?: string;
  replying?: string;
  open?: boolean;
}
const key = Symbol("conversation-drafts");
/** Memory-only drafts survive view remounts, scoped to the exact repository and conversation. */
export function createDrafts(repository: () => string = () => "") {
  const drafts = new Map<string, ConversationDraft>();
  return {
    get(id: string) {
      const name = JSON.stringify([repository(), id]);
      let draft = drafts.get(name);
      if (!draft) {
        const created = $state<ConversationDraft>({ value: "" });
        draft = created;
        drafts.set(name, draft);
      }
      return draft;
    },
  };
}
export function provideDrafts(repository: () => string) {
  const drafts = createDrafts(repository);
  setContext(key, drafts);
  return drafts;
}
export function useDrafts() {
  return (
    getContext<ReturnType<typeof createDrafts> | undefined>(key) ??
    createDrafts()
  );
}
