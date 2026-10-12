import type { ServerConnections } from "../../connections/owner.svelte.js";
export type ServerSwitcherOwner = Pick<
  ServerConnections,
  "supported" | "selected" | "records" | "select" | "manage" | "add"
>;
export type ConnectionDialogOwner = Pick<
  ServerConnections,
  | "dialog"
  | "editing"
  | "busy"
  | "error"
  | "connect"
  | "rename"
  | "remove"
  | "signOut"
>;
