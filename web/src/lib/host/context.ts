import { getContext, setContext } from "svelte";
import { browserHost, type HostEnvironment } from "./contracts.js";
const hostKey = Symbol("jj-store-host");
export function provideHostEnvironment(host: HostEnvironment) {
  setContext(hostKey, host);
  return host;
}
export function useHostEnvironment(): HostEnvironment {
  return getContext<HostEnvironment | undefined>(hostKey) ?? browserHost;
}
