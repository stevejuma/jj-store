import {
  HistoryClient,
  type HistoryTransport,
  type LocalGraphTimestamp,
} from "@lapismd/jj-history";
import { tauriHistory } from "@lapismd/jj-history/tauri";
const transport: HistoryTransport = { invoke: async () => "" };
const client = new HistoryClient("v", transport);
void client.read("handle", {
  kind: "graph",
  query: [
    {
      graph: "log",
      revset: "all()",
      user_email: "u",
      at_time_millis: -9223372036854775808n,
    },
    { limit: 1 },
  ],
});
const nativeTime: LocalGraphTimestamp = {
  millis: 9223372036854775807n,
  offset_minutes: 0,
};
void nativeTime;
// @ts-expect-error Native timestamps cannot be narrowed to JavaScript numbers.
const rounded: number = nativeTime.millis;
// @ts-expect-error Native graph requests require both query and pagination.
void client.read("h", { kind: "graph", query: [{ graph: "operations" }] });
// @ts-expect-error The native client has no code mutation command.
void transport.invoke("checkout", "{}");
// @ts-expect-error Native authority does not belong in a renderer frame.
void client.read("h", { kind: "query_info", root: "outside" });
const invoke = async <T>(
  _command: string,
  _args?: Record<string, unknown>,
): Promise<T> => {
  throw Error("fixture");
};
void tauriHistory("v", invoke);
