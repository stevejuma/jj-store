<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    error,
    onRetry,
    onDismiss,
    onRetryError,
    onClearError,
  }: {
    error: string;
    onRetry?: () => void;
    onDismiss?: () => void;
    onRetryError(): void;
    onClearError(): void;
  } = $props();
</script>

<span class="community-composer-status-actions">
  {#if error || onRetry}<Button
      type="button"
      variant="ghost"
      size="sm"
      onclick={() => (error ? onRetryError() : onRetry?.())}>Retry</Button
    >{/if}
  {#if error || onDismiss}<Button
      type="button"
      variant="ghost"
      size="sm"
      onclick={() => {
        onClearError();
        onDismiss?.();
      }}>Dismiss</Button
    >{/if}
</span>
