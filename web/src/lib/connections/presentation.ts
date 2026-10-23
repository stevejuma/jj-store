import type { ServerConnections } from "./owner.svelte.js";
/** Presentation input shared by the browser host and deterministic application stories. */
export type ServerConnectionsView = Pick<
  ServerConnections,
  | "controller"
  | "workspaces"
  | "supported"
  | "selected"
  | "records"
  | "dialog"
  | "editing"
  | "busy"
  | "error"
  | "select"
  | "manage"
  | "add"
  | "connect"
  | "rename"
  | "remove"
  | "signOut"
  | "guard"
  | "shareUrl"
>;
