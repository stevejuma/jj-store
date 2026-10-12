import { getContext, setContext } from "svelte";
import type { ServerConnections } from "./owner.svelte.js";
const key = Symbol("server-connections");
export const provideServerConnections = (owner: ServerConnections) =>
  setContext(key, owner);
export const serverConnections = () =>
  getContext<ServerConnections | undefined>(key);
