import { getContext, setContext } from "svelte";
import type { ServerConnectionsView } from "./presentation.js";
const key = Symbol("server-connections");
export const provideServerConnections = (owner: ServerConnectionsView) =>
  setContext(key, owner);
export const serverConnections = () =>
  getContext<ServerConnectionsView | undefined>(key);
