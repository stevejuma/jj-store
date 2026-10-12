export function createMessageDeletion(
  remove: () => ((id: string) => void | Promise<void>) | undefined,
  id: () => string,
) {
  const state = $state({ open: false, pending: false, error: "" });
  return {
    get open() {
      return state.open;
    },
    set open(value: boolean) {
      state.open = value;
    },
    get pending() {
      return state.pending;
    },
    get error() {
      return state.error;
    },
    async confirm() {
      const action = remove();
      if (!action || state.pending) return false;
      state.pending = true;
      state.error = "";
      try {
        await action(id());
        state.open = false;
        return true;
      } catch (cause) {
        state.error =
          cause instanceof Error
            ? cause.message
            : "The message could not be deleted";
        return false;
      } finally {
        state.pending = false;
      }
    },
  };
}
