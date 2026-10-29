import { ApiError } from "../api/client.js";

export function storeErrorMessage(cause: unknown): string {
  if (cause instanceof ApiError && cause.diagnostic)
    return cause.diagnostic.message;
  if (cause instanceof ApiError)
    return (
      (
        {
          repository_deletion_restart_required:
            "A storage write has an unknown outcome. Restart this local service, then continue the original deletion request.",
          repository_deletion_unavailable:
            "Deletion could not finish. Retry the retained original request.",
          repository_deletion_unsupported:
            "This storage bucket keeps object versions. Permanent deletion is unavailable for this bucket.",
          repository_deletion_snapshot_changed:
            "The repository changed before deletion began. Review it again before deleting.",
          repository_deletion_conflict:
            "This deletion does not match the request retained by this installation. Continue with the original request.",
          repository_deletion_pending:
            "Finish the retained repository deletion before deleting another repository.",
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
