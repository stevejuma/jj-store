/** Shared contextual discovery feedback; account credentials remain application-owned. */
export function configurationStatus(
  available: boolean,
  loading: boolean,
  error: string,
  status: string | null | undefined,
  hasResult: boolean,
  subscription: boolean,
) {
  if (!available)
    return "This harness is unavailable. Open Settings → Agents → Runtimes.";
  if (loading) return "Loading models from this configuration…";
  if (error) return error;
  if (status === "authentication_required")
    return subscription
      ? "Connect the selected ChatGPT account in Settings → Agents → Defaults, then retry."
      : "Authentication is required. Check the provider credentials in Advanced, then retry.";
  if (status === "unavailable")
    return subscription
      ? "The ChatGPT account is unavailable. Check the connection in Settings → Agents → Defaults, then retry."
      : "This provider is unavailable. Check its endpoint or Compute settings, then retry.";
  if (status === "empty")
    return "The provider returned no models. Your saved selection is retained.";
  return hasResult
    ? "Models advertised by this configuration. Saved changes apply on the next start."
    : "Choose a provider and configure credentials to discover models.";
}

export function configurationSource(
  local: boolean,
  choices: boolean,
  remembered: boolean,
  connected: boolean,
) {
  if (local) return "";
  if (!choices) return "No model or thinking choices have been advertised.";
  if (remembered) return "Showing choices from this harness's last session.";
  return connected ? "Showing choices from this harness's live session." : "";
}
