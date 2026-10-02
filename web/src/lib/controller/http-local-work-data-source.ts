import { ApiError, fullId, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
import type {
  WorkBrowseQuery,
  WorkBrowseTransport,
} from "./projects-hub-types.js";
import type { WorkQuery, WorkRequest } from "./work-types.js";
import type { StoreDataSource } from "./native-data-source.js";
import { readNativeActivityPage } from "./native-activity-reader.js";
import type { NativeActivityQuery } from "./native-activity-types.js";
import { readLocalRepositoryDirectory } from "./local-repository-directory.js";
export class HttpLocalWorkDataSource implements WorkBrowseTransport {
  constructor(
    private client: LocalClient,
    private pin: () => Promise<string>,
    private source: StoreDataSource,
    private read: LocalClient["request"],
  ) {}
  agents = async (cursor?: string, signal?: AbortSignal) => {
    signal?.throwIfAborted();
    const page = await this.source.agentSessions(cursor);
    signal?.throwIfAborted();
    return page;
  };
  refresh = async () => {
    await this.source.snapshot(true);
  };
  activity = async (query: NativeActivityQuery, signal?: AbortSignal) => {
    signal?.throwIfAborted();
    const value = await readNativeActivityPage(this.source, query);
    signal?.throwIfAborted();
    return value;
  };
  directory = (signal?: AbortSignal) =>
    readLocalRepositoryDirectory(this.source, this, signal);
  async page(query: WorkQuery, signal?: AbortSignal) {
    const pin = await this.pin();
    const page = await this.read<Model<"LocalWorkPage">>(
      "work/page",
      { metadata_commit: pin, query },
      signal,
    );
    this.validate(page.snapshot, pin);
    return page;
  }
  async browse(query: WorkBrowseQuery, signal?: AbortSignal) {
    const pin = await this.pin();
    const page = await this.read<Model<"LocalWorkBrowsePage">>(
      "work/browse",
      { metadata_commit: pin, query },
      signal,
    );
    this.validate(page.snapshot, pin);
    return page;
  }
  private validate(snapshot: Model<"LocalWorkSnapshot">, pin: string) {
    if (
      snapshot.profile !== "local_jj_git" ||
      snapshot.repo_id !== this.client.repository ||
      snapshot.metadata_commit !== pin
    )
      throw new ApiError("snapshot_mismatch");
  }
  apply(request: WorkRequest) {
    return this.client.request<Model<"LocalWorkOutcome">>(
      "work/commands",
      request,
    );
  }
  status(id: string) {
    return this.read<Model<"LocalWorkOutcome">>(
      `work/commands/${fullId(id, 32)}`,
    );
  }
}
