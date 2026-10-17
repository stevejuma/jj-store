import { HistoryClient, type Command } from "./index.js";
export type Invoke = (
  command: string,
  args: { request: string },
) => Promise<string>;
/** Pass the host's public Tauri invoke function; this entry adds no host policy. */
export function tauriHistory(vault: string, invoke: Invoke): HistoryClient {
  return new HistoryClient(vault, {
    invoke: (command: Command, request: string) =>
      invoke(`plugin:jj-store|${command}`, { request }),
  });
}
