import type { Contract } from "./generated.js";
import { HistoryProtocolError, decode, encode } from "./wire.js";
export {
  HistoryProtocolError,
  VERSION,
  MAX_REQUEST_BYTES,
  MAX_RESPONSE_BYTES,
} from "./wire.js";
export type * from "./generated.js";
export type Command = "heads" | "open" | "status" | "read" | "blame" | "close";
export interface HistoryTransport {
  invoke(command: Command, request: string): Promise<string>;
}
type Data<K extends keyof Contract> = Contract[K]["data"];
export type Handle = Data<"open_response">;
export type Query = Data<"read_request">["query"];
export type BlameQuery = Data<"blame_request">["query"];
export type BlamePage = Data<"blame_response">;
export class HistoryClient {
  readonly #vault: string;
  readonly #transport: HistoryTransport;
  constructor(vault: string, transport: HistoryTransport) {
    encode("heads_request", { vault });
    this.#vault = vault;
    this.#transport = transport;
  }
  async heads(): Promise<Data<"heads_response">> {
    return decode(
      "heads_response",
      await this.#transport.invoke(
        "heads",
        encode("heads_request", { vault: this.#vault }),
      ),
    );
  }
  async open(operation: Data<"open_request">["operation"]): Promise<Handle> {
    return decode(
      "open_response",
      await this.#transport.invoke(
        "open",
        encode("open_request", { vault: this.#vault, operation }),
      ),
    );
  }
  async status(handle: Handle): Promise<Data<"status_response">> {
    return decode(
      "status_response",
      await this.#transport.invoke(
        "status",
        encode("selection_request", { vault: this.#vault, handle }),
      ),
    );
  }
  async read(handle: Handle, query: Query): Promise<Data<"read_response">> {
    const kind = query.kind;
    const result = decode(
      "read_response",
      await this.#transport.invoke(
        "read",
        encode("read_request", { vault: this.#vault, handle, query }),
      ),
    );
    if (result.kind !== kind) throw new HistoryProtocolError();
    return result;
  }
  async close(handle: Handle): Promise<void> {
    decode(
      "close_response",
      await this.#transport.invoke(
        "close",
        encode("selection_request", { vault: this.#vault, handle }),
      ),
    );
  }
  async blame(handle: Handle, query: BlameQuery): Promise<BlamePage> {
    const result = decode(
      "blame_response",
      await this.#transport.invoke(
        "blame",
        encode("blame_request", { vault: this.#vault, handle, query }),
      ),
    );
    if (result.commit !== query.commit || result.path !== query.path)
      throw new HistoryProtocolError();
    return result;
  }
}
