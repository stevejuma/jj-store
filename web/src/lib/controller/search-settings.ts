import { Client, pretty, type Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
import type { StoreLoadStatus } from "./contracts.js";
export type SearchConfiguration = Model<"SearchConfiguration">;
export type SearchAction = NonNullable<
  operations["control_search"]["requestBody"]
>["content"]["application/json"]["action"];
export interface SearchDraft {
  enabled: boolean;
  indexSource: boolean;
  bookmarks: string;
  workspaces: string;
  include: string;
  exclude: string;
  budget: string;
}
export interface SearchSettings {
  value: Model<"LocalSearchSettings"> | null;
  draft: SearchDraft | null;
  dirty: boolean;
  state: StoreLoadStatus;
  error: string;
  validation?: string;
  changing: boolean;
}
export const emptySearchSettings = (): SearchSettings => ({
  value: null,
  draft: null,
  dirty: false,
  state: "idle",
  error: "",
  changing: false,
});
export function searchDraft(config: SearchConfiguration): SearchDraft {
  const scopes = (kind: "bookmark" | "workspace") =>
    Object.entries(config.scopes)
      .filter(([, s]) => s.kind === kind)
      .map(([id, s]) => `${id}=${s.name}`)
      .join("\n");
  return {
    enabled: config.enabled,
    indexSource: config.index_source,
    bookmarks: scopes("bookmark"),
    workspaces: scopes("workspace"),
    include: config.include_prefixes.join("\n"),
    exclude: config.exclude_prefixes.join("\n"),
    budget: String(Number(config.index_bytes) / 1048576),
  };
}
export function searchConfiguration(draft: SearchDraft): SearchConfiguration {
  const scopes: SearchConfiguration["scopes"] = Object.create(null);
  const lines = (s: string) => s.split("\n").filter(Boolean);
  for (const [kind, text] of [
    ["bookmark", draft.bookmarks],
    ["workspace", draft.workspaces],
  ] as const) {
    for (const line of lines(text)) {
      const equals = line.indexOf("="),
        id = line.slice(0, equals),
        name = line.slice(equals + 1);
      if (
        equals < 1 ||
        !name ||
        !/^[-\w:.]{1,128}$/.test(id) ||
        Object.hasOwn(scopes, id)
      )
        throw new Error(
          "Give each scope a unique name and target: scope=target.",
        );
      scopes[id] = { kind, name };
    }
  }
  const budget = Number(draft.budget) * 1048576;
  if (!Number.isSafeInteger(budget) || budget < 1 || budget > 2147483648)
    throw new Error(
      "Index budget must be greater than zero and at most 2048 MiB.",
    );
  return {
    enabled: draft.enabled,
    index_source: draft.indexSource,
    scopes,
    include_prefixes: lines(draft.include),
    exclude_prefixes: lines(draft.exclude),
    index_bytes: budget,
  };
}
export interface SearchSettingsTransport {
  status(): Promise<Model<"LocalSearchSettings">>;
  save(
    revision: string,
    configuration: SearchConfiguration,
  ): Promise<Model<"LocalSearchSettings">>;
  control(
    revision: string,
    action: SearchAction,
  ): Promise<Model<"LocalSearchSettings">>;
}
export class HttpSearchSettings implements SearchSettingsTransport {
  constructor(private client: Client) {}
  status() {
    return this.client.request<Model<"LocalSearchSettings">>("local/search");
  }
  private post(path: string, body: unknown) {
    return this.client.request<Model<"LocalSearchSettings">>(
      path,
      {},
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: pretty(body),
      },
    );
  }
  save(expected_revision: string, configuration: SearchConfiguration) {
    return this.post("local/search/save", {
      version: 1,
      expected_revision,
      configuration,
    });
  }
  control(expected_revision: string, action: SearchAction) {
    return this.post("local/search/control", {
      version: 1,
      expected_revision,
      action,
    });
  }
}
