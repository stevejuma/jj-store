import {
  ApiError,
  decode,
  fullId,
  pretty,
  type Model,
  type Exact,
} from "../api/client.js";
import type { operations } from "../api/generated.js";
import { LocalClient } from "../api/local-client.js";
import { bootstrap } from "../api/bootstrap.js";
type Request = Exact<
  operations["local_native_repository_delete"]["requestBody"]["content"]["application/json"]
>;
const key = "jj-store.native-repository-deletion.v1";
export interface PendingDeletion {
  repository: string;
  name: string;
  request: Request;
}
/** Public intent only. Installation identity prevents retrying against another service. */
export class NativeRepositoryDeletion {
  readonly client: LocalClient;
  pending: PendingDeletion | null = null;
  constructor(
    private storage: Pick<Storage, "getItem" | "setItem" | "removeItem">,
    private fetcher?: typeof fetch,
    private selection?: { repository: string; name: string; epoch: string },
  ) {
    this.client = new LocalClient("", fetcher);
    const saved = storage.getItem(key);
    if (saved) {
      const pending = decode(saved) as PendingDeletion;
      fullId(pending.repository, 32);
      fullId(pending.request.request_id, 32);
      fullId(pending.request.expected_epoch, 32);
      if (
        pending.request.version !== 1 ||
        typeof pending.request.expected_installation !== "string" ||
        !pending.request.expected_installation ||
        typeof pending.name !== "string"
      )
        throw new ApiError("invalid_deletion_intent");
      if (selection && pending.repository !== selection.repository)
        throw new ApiError("repository_deletion_pending");
      this.pending = pending;
    }
  }
  close() {
    this.client.close();
  }
  private accept(value: Model<"DeletionOutcome">) {
    if (
      value.version !== 1 ||
      value.repository !== this.pending?.repository ||
      value.request_id !== this.pending.request.request_id ||
      !["pending", "deleted"].includes(value.state)
    )
      throw new ApiError("deletion_identity_mismatch");
    if (value.state === "deleted") {
      this.storage.removeItem(key);
      this.pending = null;
    }
    return value;
  }
  async status() {
    if (!this.pending) return null;
    return this.accept(
      await this.client.request<Model<"DeletionOutcome">>(
        `repositories/${this.pending.repository}/deletions/${this.pending.request.request_id}`,
      ),
    );
  }
  async remove() {
    if (!this.pending) {
      if (!this.selection) throw new ApiError("deletion_intent_unavailable");
      const service = await bootstrap(this.fetcher, this.client.closed.signal);
      if (
        !service.installation_id ||
        service.capabilities.local_native_repository_deletion !== true
      )
        throw new ApiError("repository_deletion_unavailable");
      fullId(this.selection.repository, 32);
      fullId(this.selection.epoch, 32);
      const pending: PendingDeletion = {
        repository: this.selection.repository,
        name: this.selection.name,
        request: {
          version: 1,
          request_id: crypto.randomUUID().replaceAll("-", ""),
          expected_epoch: this.selection.epoch,
          expected_installation: service.installation_id,
        },
      };
      this.storage.setItem(key, pretty(pending));
      this.pending = pending;
    }
    try {
      while (this.pending) {
        const pending = this.pending;
        this.accept(
          await this.client.request<Model<"DeletionOutcome">>(
            `repositories/${pending.repository}/delete`,
            pending.request,
          ),
        );
      }
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        [
          "repository_deletion_unsupported",
          "repository_deletion_snapshot_changed",
        ].includes(cause.code)
      ) {
        this.storage.removeItem(key);
        this.pending = null;
      }
      throw cause;
    }
  }
}
