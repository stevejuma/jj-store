import {
  ApiError,
  decode,
  fullId,
  pretty,
  type Exact,
  type Model,
} from "../api/client.js";
import type { operations } from "../api/generated.js";
import { LocalClient } from "../api/local-client.js";
type Creation = Exact<
  operations["local_native_repository_create"]["requestBody"]["content"]["application/json"]
>;
const key = "jj-store.native-repository-creation.v1";
/** The browser retains only the exact public request; installation state owns recovery. */
export class NativeRepositoryManager {
  readonly client: LocalClient;
  pending: Creation | null = null;
  constructor(
    fetcher: typeof fetch | undefined,
    private storage: Pick<Storage, "getItem" | "setItem" | "removeItem">,
  ) {
    this.client = new LocalClient("", fetcher);
    this.refresh();
  }
  refresh() {
    this.pending = null;
    const saved = this.storage.getItem(key);
    if (saved) {
      const value = decode(saved) as Creation;
      if (
        value.version !== 1 ||
        typeof value.name !== "string" ||
        !value.name.trim()
      )
        throw new ApiError("invalid_creation_intent");
      fullId(value.request_id, 32);
      this.pending = value;
    }
  }
  close() {
    this.client.close();
  }
  private accept(value: Model<"NativeCreationOutcome">) {
    if (value.request_id !== this.pending?.request_id)
      throw new ApiError("creation_identity_mismatch");
    if (value.status === "committed") {
      fullId(value.repository, 32);
      if (value.name !== this.pending.name)
        throw new ApiError("creation_identity_mismatch");
      this.storage.removeItem(key);
      this.pending = null;
    }
    return value;
  }
  async create(name: string) {
    this.refresh();
    if (!this.pending) {
      this.pending = {
        version: 1,
        request_id: crypto.randomUUID().replaceAll("-", ""),
        name: name.trim(),
      };
      this.storage.setItem(key, pretty(this.pending));
    }
    try {
      return this.accept(
        await this.client.request<Model<"NativeCreationOutcome">>(
          "repositories/native",
          this.pending,
        ),
      );
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        ["invalid_local_folder", "local_registration_conflict"].includes(
          cause.code,
        )
      ) {
        this.storage.removeItem(key);
        this.pending = null;
      }
      throw cause;
    }
  }
  async status() {
    this.refresh();
    if (!this.pending) return null;
    return this.accept(
      await this.client.request<Model<"NativeCreationOutcome">>(
        `repositories/native/requests/${this.pending.request_id}`,
      ),
    );
  }
}
