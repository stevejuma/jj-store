import { ApiError } from "../api/client.js";

export function storeErrorMessage(cause: unknown): string {
  if (cause instanceof ApiError) return cause.message;
  if (cause instanceof DOMException && cause.name === "AbortError") return "";
  return "The repository request could not be completed.";
}

export function connectionDeniedMessage(mode: string) {
  return mode === "local_trusted"
    ? "Trusted local access was rejected. Check the server and repository catalog."
    : "Access expired or was revoked. Connect with a current token.";
}
