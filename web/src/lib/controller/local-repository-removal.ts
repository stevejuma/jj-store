import { fullId, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";

/** A failed acknowledgement never rebases this removal onto a newer registration. */
export class LocalRepositoryRemoval {
  private readonly client: LocalClient;
  private revision: Model<"LocalBindings">["revision"] | undefined;
  constructor(
    readonly repository: string,
    fetcher?: typeof fetch,
  ) {
    fullId(repository, 32);
    this.client = new LocalClient("", fetcher);
  }
  close() {
    this.client.close();
  }
  async remove() {
    const observed =
      await this.client.request<Model<"LocalBindings">>("repositories");
    if (!observed.bindings.some((b) => b.repository === this.repository))
      return;
    this.revision ??= observed.revision;
    await this.client.request<Model<"LocalBindings">>(
      `repositories/${this.repository}/remove`,
      { version: 1, expected_revision: this.revision },
    );
  }
}
