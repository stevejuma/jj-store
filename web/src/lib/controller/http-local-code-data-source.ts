import { HttpWorkingTree } from "./http-working-tree.js";
import { HttpLocalStacksTransport } from "./stack-workspace/local-transport.js";
import { ApiError, fullId } from "../api/client.js";
import { codeId } from "./repository-profile.js";
import type {
  GraphQuery,
  PathQuery,
  ReferenceQuery,
} from "./native-data-source.js";
import {
  HttpLocalQuery,
  type LocalCodeRequest as Request,
} from "./http-local-query.js";
export class HttpLocalCodeDataSource extends HttpLocalQuery {
  async codeSummary(operation: string, signal?: AbortSignal) {
    signal?.throwIfAborted();
    const summary = await this.query<"code_summary">(
      operation,
      { kind: "code_summary", query: { limit: 50 } },
      undefined,
      signal,
    );
    signal?.throwIfAborted();
    return summary;
  }
  readonly stacks = new HttpLocalStacksTransport(this.client);
  readonly workingTree = new HttpWorkingTree(this.client);
  references(operation: string, cursor?: string, input: ReferenceQuery = {}) {
    return this.query<"references">(
      operation,
      {
        kind: "references",
        query: {
          operation,
          query: input.query ?? "",
          exact: input.exact ?? null,
          kind: input.kind ?? "bookmarks",
          limit: 50,
          anchor: null,
          direction: "forward",
        },
      },
      cursor,
      input.signal,
    );
  }
  async queryInfo(operation: string) {
    return (await this.query<"query_info">(operation, { kind: "query_info" }))
      .data;
  }
  graph(kind: "log" | "operations" | "evolution", input: GraphQuery) {
    const graph: Extract<Request, { kind: "graph" }>["query"][0] =
      kind === "log"
        ? {
            graph: "log",
            revset: input.revset ?? "all()",
            workspace: input.workspace ?? null,
            user_email: input.userEmail ?? "",
            at_time_millis: input.atTimeMillis ?? Date.now(),
          }
        : kind === "operations"
          ? input.operationQuery
            ? { graph: "operations_filtered", query: input.operationQuery }
            : { graph: "operations" }
          : {
              graph: "evolution",
              commit: codeId(input.commit ?? "", this.profile),
            };
    return this.query<"graph">(
      input.operation,
      {
        kind: "graph",
        query: [graph, { limit: input.limit ?? 50, after: null }],
      },
      input.cursor,
      input.signal,
    );
  }
  async stackHistory(
    input: import("./native-data-source.js").StackHistoryQuery,
  ) {
    const limit = input.limit ?? 40;
    if (!Number.isInteger(limit) || limit < 1 || limit > 500)
      throw new ApiError("invalid_stack_page");
    const head = codeId(input.head, this.profile);
    const base =
      input.base === undefined ? null : codeId(input.base, this.profile);
    const result = await this.query<"stack_history">(
      input.operation,
      { kind: "stack_history", query: { head, base, limit, after: null } },
      input.cursor,
      input.signal,
    );
    const page = result.data;
    if (
      page.version !== 1 ||
      page.operation !== input.operation ||
      page.head !== head ||
      page.base !== base
    )
      throw new ApiError("snapshot_mismatch");
    if (
      !Array.isArray(page.commits) ||
      page.commits.length > limit ||
      Boolean(page.next) !== Boolean(result.next_cursor) ||
      (result.next_cursor !== null && result.next_cursor === input.cursor)
    )
      throw new ApiError("invalid_stack_page");
    return result;
  }
  commit(id: string, operation: string) {
    return this.query<"commit">(operation, {
      kind: "commit",
      query: { commit: codeId(id, this.profile) },
    });
  }
  commitSummary(id: string, operation: string, signal?: AbortSignal) {
    return this.query<"commit_summary">(
      operation,
      { kind: "commit_summary", query: { commit: codeId(id, this.profile) } },
      undefined,
      signal,
    );
  }
  paths(commit: string, operation: string, input: PathQuery = {}) {
    return this.query<"paths">(
      operation,
      {
        kind: "paths",
        query: {
          commit: codeId(commit, this.profile),
          path: input.path ?? "",
          recursive: input.recursive ?? false,
          include_last_change: input.includeLastChange ?? true,
          ...(input.lastChangeMode
            ? { last_change_mode: input.lastChangeMode }
            : {}),
          limit: input.limit ?? 50,
          anchor: input.anchor ?? null,
          direction: "forward",
        },
      },
      input.cursor,
      input.signal,
    );
  }
  async file(
    id: string,
    operation: string,
    context?: { commit: string; path: string; signal?: AbortSignal },
  ) {
    const result = context
      ? await this.query<"file_at">(
          operation,
          {
            kind: "file_at",
            query: {
              commit: codeId(context.commit, this.profile),
              path: context.path,
              file: codeId(id, this.profile),
            },
          },
          undefined,
          context.signal,
        )
      : await this.query<"file">(operation, {
          kind: "file",
          query: { file: codeId(id, this.profile) },
        });
    return {
      ...result.data,
      text: result.data.text ?? undefined,
      bytes: Number(result.data.bytes),
    };
  }
  operation(id: string) {
    return this.query<"operation">(id, {
      kind: "operation",
      query: { operation: fullId(id) },
    });
  }
  operationDiff(id: string, parent?: string) {
    return this.query<"operation_diff">(id, {
      kind: "operation_diff",
      query: { operation: fullId(id), parent: parent ? fullId(parent) : null },
    });
  }
  operationChanges(id: string, parent: string, cursor?: string) {
    return this.query<"operation_changes">(
      id,
      {
        kind: "operation_changes",
        query: {
          operation: fullId(id),
          parent: fullId(parent),
          limit: 50,
          anchor: null,
          direction: "forward",
        },
      },
      cursor,
    );
  }
  diffPaths(
    operation: string,
    from: string,
    to: string,
    cursor?: string,
    signal?: AbortSignal,
  ) {
    return this.query<"diff_paths">(
      operation,
      {
        kind: "diff_paths",
        query: {
          from: codeId(from, this.profile),
          to: codeId(to, this.profile),
          path: "",
          limit: 50,
          anchor: null,
          direction: "forward",
        },
      },
      cursor,
      signal,
    );
  }
  diff(
    operation: string,
    from: string,
    to: string,
    path: string,
    signal?: AbortSignal,
  ) {
    return this.query<"diff">(
      operation,
      {
        kind: "diff",
        query: {
          from: codeId(from, this.profile),
          to: codeId(to, this.profile),
          path,
          content: true,
        },
      },
      undefined,
      signal,
    );
  }
  conflicts(operation: string, commit: string) {
    return this.query<"conflicts">(operation, {
      kind: "conflicts",
      query: { commit: codeId(commit, this.profile), path: "" },
    });
  }
  protected async metadata() {
    const head = await this.snapshot();
    if (!head.metadata_commit) throw new ApiError("local_metadata_unavailable");
    return head.metadata_commit;
  }
}
