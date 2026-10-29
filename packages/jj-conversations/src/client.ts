/** Host-owned identities and lossless encoding; this client is not publication authority. */
export interface RequestIdentity {
  request_id: string;
}
export interface EncodedCommand {
  readonly request_id: string;
  readonly body: string;
}
export interface ConversationTransport<O extends RequestIdentity> {
  submit(command: EncodedCommand): Promise<O>;
  /** Read status only. Never dispatch, replace an identity or replay a prompt. */
  lookup(request_id: string): Promise<O | undefined>;
}
export type ClientState = "ready" | "submitting" | "receipt" | "unknown";
export type ClientErrorCode =
  | "invalid_request"
  | "invalid_command"
  | "identity_conflict"
  | "capacity"
  | "unknown_outcome"
  | "receipt_identity_mismatch"
  | "unresolved";
const messages: Record<ClientErrorCode, string> = {
  invalid_request: "Invalid conversation request identity.",
  invalid_command: "Invalid or oversized conversation command.",
  identity_conflict:
    "The request identity is already bound to another command.",
  capacity: "The conversation request ledger is full.",
  unknown_outcome:
    "The request outcome is unknown. Read its retained status before continuing.",
  receipt_identity_mismatch:
    "The conversation receipt has another request identity.",
  unresolved: "The conversation request is unresolved.",
};
export class ConversationClientError extends Error {
  constructor(
    readonly code: ClientErrorCode,
    readonly request_id?: string,
  ) {
    super(messages[code]);
    this.name = "ConversationClientError";
  }
}
export interface ClientSnapshot<O> {
  readonly request_id: string;
  /** Receipt means a response exists, not proof of publication, delivery or capture. */
  readonly state: ClientState;
  readonly receipt?: O;
}
interface Entry<O> {
  request_id: string;
  encoded?: EncodedCommand;
  state: ClientState;
  receipt?: O;
  pending?: Promise<O>;
}
export interface ClientOptions {
  maxEntries?: number;
  maxBodyBytes?: number;
}
/** One app/vault scope. Unknown requests are never evicted or automatically submitted. */
export class ConversationClient<
  C extends RequestIdentity,
  O extends RequestIdentity,
> {
  readonly #entries = new Map<string, Entry<O>>();
  readonly #maxEntries: number;
  readonly #maxBodyBytes: number;
  constructor(
    readonly scope: string,
    private readonly transport: ConversationTransport<O>,
    private readonly encode: (command: C) => string,
    options: ClientOptions = {},
  ) {
    this.#maxEntries = options.maxEntries ?? 256;
    this.#maxBodyBytes = options.maxBodyBytes ?? 4 * 1024 * 1024;
    if (
      !scope ||
      !Number.isInteger(this.#maxEntries) ||
      this.#maxEntries < 1 ||
      this.#maxEntries > 4096 ||
      !Number.isInteger(this.#maxBodyBytes) ||
      this.#maxBodyBytes < 1 ||
      this.#maxBodyBytes > 16 * 1024 * 1024
    ) {
      throw new ConversationClientError("invalid_command");
    }
  }
  prepare(command: C): EncodedCommand {
    return this.#entry(command).encoded!;
  }
  snapshot(request_id: string): ClientSnapshot<O> | undefined {
    const entry = this.#entries.get(request_id);
    return (
      entry && {
        request_id,
        state: entry.state,
        ...(entry.receipt === undefined
          ? {}
          : { receipt: structuredClone(entry.receipt) }),
      }
    );
  }
  /** Restore an interrupted host intent without making a transport call. */
  restoreUnknown(request_id: string): ClientSnapshot<O> {
    this.#id(request_id);
    let entry = this.#entries.get(request_id);
    if (!entry) {
      if (this.#entries.size >= this.#maxEntries)
        throw new ConversationClientError("capacity", request_id);
      entry = { request_id, state: "unknown" };
      this.#entries.set(request_id, entry);
    } else if (entry.state === "ready") entry.state = "unknown";
    return this.snapshot(request_id)!;
  }
  send(command: C): Promise<O> {
    const entry = this.#entry(command);
    if (entry.pending)
      return entry.pending.then((value) => structuredClone(value));
    if (entry.receipt !== undefined)
      return Promise.resolve(structuredClone(entry.receipt));
    if (entry.state === "unknown")
      return Promise.reject(
        new ConversationClientError("unknown_outcome", entry.request_id),
      );
    entry.state = "submitting";
    // Defer the call until pending is retained, including synchronously throwing transports.
    entry.pending = Promise.resolve()
      .then(() => this.transport.submit(entry.encoded!))
      .then((receipt) => this.#accept(entry, receipt))
      .catch((cause) => {
        entry.state = "unknown";
        const code =
          cause instanceof ConversationClientError &&
          cause.code === "receipt_identity_mismatch"
            ? cause.code
            : "unknown_outcome";
        throw new ConversationClientError(code, entry.request_id);
      })
      .finally(() => {
        delete entry.pending;
      });
    return entry.pending.then((value) => structuredClone(value));
  }
  /** A refresh may update receipts; it never calls submit. Missing status proves nothing. */
  async refresh(request_id: string): Promise<O | undefined> {
    const entry = this.#entries.get(request_id);
    if (!entry || entry.state === "ready")
      throw new ConversationClientError("unresolved", request_id);
    if (entry.pending) return structuredClone(await entry.pending);
    let receipt: O | undefined;
    try {
      receipt = await this.transport.lookup(request_id);
    } catch {
      throw new ConversationClientError("unknown_outcome", request_id);
    }
    if (receipt === undefined) return undefined;
    return structuredClone(this.#accept(entry, receipt));
  }
  /** The host decides whether its separate publication/delivery/capture receipts are final. */
  retire(request_id: string, isFinal: (receipt: O) => boolean): void {
    const entry = this.#entries.get(request_id);
    if (
      !entry ||
      entry.state !== "receipt" ||
      entry.receipt === undefined ||
      !isFinal(structuredClone(entry.receipt))
    ) {
      throw new ConversationClientError("unresolved", request_id);
    }
    this.#entries.delete(request_id);
  }
  async drain(): Promise<readonly ClientSnapshot<O>[]> {
    await Promise.allSettled(
      [...this.#entries.values()].flatMap((entry) =>
        entry.pending ? [entry.pending] : [],
      ),
    );
    return [...this.#entries.keys()].map((id) => this.snapshot(id)!);
  }
  #accept(entry: Entry<O>, receipt: O): O {
    if (receipt.request_id !== entry.request_id)
      throw new ConversationClientError(
        "receipt_identity_mismatch",
        entry.request_id,
      );
    const retained = structuredClone(receipt);
    entry.receipt = retained;
    entry.state = "receipt";
    return retained;
  }
  #id(id: string): void {
    if (typeof id !== "string" || !/^[A-Za-z0-9_-]{1,256}$/.test(id))
      throw new ConversationClientError("invalid_request");
  }
  #entry(command: C): Entry<O> {
    const id = command.request_id;
    this.#id(id);
    const bound = this.#entries.get(id);
    if (bound && !bound.encoded)
      throw new ConversationClientError("unresolved", id);
    let body: string;
    try {
      body = this.encode(command);
    } catch {
      throw new ConversationClientError("invalid_command", id);
    }
    if (
      typeof body !== "string" ||
      new TextEncoder().encode(body).byteLength > this.#maxBodyBytes
    )
      throw new ConversationClientError("invalid_command", id);
    const existing = this.#entries.get(id);
    if (existing) {
      if (existing.encoded?.body !== body)
        throw new ConversationClientError("identity_conflict", id);
      return existing;
    }
    if (this.#entries.size >= this.#maxEntries)
      throw new ConversationClientError("capacity", id);
    const entry: Entry<O> = {
      request_id: id,
      encoded: Object.freeze({ request_id: id, body }),
      state: "ready",
    };
    this.#entries.set(id, entry);
    return entry;
  }
}
