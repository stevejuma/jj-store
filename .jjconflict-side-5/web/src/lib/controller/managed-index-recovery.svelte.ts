/** Observe exact read recovery under the owning controller's Svelte lifetime. */
export function watchCodeIndexRecovery(recover: () => void) {
  $effect(() => {
    recover();
  });
}
