import { ApiError, decode, fullId, pretty, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import type { Exact } from "../api/client.js";
import type { operations } from "../api/generated.js";
type Registration = Exact<
  operations["local_repository_register"]["requestBody"]["content"]["application/json"]
>;
const pendingKey = "jj-store.local-folder-registration.v1";
/** Persists the exact Add intent before dispatch; reload/status/retry never invent another identity. */
export class LocalFolderManager {
  readonly client: LocalClient;
  pending: Registration | null = null;
  bindings: Model<"LocalBindings"> | null = null;
  constructor(
    fetcher: typeof fetch | undefined,
    private storage: Pick<Storage, "getItem" | "setItem" | "removeItem">,
  ) {
    this.client = new LocalClient("", fetcher);
    const saved = storage.getItem(pendingKey);
    if (saved) {
      const value = decode(saved) as Registration;
      if (
        value.version !== 1 ||
        typeof value.path !== "string" ||
        !value.path.startsWith("/") ||
        typeof value.name !== "string" ||
        BigInt(value.expected_revision) < 0n
      )
        throw new ApiError("invalid_registration_intent");
      fullId(value.request_id, 32);
      this.pending = value;
    }
  }
  close() {
    this.client.close();
  }
  async list() {
    return (this.bindings =
      await this.client.request<Model<"LocalBindings">>("repositories"));
  }
  inspect(path: string) {
    return this.client.request<Model<"LocalFolder">>("repositories/inspect", {
      path,
    });
  }
  pick() {
    return this.client.request<Model<"FolderSelection">>(
      "repositories/pick",
      {},
    );
  }
  private accept(outcome: Model<"RegistrationOutcome">) {
    if (outcome.request_id !== this.pending?.request_id)
      throw new ApiError("registration_identity_mismatch");
    if (outcome.status === "committed") {
      this.storage.removeItem(pendingKey);
      this.pending = null;
    }
    return outcome;
  }
  async status() {
    if (!this.pending) return null;
    return this.accept(
      await this.client.request<Model<"RegistrationOutcome">>(
        `repositories/requests/${this.pending.request_id}`,
      ),
    );
  }
  async add(path: string, name: string) {
    if (!this.pending) {
      if (!this.bindings)
        throw new ApiError("registration_revision_unavailable");
      const request: Registration = {
        version: 1,
        request_id: crypto.randomUUID().replaceAll("-", ""),
        expected_revision: this.bindings.revision,
        path,
        name,
      };
      this.storage.setItem(pendingKey, pretty(request));
      this.pending = request;
    }
    return this.accept(
      await this.client.request<Model<"RegistrationOutcome">>(
        "repositories",
        this.pending,
      ),
    );
  }
  async reconnect(repository: string, path: string) {
    if (!this.bindings) throw new ApiError("registration_revision_unavailable");
    return (this.bindings = await this.client.request<Model<"LocalBindings">>(
      `repositories/${fullId(repository, 32)}/reconnect`,
      { version: 1, expected_revision: this.bindings.revision, path },
    ));
  }
  async remove(repository: string) {
    if (!this.bindings) throw new ApiError("registration_revision_unavailable");
    return (this.bindings = await this.client.request<Model<"LocalBindings">>(
      `repositories/${fullId(repository, 32)}/remove`,
      { version: 1, expected_revision: this.bindings.revision },
    ));
  }
}
