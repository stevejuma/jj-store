export interface paths {
  "/v1/repos/{repo}/capabilities": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["capabilities"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["capabilities_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/snapshot": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["snapshot"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["snapshot_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/events": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["events"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["events_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/objects/{kind}/{id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["object"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["object_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/log": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["log"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["log_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/operations": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["operations"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["operations_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/operations/{operation}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["operation"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["operation_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/operations/{operation}/diff": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["operation_diff"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["operation_diff_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/evolution": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["evolution"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["evolution_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/commits/{commit}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["commit"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["commit_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/commits/{commit}/tree": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["tree"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["tree_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/commits/{commit}/conflicts": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["conflicts"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["conflicts_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/files/{file}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["file"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["file_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/diff": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["diff"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["diff_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/index/tasks": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    post: operations["prepare"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/index/tasks/{task}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["task"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["task_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/index/tasks/{task}/cancel": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    post: operations["cancel"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/mutations/preview": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred. */
    post: operations["mutation_preview"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/mutations": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred. */
    post: operations["mutation_apply"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/mutations/{request_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred. */
    get: operations["mutation_status"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["mutation_status_head"];
    patch?: never;
    trace?: never;
  };
}
export type webhooks = Record<string, never>;
export interface components {
  schemas: {
    RepositoryId: string;
    ErrorBody: {
      /** Format: uint32 */
      version: number;
      code: string;
      snapshot?: components["schemas"]["ReadSnapshot"] | null;
      heads?: components["schemas"]["HeadSnapshot"] | null;
    };
    ReadSnapshot: {
      /** Format: uint32 */
      version: number;
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      manifest_revision: number;
      heads: components["schemas"]["OperationId"][];
      operation_id: components["schemas"]["OperationId"];
    };
    EpochId: string;
    /**
     * Format: uint64
     * @description Publication order is distinct from native operation ancestry.
     */
    WalPosition: number;
    OperationId: string;
    HeadSnapshot: {
      /** Format: uint32 */
      version: number;
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      manifest_revision: number;
      heads: components["schemas"]["OperationId"][];
    };
    Capabilities: {
      /** Format: uint32 */
      version: number;
      native_profile: string;
      authentication: string;
      read_api: boolean;
      index_tasks: boolean;
      events: boolean;
      search: boolean;
      typed_mutations: string[];
      /** Format: uint */
      mutation_request_bytes: number;
      event_limits: components["schemas"]["EventLimits"];
      limits: components["schemas"]["ReadLimits"];
      log_context: components["schemas"]["LogContext"];
    };
    EventLimits: {
      /** Format: uint */
      streams: number;
      /** Format: uint64 */
      replay_window: number;
      /** Format: uint */
      page: number;
      /** Format: uint128 */
      poll_millis: number;
    };
    ReadLimits: {
      /** Format: uint */
      requests: number;
      /** Format: uint */
      workers: number;
      /** Format: uint128 */
      deadline_millis: number;
      /** Format: uint */
      json_bytes: number;
      /** Format: uint */
      page: number;
      /** Format: uint */
      index_commits: number;
      /** Format: uint */
      inline_index_commits: number;
      /** Format: uint64 */
      file_bytes: number;
    };
    LogContext: {
      at_time_millis: string;
      workspace: string;
      user_email: string;
    };
    CommittedEvent: {
      /** Format: uint32 */
      version: number;
      cursor: components["schemas"]["EventCursor"];
    } & (
      | {
          snapshot: components["schemas"]["HeadSnapshot"];
          /** @constant */
          type: "ready";
        }
      | {
          /** @constant */
          type: "changed";
        }
      | {
          reason: components["schemas"]["EventResetReason"];
          snapshot: components["schemas"]["HeadSnapshot"];
          /** @constant */
          type: "reset";
        }
    );
    EventCursor: {
      repository: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      position: components["schemas"]["WalPosition"];
    };
    /** @enum {string} */
    EventResetReason: "epoch_changed" | "future_position" | "replay_window";
    CommitId: string;
    Envelope_ContentObject: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["ContentObject"];
      next_cursor: string | null;
    };
    ContentObject: {
      /** Format: uint32 */
      version: number;
      profile: string;
      key: components["schemas"]["ObjectKey"];
      virtual_object: boolean;
      data: components["schemas"]["ContentObjectData"];
      stats: components["schemas"]["GraphStats"];
    };
    ObjectKey:
      | {
          /** @constant */
          kind: "commit";
          id: components["schemas"]["CommitId"];
        }
      | {
          /** @constant */
          kind: "tree";
          id: components["schemas"]["TreeId"];
        }
      | {
          /** @constant */
          kind: "file";
          id: components["schemas"]["FileId"];
        }
      | {
          /** @constant */
          kind: "symlink";
          id: components["schemas"]["SymlinkId"];
        }
      | {
          /** @constant */
          kind: "copy";
          id: components["schemas"]["CopyId"];
        }
      | {
          /** @constant */
          kind: "view";
          id: components["schemas"]["ViewId"];
        }
      | {
          /** @constant */
          kind: "operation";
          id: components["schemas"]["OperationId"];
        };
    TreeId: string;
    FileId: string;
    SymlinkId: string;
    CopyId: string;
    ViewId: string;
    ContentObjectData:
      | {
          /** @constant */
          kind: "commit";
          data: components["schemas"]["NativeCommit"];
        }
      | {
          /** @constant */
          kind: "tree";
          data: components["schemas"]["NativeTree"];
        }
      | {
          /** @constant */
          kind: "copy";
          data: components["schemas"]["NativeCopyHistory"];
        }
      | {
          /** @constant */
          kind: "view";
          data: components["schemas"]["NativeView"];
        }
      | {
          /** @constant */
          kind: "operation";
          data: components["schemas"]["NativeOperation"];
        }
      | {
          /** @constant */
          kind: "symlink";
          data: {
            target: string;
          };
        }
      | {
          /** @constant */
          kind: "file";
          data: {
            /** Format: uint64 */
            bytes: number;
          };
        };
    NativeCommit: {
      parents: components["schemas"]["CommitId"][];
      predecessors: components["schemas"]["CommitId"][];
      root_tree: components["schemas"]["TreeId"][];
      conflict_labels: string[];
      change_id: components["schemas"]["ChangeId"];
      description: string;
      author: components["schemas"]["NativeSignature"];
      committer: components["schemas"]["NativeSignature"];
      secure_sig: components["schemas"]["NativeSecureSig"] | null;
    };
    ChangeId: string;
    NativeSignature: {
      name: string;
      email: string;
      timestamp: components["schemas"]["NativeTimestamp"];
    };
    NativeTimestamp: {
      /** Format: int64 */
      millis: number;
      /** Format: int32 */
      offset_minutes: number;
    };
    NativeSecureSig: {
      data: number[];
      sig: number[];
    };
    NativeTree: {
      entries: [string, components["schemas"]["NativeTreeValue"]][];
    };
    NativeTreeValue:
      | {
          id: components["schemas"]["FileId"];
          executable: boolean;
          copy: components["schemas"]["CopyReference"];
          /** @constant */
          type: "file";
        }
      | {
          id: components["schemas"]["SymlinkId"];
          /** @constant */
          type: "symlink";
        }
      | {
          id: components["schemas"]["TreeId"];
          /** @constant */
          type: "tree";
        }
      | {
          id: number[];
          /** @constant */
          type: "submodule";
        };
    /** @description A copy placeholder is not a content-addressed object or dependency. */
    CopyReference:
      | {
          /** @constant */
          type: "placeholder";
        }
      | {
          /** @constant */
          type: "object";
          id: components["schemas"]["CopyId"];
        };
    NativeCopyHistory: {
      current_path: string;
      parents: components["schemas"]["CopyReference"][];
      salt: number[];
    };
    NativeView: {
      head_ids: components["schemas"]["CommitId"][];
      local_bookmarks: {
        [key: string]: (components["schemas"]["CommitId"] | null)[];
      };
      local_tags: {
        [key: string]: (components["schemas"]["CommitId"] | null)[];
      };
      remote_views: {
        [key: string]: components["schemas"]["NativeRemoteView"];
      };
      git_refs: {
        [key: string]: (components["schemas"]["CommitId"] | null)[];
      };
      git_heads: {
        [key: string]: (components["schemas"]["CommitId"] | null)[];
      };
      wc_commit_ids: {
        [key: string]: components["schemas"]["CommitId"];
      };
    };
    NativeRemoteView: {
      bookmarks: {
        [key: string]: components["schemas"]["NativeRemoteRef"];
      };
      tags: {
        [key: string]: components["schemas"]["NativeRemoteRef"];
      };
    };
    NativeRemoteRef: {
      target: (components["schemas"]["CommitId"] | null)[];
      state: components["schemas"]["NativeRemoteState"];
    };
    /** @enum {string} */
    NativeRemoteState: "new" | "tracked";
    NativeOperation: {
      view_id: components["schemas"]["ViewId"];
      parents: components["schemas"]["OperationId"][];
      metadata: components["schemas"]["NativeMetadata"];
      commit_predecessors: {
        [key: string]: components["schemas"]["CommitId"][];
      } | null;
    };
    NativeMetadata: {
      start: components["schemas"]["NativeTimestamp"];
      end: components["schemas"]["NativeTimestamp"];
      description: string;
      hostname: string;
      username: string;
      is_snapshot: boolean;
      workspace_name: string | null;
      attributes: {
        [key: string]: string;
      };
    };
    GraphStats: {
      /** Format: uint64 */
      metadata_nodes: number;
      /** Format: uint64 */
      predicate_file_bytes: number;
      /** Format: uint64 */
      predicate_lines: number;
      /** Format: uint64 */
      object_reads: number;
      /** Format: uint64 */
      object_bytes: number;
      /** Format: uint64 */
      work: number;
    };
    Envelope_GraphPage: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["GraphPage"];
      next_cursor: string | null;
    };
    GraphPage: {
      /** Format: uint32 */
      version: number;
      cache: components["schemas"]["CacheReport"];
      operation: components["schemas"]["OperationId"];
      data: components["schemas"]["GraphNodes"];
      next: components["schemas"]["ObjectKey"] | null;
      stats: components["schemas"]["GraphStats"];
    };
    CacheReport: {
      state: components["schemas"]["CacheState"];
      /** Format: uint */
      evicted: number;
      /** Format: uint */
      retained_entries: number;
      /** Format: uint64 */
      retained_bytes: number;
    };
    /** @enum {string} */
    CacheState: "built" | "reused" | "rebuilt";
    GraphNodes:
      | {
          /** @constant */
          graph: "log";
          nodes: components["schemas"]["GraphCommitNode"][];
        }
      | {
          /** @constant */
          graph: "operations";
          nodes: components["schemas"]["GraphOperationNode"][];
        }
      | {
          /** @constant */
          graph: "evolution";
          nodes: components["schemas"]["GraphEvolutionNode"][];
        };
    GraphCommitNode: {
      commit: components["schemas"]["GraphCommit"];
      edges: components["schemas"]["GraphEdge"][];
    };
    GraphCommit: {
      id: components["schemas"]["CommitId"];
      change_id: components["schemas"]["ChangeId"];
      parents: components["schemas"]["CommitId"][];
      description: string;
      author: components["schemas"]["GraphSignature"];
      committer: components["schemas"]["GraphSignature"];
      tree_terms: components["schemas"]["TreeId"][];
      conflicted: boolean;
      bookmarks: components["schemas"]["GraphBookmark"][];
      workspaces: string[];
      visible: boolean;
      divergent: boolean;
    };
    GraphSignature: {
      name: string;
      email: string;
      timestamp: components["schemas"]["GraphTimestamp"];
    };
    GraphTimestamp: {
      /** Format: int64 */
      millis: number;
      /** Format: int32 */
      offset_minutes: number;
    };
    GraphBookmark: {
      name: string;
      added: boolean;
      removed: boolean;
      conflicted: boolean;
    };
    GraphEdge: {
      target: components["schemas"]["CommitId"];
      kind: components["schemas"]["GraphEdgeKind"];
    };
    /** @enum {string} */
    GraphEdgeKind: "direct" | "indirect" | "missing";
    GraphOperationNode: {
      id: components["schemas"]["OperationId"];
      parents: components["schemas"]["OperationId"][];
      view: components["schemas"]["ViewId"];
      description: string;
      user: string;
      host: string;
      start: components["schemas"]["GraphTimestamp"];
      end: components["schemas"]["GraphTimestamp"];
      is_snapshot: boolean;
      workspace: string | null;
      attributes: {
        [key: string]: string;
      };
      predecessor_data_present: boolean;
    };
    GraphEvolutionNode: {
      commit: components["schemas"]["GraphCommit"];
      operation: components["schemas"]["OperationId"] | null;
      predecessors: components["schemas"]["CommitId"][];
      predecessors_known: boolean;
      legacy_predecessors: components["schemas"]["CommitId"][];
    };
    Envelope_OperationDetail: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["OperationDetail"];
      next_cursor: string | null;
    };
    OperationDetail: {
      operation: components["schemas"]["ContentObject"];
      view: components["schemas"]["ContentObject"];
    };
    Envelope_OperationDiffResponse: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["OperationDiffResponse"];
      next_cursor: string | null;
    };
    OperationDiffResponse: {
      /** Format: uint32 */
      version: number;
      data: components["schemas"]["OperationDiffData"];
      stats: components["schemas"]["GraphStats"];
    };
    OperationDiffData:
      | {
          operation: components["schemas"]["OperationId"];
          view: components["schemas"]["ViewId"];
          /** @constant */
          status: "root";
        }
      | {
          operation: components["schemas"]["OperationId"];
          parents: components["schemas"]["OperationId"][];
          /** @constant */
          status: "select_parent";
        }
      | ({
          /** @constant */
          status: "difference";
        } & components["schemas"]["OperationDiffDifference"]);
    OperationDiffViewDelta: {
      heads: components["schemas"]["SetDelta_CommitId"];
      local_bookmarks: components["schemas"]["NamedChange_Array_of_Nullable_CommitId"][];
      local_tags: components["schemas"]["NamedChange_Array_of_Nullable_CommitId"][];
      remote_views: components["schemas"]["NamedChange_NativeRemoteView"][];
      git_refs: components["schemas"]["NamedChange_Array_of_Nullable_CommitId"][];
      git_heads: components["schemas"]["NamedChange_Array_of_Nullable_CommitId"][];
      workspaces: components["schemas"]["NamedChange_CommitId"][];
    };
    SetDelta_CommitId: {
      added: components["schemas"]["CommitId"][];
      removed: components["schemas"]["CommitId"][];
    };
    NamedChange_Array_of_Nullable_CommitId: {
      name: string;
      before: (components["schemas"]["CommitId"] | null)[] | null;
      after: (components["schemas"]["CommitId"] | null)[] | null;
    };
    NamedChange_NativeRemoteView: {
      name: string;
      before: components["schemas"]["NativeRemoteView"] | null;
      after: components["schemas"]["NativeRemoteView"] | null;
    };
    NamedChange_CommitId: {
      name: string;
      before: components["schemas"]["CommitId"] | null;
      after: components["schemas"]["CommitId"] | null;
    };
    OperationDiffDifference: {
      operation: components["schemas"]["OperationId"];
      parent: components["schemas"]["OperationId"];
      before_view: components["schemas"]["ViewId"];
      after_view: components["schemas"]["ViewId"];
      view: components["schemas"]["OperationDiffViewDelta"];
      visible_commits: components["schemas"]["SetDelta_CommitId"];
      commit_predecessors: {
        [key: string]: components["schemas"]["CommitId"][];
      } | null;
    };
    Envelope_CommitDetail: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["CommitDetail"];
      next_cursor: string | null;
    };
    CommitDetail: {
      object: components["schemas"]["ContentObject"];
      commit: components["schemas"]["GraphCommit"];
    };
    Envelope_TreePage: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["TreePage"];
      next_cursor: string | null;
    };
    TreePage: {
      /** Format: uint32 */
      version: number;
      commit: components["schemas"]["CommitId"];
      path: string;
      entries: components["schemas"]["TreeEntry"][];
      next: string | null;
      stats: components["schemas"]["GraphStats"];
    };
    TreeEntry: {
      name: string;
      /**
       * @description Native alternating add/remove terms. A sole None means absence and is
       *     excluded from directory listings; unresolved deletions remain explicit.
       */
      terms: (components["schemas"]["NativeTreeValue"] | null)[];
      conflicted: boolean;
    };
    Envelope_ConflictResponse: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["ConflictResponse"];
      next_cursor: string | null;
    };
    ConflictResponse: {
      /** Format: uint32 */
      version: number;
      commit: components["schemas"]["CommitId"];
      change: components["schemas"]["ChangeId"];
      conflicts: components["schemas"]["ConflictDescriptor"][];
      stats: components["schemas"]["GraphStats"];
    };
    ConflictDescriptor: {
      path: string;
      fingerprint: components["schemas"]["ConflictFingerprint"];
      terms: components["schemas"]["ConflictTerm"][];
      editor: components["schemas"]["ConflictEditor"];
    };
    ConflictFingerprint: string;
    ConflictTerm: {
      /** Format: uint32 */
      id: number;
      role: components["schemas"]["ConflictRole"];
      label: string | null;
      value: components["schemas"]["NativeTreeValue"] | null;
    };
    /** @enum {string} */
    ConflictRole: "add" | "remove";
    ConflictEditor:
      | {
          contents: string[];
          executable: boolean;
          /** @constant */
          status: "text";
        }
      | {
          reason: components["schemas"]["ConflictUnsupported"];
          /** @constant */
          status: "unsupported";
        };
    /** @enum {string} */
    ConflictUnsupported:
      | "multiple_sides"
      | "delete_modify"
      | "structural"
      | "executable_mode"
      | "copy_history"
      | "binary"
      | "too_large"
      | "invalid_path";
    Envelope_FileDiffResponse: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["FileDiffResponse"];
      next_cursor: string | null;
    };
    FileDiffResponse: {
      /** Format: uint32 */
      version: number;
      from: components["schemas"]["CommitId"];
      to: components["schemas"]["CommitId"];
      path: string;
      changes: components["schemas"]["FileDiffChange"][];
      stats: components["schemas"]["GraphStats"];
    };
    FileDiffChange: {
      path: string;
      /** @description Native values at the target; source annotations never replace these. */
      before: (components["schemas"]["NativeTreeValue"] | null)[];
      after: (components["schemas"]["NativeTreeValue"] | null)[];
      sources: components["schemas"]["FileDiffSource"][];
      content: components["schemas"]["FileDiffContent"];
    };
    FileDiffSource: {
      kind: components["schemas"]["FileDiffSourceKind"];
      path: string;
      terms: (components["schemas"]["NativeTreeValue"] | null)[];
    };
    /** @enum {string} */
    FileDiffSourceKind: "normal" | "copy" | "rename";
    FileDiffContent:
      | {
          hunks: components["schemas"]["FileDiffHunk"][];
          /** @constant */
          status: "text";
        }
      | {
          reason: components["schemas"]["FileDiffUnavailable"];
          /** @constant */
          status: "unavailable";
        };
    FileDiffHunk: {
      kind: components["schemas"]["FileDiffHunkKind"];
      /** Format: uint */
      before_start: number;
      /** Format: uint */
      after_start: number;
      before: string;
      after: string;
    };
    /** @enum {string} */
    FileDiffHunkKind: "matching" | "different";
    /** @enum {string} */
    FileDiffUnavailable:
      | "not_requested"
      | "conflict"
      | "structural"
      | "multiple_sources"
      | "binary"
      | "too_large"
      | "complexity";
    IndexTask: {
      /** Format: uint32 */
      version: number;
      id: string;
      snapshot: components["schemas"]["ReadSnapshot"];
    } & (
      | {
          /** @constant */
          status: "running";
        }
      | {
          /** @constant */
          status: "canceling";
        }
      | {
          index: components["schemas"]["IndexReport"];
          /** @constant */
          status: "ready";
        }
      | {
          error: components["schemas"]["GraphError"];
          /** @constant */
          status: "failed";
        }
      | {
          /** @constant */
          status: "canceled";
        }
    );
    IndexReport: {
      /** Format: uint32 */
      version: number;
      operation: components["schemas"]["OperationId"];
      cache: components["schemas"]["CacheReport"];
      stats: components["schemas"]["GraphStats"];
    };
    /** @enum {string} */
    GraphError:
      | "canceled"
      | "denied"
      | "not_committed"
      | "epoch_changed"
      | "limit"
      | "busy"
      | "index_budget"
      | "index_rebuild_required"
      | "invalid"
      | "unavailable"
      | "corrupt"
      | "incomplete"
      | "not_found"
      | "not_directory"
      | "range";
    Preview:
      | {
          preview: components["schemas"]["PreviewReport"];
          /** @constant */
          status: "prepared";
        }
      | {
          receipt: components["schemas"]["Receipt"];
          /** @constant */
          status: "committed";
        };
    PreviewReport: {
      /** Format: uint32 */
      version: number;
      request_id: components["schemas"]["RequestId"];
      request_digest: components["schemas"]["IntentDigest"];
      expected: components["schemas"]["Snapshot"];
      /** @description Provisional native IDs; apply revalidates and may produce different IDs. */
      result: components["schemas"]["Prepared"];
    };
    RequestId: string;
    IntentDigest: string;
    Snapshot: {
      epoch: components["schemas"]["EpochId"];
      position: components["schemas"]["WalPosition"];
      heads: components["schemas"]["OperationId"][];
    };
    Prepared: {
      /** Format: uint32 */
      version: number;
      operation: components["schemas"]["OperationId"];
      created: components["schemas"]["CommitId"][];
      rewrites: components["schemas"]["Rewrite"][];
      bookmarks: {
        [key: string]: components["schemas"]["Change"];
      };
      workspaces: {
        [key: string]: components["schemas"]["Change2"];
      };
      stats: components["schemas"]["GraphStats"];
      /** Format: uint */
      staged_objects: number;
      /** Format: uint64 */
      staged_bytes: number;
      abandoned?: components["schemas"]["CommitId"][];
      skipped_hidden?: components["schemas"]["CommitId"][];
    };
    Rewrite: {
      commit: components["schemas"]["CommitId"];
      change: components["schemas"]["ChangeId"];
      predecessors: components["schemas"]["CommitId"][];
    };
    Change: {
      before: (components["schemas"]["CommitId"] | null)[];
      after: (components["schemas"]["CommitId"] | null)[];
    };
    Change2: {
      before: components["schemas"]["CommitId"] | null;
      after: components["schemas"]["CommitId"] | null;
    };
    Receipt: {
      /** Format: uint32 */
      version: number;
      request_id: components["schemas"]["RequestId"];
      publication_id: components["schemas"]["PublicationId"];
      request_digest: components["schemas"]["IntentDigest"];
      accepted: components["schemas"]["Snapshot"];
      result: components["schemas"]["Prepared"];
    };
    PublicationId: string;
    Outcome:
      | {
          receipt: components["schemas"]["Receipt"];
          /** @constant */
          status: "committed";
        }
      | {
          request_id: components["schemas"]["RequestId"];
          /** @constant */
          status: "outcome_unknown";
        };
    Status:
      | {
          receipt: components["schemas"]["Receipt"];
          /** @constant */
          status: "committed";
        }
      | {
          request_id: components["schemas"]["RequestId"];
          /** @constant */
          status: "not_known";
        }
      | {
          request_id: components["schemas"]["RequestId"];
          /** @constant */
          status: "outcome_unknown";
        };
  };
  responses: never;
  parameters: never;
  requestBodies: never;
  headers: never;
  pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
  capabilities: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Capabilities"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  capabilities_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  snapshot: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["HeadSnapshot"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  snapshot_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  events: {
    parameters: {
      query?: {
        after?: string;
      };
      header?: {
        /** @description Must agree with after when both are supplied. */
        "Last-Event-ID"?: string;
      };
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Bounded SSE stream. JSON data appears inside ready, repository-changed and reset frames. Authorized heartbeat comments do not advance IDs. Reset closes the stream. Resume the last fully parsed ID after EOF; post-header failures terminate the stream. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "text/event-stream": string;
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  events_head: {
    parameters: {
      query?: {
        after?: string;
      };
      header?: {
        /** @description Must agree with after when both are supplied. */
        "Last-Event-ID"?: string;
      };
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Bounded SSE stream. JSON data appears inside ready, repository-changed and reset frames. Authorized heartbeat comments do not advance IDs. Reset closes the stream. Resume the last fully parsed ID after EOF; post-header failures terminate the stream. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  object: {
    parameters: {
      query?: {
        at_op?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        kind:
          | "commit"
          | "tree"
          | "file"
          | "symlink"
          | "copy"
          | "view"
          | "operation";
        id: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_ContentObject"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  object_head: {
    parameters: {
      query?: {
        at_op?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        kind:
          | "commit"
          | "tree"
          | "file"
          | "symlink"
          | "copy"
          | "view"
          | "operation";
        id: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  log: {
    parameters: {
      query: {
        at_op?: string;
        cursor?: string;
        limit?: number;
        revset?: string;
        workspace?: string;
        user_email?: string;
        at_time_millis: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_GraphPage"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  log_head: {
    parameters: {
      query: {
        at_op?: string;
        cursor?: string;
        limit?: number;
        revset?: string;
        workspace?: string;
        user_email?: string;
        at_time_millis: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  operations: {
    parameters: {
      query?: {
        at_op?: string;
        heads?: string;
        cursor?: string;
        limit?: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_GraphPage"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  operations_head: {
    parameters: {
      query?: {
        at_op?: string;
        heads?: string;
        cursor?: string;
        limit?: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  operation: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        operation: components["schemas"]["OperationId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_OperationDetail"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  operation_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        operation: components["schemas"]["OperationId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  operation_diff: {
    parameters: {
      query?: {
        parent?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        operation: components["schemas"]["OperationId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_OperationDiffResponse"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  operation_diff_head: {
    parameters: {
      query?: {
        parent?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        operation: components["schemas"]["OperationId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  evolution: {
    parameters: {
      query: {
        at_op?: string;
        commit: string;
        cursor?: string;
        limit?: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_GraphPage"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  evolution_head: {
    parameters: {
      query: {
        at_op?: string;
        commit: string;
        cursor?: string;
        limit?: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  commit: {
    parameters: {
      query?: {
        at_op?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        commit: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_CommitDetail"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  commit_head: {
    parameters: {
      query?: {
        at_op?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        commit: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  tree: {
    parameters: {
      query?: {
        at_op?: string;
        cursor?: string;
        path?: string;
        limit?: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        commit: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_TreePage"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  tree_head: {
    parameters: {
      query?: {
        at_op?: string;
        cursor?: string;
        path?: string;
        limit?: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        commit: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  conflicts: {
    parameters: {
      query?: {
        at_op?: string;
        path?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        commit: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_ConflictResponse"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  conflicts_head: {
    parameters: {
      query?: {
        at_op?: string;
        path?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        commit: components["schemas"]["CommitId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  file: {
    parameters: {
      query?: {
        at_op?: string;
      };
      header?: {
        /** @description One bytes=start-end, bytes=start- or bytes=-count range. Bounds are clamped to EOF after verifying the complete native file. */
        Range?: string;
      };
      path: {
        repo: components["schemas"]["RepositoryId"];
        file: components["schemas"]["FileId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery. */
      200: {
        headers: {
          /** @description attachment; filename="<file ID>.bin" */
          "Content-Disposition"?: string;
          "Accept-Ranges"?: "bytes";
          "Content-Length"?: string;
          "x-jj-store-repository"?: components["schemas"]["RepositoryId"];
          "x-jj-store-epoch"?: components["schemas"]["EpochId"];
          "x-jj-store-operation"?: components["schemas"]["OperationId"];
          "x-jj-store-position"?: string;
          "x-jj-store-revision"?: string;
          [name: string]: unknown;
        };
        content: {
          "application/octet-stream": string;
        };
      };
      /** @description Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery. */
      206: {
        headers: {
          /** @description attachment; filename="<file ID>.bin" */
          "Content-Disposition"?: string;
          "Accept-Ranges"?: "bytes";
          "Content-Length"?: string;
          "x-jj-store-repository"?: components["schemas"]["RepositoryId"];
          "x-jj-store-epoch"?: components["schemas"]["EpochId"];
          "x-jj-store-operation"?: components["schemas"]["OperationId"];
          "x-jj-store-position"?: string;
          "x-jj-store-revision"?: string;
          /** @description bytes <start>-<inclusive end>/<full length> */
          "Content-Range"?: string;
          [name: string]: unknown;
        };
        content: {
          "application/octet-stream": string;
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Invalid or unsatisfiable byte range. */
      416: {
        headers: {
          /** @description bytes *\/<full length> */
          "Content-Range"?: string;
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  file_head: {
    parameters: {
      query?: {
        at_op?: string;
      };
      header?: {
        /** @description One bytes=start-end, bytes=start- or bytes=-count range. Bounds are clamped to EOF after verifying the complete native file. */
        Range?: string;
      };
      path: {
        repo: components["schemas"]["RepositoryId"];
        file: components["schemas"]["FileId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery. */
      200: {
        headers: {
          /** @description attachment; filename="<file ID>.bin" */
          "Content-Disposition"?: string;
          "Accept-Ranges"?: "bytes";
          "Content-Length"?: string;
          "x-jj-store-repository"?: components["schemas"]["RepositoryId"];
          "x-jj-store-epoch"?: components["schemas"]["EpochId"];
          "x-jj-store-operation"?: components["schemas"]["OperationId"];
          "x-jj-store-position"?: string;
          "x-jj-store-revision"?: string;
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery. */
      206: {
        headers: {
          /** @description attachment; filename="<file ID>.bin" */
          "Content-Disposition"?: string;
          "Accept-Ranges"?: "bytes";
          "Content-Length"?: string;
          "x-jj-store-repository"?: components["schemas"]["RepositoryId"];
          "x-jj-store-epoch"?: components["schemas"]["EpochId"];
          "x-jj-store-operation"?: components["schemas"]["OperationId"];
          "x-jj-store-position"?: string;
          "x-jj-store-revision"?: string;
          /** @description bytes <start>-<inclusive end>/<full length> */
          "Content-Range"?: string;
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid or unsatisfiable byte range. */
      416: {
        headers: {
          /** @description bytes *\/<full length> */
          "Content-Range"?: string;
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  diff: {
    parameters: {
      query: {
        at_op?: string;
        from: string;
        to: string;
        path?: string;
        content?: boolean;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Envelope_FileDiffResponse"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  diff_head: {
    parameters: {
      query: {
        at_op?: string;
        from: string;
        to: string;
        path?: string;
        content?: boolean;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  prepare: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody: {
      content: {
        "application/json": {
          operation: string;
        };
      };
    };
    responses: {
      /** @description Authorized result at the reported snapshot. */
      202: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["IndexTask"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  task: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        task: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["IndexTask"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  task_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        task: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  cancel: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        task: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["IndexTask"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  mutation_preview: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody: {
      content: {
        "application/json": {
          /** Format: uint32 */
          version: number;
          request_id: string;
          expected: {
            epoch: string;
            /**
             * Format: uint64
             * @description Publication order is distinct from native operation ancestry.
             */
            position: number;
            heads: string[];
          };
          command:
            | {
                parents: string[];
                description: string;
                workspace?: string | null;
                /** @constant */
                type: "new";
              }
            | {
                commit: string;
                description: string;
                /** @constant */
                type: "describe";
              }
            | {
                name: string;
                target: string;
                /** @constant */
                type: "bookmark_create";
              }
            | {
                name: string;
                expected: (string | null)[];
                target: string;
                /** @constant */
                type: "bookmark_move";
              }
            | {
                name: string;
                expected: (string | null)[];
                /** @constant */
                type: "bookmark_delete";
              }
            | {
                base: string;
                /** @constant */
                type: "reconcile_operations";
              }
            | {
                commits: string[];
                retain_bookmarks: boolean;
                restore_descendants: boolean;
                /** @constant */
                type: "abandon";
              };
        };
      };
    };
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Preview"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description stale_snapshot, stale_bookmark, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  mutation_apply: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody: {
      content: {
        "application/json": {
          /** Format: uint32 */
          version: number;
          request_id: string;
          expected: {
            epoch: string;
            /**
             * Format: uint64
             * @description Publication order is distinct from native operation ancestry.
             */
            position: number;
            heads: string[];
          };
          command:
            | {
                parents: string[];
                description: string;
                workspace?: string | null;
                /** @constant */
                type: "new";
              }
            | {
                commit: string;
                description: string;
                /** @constant */
                type: "describe";
              }
            | {
                name: string;
                target: string;
                /** @constant */
                type: "bookmark_create";
              }
            | {
                name: string;
                expected: (string | null)[];
                target: string;
                /** @constant */
                type: "bookmark_move";
              }
            | {
                name: string;
                expected: (string | null)[];
                /** @constant */
                type: "bookmark_delete";
              }
            | {
                base: string;
                /** @constant */
                type: "reconcile_operations";
              }
            | {
                commits: string[];
                retain_bookmarks: boolean;
                restore_descendants: boolean;
                /** @constant */
                type: "abandon";
              };
        };
      };
    };
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Outcome"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description stale_snapshot, stale_bookmark, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  mutation_status: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        request_id: components["schemas"]["RequestId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Status"];
        };
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description stale_snapshot, stale_bookmark, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
    };
  };
  mutation_status_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        request_id: components["schemas"]["RequestId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Invalid request, unknown query field or invalid cursor. */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Missing repository Bearer credential. */
      401: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Access denied, including revoked credentials or disallowed Origin. */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Object not committed in the selected snapshot or unknown route. */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Unsupported method. */
      405: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Native query canceled. */
      408: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description stale_snapshot, stale_bookmark, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
      409: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Index task expired or was lost on server restart. */
      410: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response resource limit exceeded. */
      413: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Busy, unavailable, corrupt or incomplete history. */
      503: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Request or response deadline exceeded. */
      504: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
}
