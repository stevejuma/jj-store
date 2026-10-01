import { ApiError } from "../api/client.js";

export function storeErrorMessage(cause: unknown): string {
  if (cause instanceof ApiError)
    return (
      (
        {
          local_folder_unavailable:
            "The local folder is unavailable. Reconnect it in Service settings.",
          local_identity_changed:
            "The folder now contains a different repository. Reconnect the original repository in Service settings.",
          unsupported_local_capability:
            "This feature is not supported for local JJ repositories.",
        } as Record<string, string>
      )[cause.code] ?? cause.message
    );
  if (cause instanceof DOMException && cause.name === "AbortError") return "";
  return "The repository request could not be completed.";
}

export function connectionDeniedMessage(mode: string) {
  return mode === "local_trusted"
    ? "Trusted local access was rejected. Check the server and repository catalog."
    : "Access expired or was revoked. Connect with a current token.";
}
