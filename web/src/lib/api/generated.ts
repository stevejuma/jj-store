export interface paths {
  "/v1/bootstrap": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog. */
    get: operations["bootstrap"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog. */
    head: operations["bootstrap_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repositories": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["repositories"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["repositories_head"];
    patch?: never;
    trace?: never;
  };
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
  "/v1/repos/{repo}/local/search": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data. */
    get: operations["local_search_settings"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["local_search_settings_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/search/save": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    post: operations["save_search_settings"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/search/control": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    post: operations["control_search"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/sync": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["local_sync_status"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["local_sync_status_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/sync/control": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    post: operations["local_sync_control"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/workspaces": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    get: operations["local_workspaces"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["local_workspaces_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/workspaces/pick": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    post: operations["pick_workspace_folder"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/local/workspaces/link": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    post: operations["link_workspace"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/search": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data. */
    get: operations["search"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["search_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/search/status": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data. */
    get: operations["search_status"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["search_status_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/sessions": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_sessions"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_sessions_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/sessions/{session}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_session"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_session_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_transcript"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_transcript_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/associations": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_associations"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_associations_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/checkpoints": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_checkpoints"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_checkpoints_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/checkpoints/{checkpoint}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_checkpoint"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_checkpoint_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["agent_checkpoint_version"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["agent_checkpoint_version_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reviews/threads": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["review_threads"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["review_threads_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reviews/threads/{thread}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state. */
    get: operations["review_thread"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["review_thread_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reviews/commands": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication. */
    post: operations["review_command_apply"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reviews/commands/{request_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication. */
    get: operations["review_command_status"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["review_command_status_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/threads": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_threads"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_threads_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/threads/{thread_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_thread"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_thread_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/threads/{thread_id}/timeline": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_events"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_events_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/threads/{thread_id}/messages/{message_id}/revisions/{message_revision}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_message"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_message_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/threads/commands": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    post: operations["collab_thread_apply"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/threads/commands/{request_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_status_threads"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_status_threads_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reactions/query": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    post: operations["collab_reactions"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reactions/commands": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    post: operations["collab_reaction_apply"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reactions/commands/{request_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_status_reactions"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_status_reactions_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reaction-emojis": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_emojis"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_emojis_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reaction-emojis/{emoji_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    post: operations["collab_emoji_define"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reaction-emojis/commands": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    post: operations["collab_emoji_retire"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reaction-emojis/commands/{request_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_status_reaction-emojis"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_status_reaction-emojis_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/reaction-assets/{asset}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents. */
    get: operations["collab_image"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["collab_image_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/work/browse": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    get: operations["work_browse"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["work_browse_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/work": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    get: operations["work_page"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["work_page_head"];
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/work/commands": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    post: operations["work_apply"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/v1/repos/{repo}/work/commands/{request_id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported. */
    get: operations["work_status"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["work_status_head"];
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
  "/v1/repos/{repo}/references": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["references"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["references_head"];
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
  "/v1/repos/{repo}/operations/{operation}/changes": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["operation_changes"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["operation_changes_head"];
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
  "/v1/repos/{repo}/commits/{commit}/paths": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["paths"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["paths_head"];
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
  "/v1/repos/{repo}/diff/paths": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** @description Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations. */
    get: operations["diff_paths"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    /** @description Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned. */
    head: operations["diff_paths_head"];
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
  "/v1/repos/{repo}/conflict-uploads": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** @description Requires current repository write permission before body consumption. Binary staging only; never publishes history. Binding must name the exact current single-head snapshot and a committed target. Repeating identical bytes/binding returns the same scoped handle. Deadline or unavailable responses permit safe re-upload; no mutation receipt is created. Upload limits are advertised by capabilities. */
    post: operations["conflict_upload"];
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
    ErrorBody: {
      /** Format: uint32 */
      version: number;
      code: string;
      snapshot?: components["schemas"]["ReadSnapshot"] | null;
      heads?: components["schemas"]["HeadSnapshot"] | null;
      search_status?: components["schemas"]["SearchStatus"] | null;
    };
    ReadSnapshot: {
      /** Format: uint32 */
      version: number;
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      manifest_revision: number;
      /** Format: uint64 */
      code_state_version: number;
      heads: components["schemas"]["OperationId"][];
      operation_id: components["schemas"]["OperationId"];
    };
    RepositoryId: string;
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
      /** Format: uint64 */
      code_state_version: number;
      heads: components["schemas"]["OperationId"][];
    };
    SearchStatus: {
      /** Format: uint32 */
      version: number;
      repository: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      repository_position: components["schemas"]["WalPosition"];
      durable_position: components["schemas"]["WalPosition"] | null;
      indexed_position: components["schemas"]["WalPosition"] | null;
      index_generation: components["schemas"]["RecordDigest"] | null;
      is_stale: boolean;
      degraded: boolean;
    };
    RecordDigest: string;
    Bootstrap: {
      /** Format: uint32 */
      version: number;
      access_mode: components["schemas"]["AccessMode"];
      capabilities: components["schemas"]["Capabilities"];
      storage: components["schemas"]["StorageState"];
      backups: components["schemas"]["BackupStatus"][];
    };
    /** @enum {string} */
    AccessMode: "authenticated" | "local_trusted";
    Capabilities: {
      /** Format: uint32 */
      version: number;
      native_profile: string;
      authentication: string;
      read_api: boolean;
      browsing_windows: boolean;
      index_tasks: boolean;
      events: boolean;
      search: boolean;
      local_workspace_links?: boolean | null;
      local_folder_picker?: boolean | null;
      local_search_management?: boolean | null;
      local_sync_management?: boolean | null;
      typed_mutations: string[];
      /** Format: uint */
      mutation_request_bytes: number;
      /** Format: uint64 */
      conflict_upload_bytes: number;
      conflict_resolution_actions: string[];
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
    StorageState: {
      ready: boolean;
    };
    BackupStatus: {
      target: string;
      driver: string;
      transfer: components["schemas"]["BackupTransfer"];
      last_verified_restore_point:
        components["schemas"]["VerifiedRestorePoint"] | null;
    };
    BackupTransfer: {
      state: string;
      /** Format: uint64 */
      completed_objects: number;
      /** Format: uint64 */
      total_objects: number | null;
      /** Format: uint64 */
      completed_bytes: number;
      /** Format: uint64 */
      total_bytes: number | null;
    };
    VerifiedRestorePoint: {
      repository: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      /** Format: uint64 */
      position: number;
      operation: components["schemas"]["OperationId"];
      /** Format: int64 */
      verified_at_millis: number;
    };
    RepositoryPage: {
      /** Format: uint32 */
      version: number;
      /** Format: uint64 */
      generation: number;
      repositories: components["schemas"]["RepositorySummary"][];
      next_cursor?: string | null;
    };
    RepositorySummary: {
      id: components["schemas"]["RepositoryId"];
      name: string;
      permission: components["schemas"]["EffectivePermission"];
      principal: components["schemas"]["PrincipalId"];
    };
    /** @enum {string} */
    EffectivePermission: "read" | "write" | "admin";
    PrincipalId: string;
    LocalSearchSettings: {
      /** Format: uint32 */
      version: number;
      revision: string;
      desired: components["schemas"]["SearchConfiguration"];
      effective: components["schemas"]["SearchConfiguration"];
      operation: components["schemas"]["SearchOperation"];
      error: string | null;
    };
    SearchConfiguration: {
      enabled: boolean;
      index_source: boolean;
      scopes: {
        [key: string]: components["schemas"]["SearchScope"];
      };
      include_prefixes: string[];
      exclude_prefixes: string[];
      /** Format: uint64 */
      index_bytes: number;
    };
    SearchScope:
      | {
          name: string;
          /** @constant */
          kind: "bookmark";
        }
      | {
          name: string;
          /** @constant */
          kind: "workspace";
        };
    /** @enum {string} */
    SearchOperation: "idle" | "applying" | "rebuilding" | "retrying" | "failed";
    LocalSyncStatus: {
      /** Format: uint32 */
      version: number;
      configured: boolean;
      worker: components["schemas"]["SyncStatus"] | null;
    };
    SyncStatus: {
      /** Format: uint32 */
      version: number;
      repository: components["schemas"]["RepositoryId"];
      local_ready: boolean;
      offline_ready: boolean;
      phase: components["schemas"]["SyncPhase"];
      local_replica: components["schemas"]["ReplicaId"] | null;
      peer_replica: components["schemas"]["ReplicaId"] | null;
      local_frontier: components["schemas"]["SyncFrontier"] | null;
      peer_frontier: components["schemas"]["SyncFrontier"] | null;
      pushed: components["schemas"]["SyncFrontier"] | null;
      pulled: components["schemas"]["SyncFrontier"] | null;
      /** Format: uint64 */
      local_pending_positions: number | null;
      /** Format: uint64 */
      peer_pending_positions: number | null;
      /** Format: uint64 */
      checkpoint_revision: number;
      /** Format: uint */
      operation_heads: number;
      reconciliation_required: boolean;
      pull: components["schemas"]["SyncDirectionStatus"];
      push: components["schemas"]["SyncDirectionStatus"];
      search: components["schemas"]["SyncSearchStatus"];
    };
    /** @enum {string} */
    SyncPhase: "connecting" | "running" | "paused" | "degraded" | "stopped";
    ReplicaId: string;
    SyncFrontier: {
      replica: components["schemas"]["ReplicaId"];
      log_epoch: components["schemas"]["EpochId"];
      through: components["schemas"]["WalPosition"];
      observed_upper_bound: components["schemas"]["WalPosition"];
    };
    SyncDirectionStatus: {
      /** Format: uint32 */
      consecutive_failures: number;
      circuit_open: boolean;
      last_fault: components["schemas"]["SyncFault"] | null;
    };
    /** @enum {string} */
    SyncFault:
      | "transient_endpoint"
      | "permanent_endpoint"
      | "integrity"
      | "limit"
      | "cursor_conflict"
      | "outcome_unknown";
    SyncSearchStatus: {
      configured: boolean;
      repository_position: components["schemas"]["WalPosition"] | null;
      indexed_position: components["schemas"]["WalPosition"] | null;
      degraded: boolean;
    };
    WorkspaceLinks: {
      /** Format: uint32 */
      version: number;
      /** Format: uint64 */
      revision: number;
      links: components["schemas"]["WorkspaceLink"][];
    };
    WorkspaceLink: {
      id: string;
      path: string;
      profile: components["schemas"]["WorkspaceProfile"];
      availability: components["schemas"]["WorkspaceAvailability"];
    };
    /** @enum {string} */
    WorkspaceProfile: "native" | "local_jj_git";
    /** @enum {string} */
    WorkspaceAvailability: "ready" | "unavailable";
    FolderSelection: {
      /** Format: uint32 */
      version: number;
      path: string | null;
    };
    SearchPage: {
      /** Format: uint32 */
      version: number;
      repository: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      repository_position: components["schemas"]["WalPosition"];
      indexed_position: components["schemas"]["WalPosition"];
      index_generation: components["schemas"]["RecordDigest"];
      selected_view: components["schemas"]["ReadSnapshot"] | null;
      scope_snapshots: {
        [key: string]: components["schemas"]["SearchScopeSnapshot"];
      };
      results: components["schemas"]["SearchDocument"][];
      is_stale: boolean;
      degraded: boolean;
      next_cursor: string | null;
      truncated: boolean;
    };
    SearchScopeSnapshot: {
      state: components["schemas"]["SearchScopeState"];
      operation: components["schemas"]["OperationId"] | null;
      commits: components["schemas"]["CommitId"][];
    };
    /** @enum {string} */
    SearchScopeState:
      "ready" | "conflicted" | "missing" | "ambiguous_heads" | "disabled";
    CommitId: string;
    /**
     * @description Owned extraction output. Native identity fields are stored exactly; the body
     *     is indexed but never stored in query hits. Fetch source via an authorized ID.
     */
    SearchDocument: {
      key: components["schemas"]["SearchDocumentKey"];
      title: string;
      body: string;
      actor: string | null;
      author: string;
      workspace: string | null;
      operation: components["schemas"]["OperationId"] | null;
      commit: components["schemas"]["CommitId"] | null;
      change: components["schemas"]["ChangeId"] | null;
      file: components["schemas"]["FileId"] | null;
      parents: components["schemas"]["CommitId"][];
      predecessors: components["schemas"]["CommitId"][];
      attributes: {
        [key: string]: string;
      };
      language: string | null;
      has_conflict: boolean;
      /** Format: int64 */
      timestamp_millis: number;
    };
    SearchDocumentKey:
      | {
          id: components["schemas"]["OperationId"];
          /** @constant */
          kind: "operation";
        }
      | {
          id: components["schemas"]["CommitId"];
          /** @constant */
          kind: "commit";
        }
      | {
          scope: string;
          path: string;
          /** @constant */
          kind: "file";
        };
    ChangeId: string;
    FileId: string;
    AgentSessionPage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["AgentReadSnapshot"];
      sessions: components["schemas"]["AgentSessionSummary"][];
      next_cursor: string | null;
    };
    AgentReadSnapshot: {
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      manifest_revision: number;
      agent_root: components["schemas"]["RecordDigest"];
      /** Format: uint64 */
      agent_state_version: number;
    };
    AgentSessionSummary: {
      session_id: components["schemas"]["AgentSessionId"];
      snapshot_id: components["schemas"]["AgentObjectId"];
      integration: components["schemas"]["IntegrationId"];
      state: components["schemas"]["SessionState"];
      parent_session_id: components["schemas"]["AgentSessionId"] | null;
      /** Format: int64 */
      first_activity_at_millis: number | null;
      /** Format: int64 */
      last_activity_at_millis: number | null;
      coverage: components["schemas"]["AgentCoverage"];
      /** Format: uint */
      evidence_count: number;
      /** Format: uint */
      event_batch_count: number;
      /** Format: uint */
      transcript_count: number;
      /** Format: uint */
      tool_count: number;
    };
    AgentSessionId: string;
    AgentObjectId: string;
    /** @enum {string} */
    IntegrationId: "codex" | "cursor" | "open_code" | "owned_controller";
    /** @enum {string} */
    SessionState: "discovered" | "active" | "idle" | "ended" | "unknown";
    AgentCoverage: {
      completeness: components["schemas"]["CompletenessV1"];
      /** Format: uint32 */
      gaps: number;
      /** Format: uint32 */
      redacted_fields: number;
      truncated: boolean;
    };
    /** @enum {string} */
    CompletenessV1: "complete" | "partial" | "unknown";
    AgentSessionDetail: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["AgentReadSnapshot"];
      session: components["schemas"]["AgentSessionSummary"];
      evidence: components["schemas"]["AgentObjectId"][];
      event_batches: components["schemas"]["AgentObjectId"][];
      transcripts: components["schemas"]["AgentTranscriptDescriptor"][];
      tools: components["schemas"]["AgentToolLink"][];
      activity: components["schemas"]["AgentActivity"][];
      activity_truncated: boolean;
      code: components["schemas"]["AgentCodeAssociation"][];
      code_truncated: boolean;
    };
    AgentTranscriptDescriptor: {
      transcript_id: components["schemas"]["AgentObjectId"];
      source_instance_id: components["schemas"]["SourceInstanceId"];
      /** Format: uint64 */
      source_generation: number;
      source_format: string;
      decoder_version: string;
      sanitizer_version: string;
      representation: components["schemas"]["SanitizedRepresentation"];
      /** Format: uint64 */
      logical_byte_length: number;
      /** Format: uint64 */
      record_count: number;
      /** Format: uint32 */
      gaps: number;
    };
    SourceInstanceId: string;
    /** @enum {string} */
    SanitizedRepresentation: "sanitized_native" | "canonical_producer";
    AgentToolLink: {
      tool_call_id: components["schemas"]["ToolCallId"];
      snapshot_id: components["schemas"]["AgentObjectId"];
    };
    ToolCallId: string;
    AgentActivity:
      | {
          /** Format: uint64 */
          sequence: number;
          event_id: components["schemas"]["AgentEventId"];
          state: components["schemas"]["SessionState"];
          /** @constant */
          type: "session_state";
        }
      | {
          reaction_target?: components["schemas"]["ReactionTargetV1"] | null;
          /** Format: uint64 */
          sequence: number;
          event_id: components["schemas"]["AgentEventId"];
          turn_id: components["schemas"]["TurnId"] | null;
          message_id: components["schemas"]["MessageId"];
          role: components["schemas"]["MessageRole"];
          content: components["schemas"]["AgentContent"];
          /** @constant */
          type: "message";
        }
      | {
          /** Format: uint64 */
          sequence: number;
          event_id: components["schemas"]["AgentEventId"];
          tool: components["schemas"]["AgentToolActivity"];
          /** @constant */
          type: "tool";
        }
      | {
          /** Format: uint64 */
          sequence: number;
          event_id: components["schemas"]["AgentEventId"];
          tool_call_id: components["schemas"]["ToolCallId"] | null;
          state: components["schemas"]["PermissionState"];
          /** @constant */
          type: "permission";
        }
      | {
          /** Format: uint64 */
          sequence: number;
          event_id: components["schemas"]["AgentEventId"];
          path: string;
          /** @constant */
          type: "file_observed";
        }
      | {
          /** Format: uint64 */
          sequence: number;
          event_id: components["schemas"]["AgentEventId"];
          /** Format: uint64 */
          missing_from: number;
          /** Format: uint64 */
          missing_to: number;
          /** @constant */
          type: "capture_gap";
        };
    AgentEventId: string;
    /** @description Identity, immutable content and validation witnesses are explicit, never a UI position. */
    ReactionTargetV1:
      | {
          thread_id: components["schemas"]["ThreadId"];
          /** @constant */
          type: "thread";
        }
      | {
          thread_id: components["schemas"]["ThreadId"];
          message_id: components["schemas"]["HumanMessageId"];
          revision: components["schemas"]["CollabObjectRef"];
          /** @constant */
          type: "human_message";
        }
      | {
          session_id: components["schemas"]["AgentSessionId"];
          message_id: components["schemas"]["MessageId"];
          event_id: components["schemas"]["AgentEventId"];
          batch: components["schemas"]["AgentObjectRef"];
          content: components["schemas"]["AgentObjectRef"];
          /** @constant */
          type: "agent_message";
        }
      | {
          session_id: components["schemas"]["AgentSessionId"];
          tool_id: components["schemas"]["ToolCallId"];
          snapshot: components["schemas"]["AgentObjectRef"];
          /** @constant */
          type: "tool_invocation";
        }
      | {
          session_id: components["schemas"]["AgentSessionId"];
          tool_id: components["schemas"]["ToolCallId"];
          snapshot: components["schemas"]["AgentObjectRef"];
          content: components["schemas"]["AgentObjectRef"];
          /** @constant */
          type: "tool_output";
        }
      | {
          thread_id: components["schemas"]["ReviewThreadId"];
          /** @constant */
          type: "review_thread";
        }
      | {
          thread_id: components["schemas"]["ReviewThreadId"];
          comment_id: components["schemas"]["CommentId"];
          revision: components["schemas"]["ReviewObjectRef"];
          /** @constant */
          type: "review_comment";
        }
      | {
          session_id: components["schemas"]["AgentSessionId"];
          checkpoint_id: components["schemas"]["AgentCheckpointId"];
          version: components["schemas"]["AgentObjectRef"];
          /** @constant */
          type: "agent_checkpoint";
        };
    ThreadId: string;
    HumanMessageId: string;
    CollabObjectRef: {
      kind: components["schemas"]["CollabObjectKind"];
      id: components["schemas"]["CollabObjectId"];
    };
    /** @enum {string} */
    CollabObjectKind:
      | "index_page"
      | "thread_version"
      | "thread_event"
      | "human_message_revision"
      | "thread_link"
      | "reaction"
      | "reaction_retraction"
      | "reaction_slot"
      | "custom_emoji_version"
      | "image_asset";
    CollabObjectId: string;
    MessageId: string;
    AgentObjectRef: {
      kind: components["schemas"]["AgentObjectKind"];
      id: components["schemas"]["AgentObjectId"];
    };
    /** @enum {string} */
    AgentObjectKind:
      | "index_page"
      | "session_snapshot"
      | "turn_snapshot"
      | "message_snapshot"
      | "tool_snapshot"
      | "intent"
      | "observation_batch"
      | "event_batch"
      | "transcript_manifest"
      | "transcript_page"
      | "transcript_chunk"
      | "content"
      | "checkpoint_version"
      | "association"
      | "alias"
      | "relation_page"
      | "source_state"
      | "projection_catalog"
      | "catalog_conflict";
    ReviewThreadId: string;
    CommentId: string;
    ReviewObjectRef: {
      kind: components["schemas"]["ReviewObjectKind"];
      id: components["schemas"]["ReviewObjectId"];
    };
    /** @enum {string} */
    ReviewObjectKind:
      | "index_page"
      | "anchor"
      | "comment_body"
      | "comment_revision"
      | "thread_event"
      | "event_page"
      | "thread_snapshot"
      | "relation_page"
      | "status_page";
    ReviewObjectId: string;
    AgentCheckpointId: string;
    TurnId: string;
    /** @enum {string} */
    MessageRole: "user" | "assistant" | "system" | "tool";
    AgentContent: {
      media_type: components["schemas"]["SanitizedMediaType"];
      content: string;
    };
    /** @enum {string} */
    SanitizedMediaType: "json" | "text";
    AgentToolActivity: {
      tool_call_id: components["schemas"]["ToolCallId"];
      snapshot_id: components["schemas"]["AgentObjectId"];
      turn_id: components["schemas"]["TurnId"] | null;
      native_name: string;
      kind: components["schemas"]["ToolKind"];
      state: components["schemas"]["ToolState"];
      input: components["schemas"]["AgentContent"] | null;
      output: components["schemas"]["AgentContent"] | null;
      observed_paths: string[];
      code: components["schemas"]["AgentCodeAssociation"][];
    };
    /** @enum {string} */
    ToolKind:
      | "read"
      | "edit"
      | "delete"
      | "move"
      | "search"
      | "execute"
      | "fetch"
      | "plan"
      | "delegate"
      | "other";
    /** @enum {string} */
    ToolState:
      | "pending"
      | "running"
      | "succeeded"
      | "failed"
      | "cancelled"
      | "interrupted"
      | "skipped"
      | "unknown";
    AgentCodeAssociation: {
      association_id: components["schemas"]["AssociationId"];
      object_id: components["schemas"]["AgentObjectId"];
      session_id: components["schemas"]["AgentSessionId"];
      tool_call_ids: components["schemas"]["ToolCallId"][];
      relation: components["schemas"]["AssociationRelation"];
      before_operations: components["schemas"]["OperationId"][];
      after_operations: components["schemas"]["OperationId"][];
      before_commits: components["schemas"]["CommitId"][];
      after_commits: components["schemas"]["CommitId"][];
      changes: components["schemas"]["ChangeId"][];
      paths: string[];
      /** Format: int64 */
      created_at_millis: number;
    };
    AssociationId: string;
    /** @enum {string} */
    AssociationRelation:
      | "initiated_revision"
      | "rewrote_revision"
      | "contributed_to_snapshot"
      | "observed_during_interval"
      | "read_revision"
      | "validated_revision"
      | "manual_assertion";
    /** @enum {string} */
    PermissionState: "not_observed" | "requested" | "granted" | "denied";
    AgentTranscriptPage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["AgentReadSnapshot"];
      session_id: components["schemas"]["AgentSessionId"];
      transcript_id: components["schemas"]["AgentObjectId"];
      source_instance_id: components["schemas"]["SourceInstanceId"];
      /** Format: uint64 */
      source_generation: number;
      chunks: components["schemas"]["AgentTranscriptChunk"][];
      next_cursor: string | null;
    };
    AgentTranscriptChunk: {
      /** Format: uint64 */
      sequence: number;
      media_type: components["schemas"]["SanitizedMediaType"];
      content: string;
    };
    AgentRelationPage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["AgentReadSnapshot"];
      subject: components["schemas"]["AgentRelationSubject"];
      relations: components["schemas"]["AgentRelationLink"][];
      next_cursor: string | null;
    };
    AgentRelationSubject:
      | {
          session_id: components["schemas"]["AgentSessionId"];
          /** @constant */
          subject: "session";
        }
      | {
          session_id: components["schemas"]["AgentSessionId"];
          tool_call_id: components["schemas"]["ToolCallId"];
          /** @constant */
          subject: "tool";
        }
      | {
          operation_id: components["schemas"]["OperationId"];
          /** @constant */
          subject: "operation";
        }
      | {
          commit_id: components["schemas"]["CommitId"];
          /** @constant */
          subject: "revision";
        }
      | {
          checkpoint_id: components["schemas"]["AgentCheckpointId"];
          /** @constant */
          subject: "checkpoint";
        };
    AgentRelationLink: {
      kind: components["schemas"]["AgentRelationKind"];
      object_id: components["schemas"]["AgentObjectId"];
      association: components["schemas"]["AgentCodeAssociation"] | null;
    };
    /** @enum {string} */
    AgentRelationKind: "checkpoint" | "association";
    AgentCheckpointPage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["AgentReadSnapshot"];
      checkpoints: components["schemas"]["AgentCheckpointSummary"][];
      next_cursor: string | null;
    };
    AgentCheckpointSummary: {
      checkpoint_id: components["schemas"]["AgentCheckpointId"];
      version_id: components["schemas"]["AgentObjectId"];
      session_id: components["schemas"]["AgentSessionId"];
      turn_id: components["schemas"]["TurnId"] | null;
      scope_id: components["schemas"]["ProvenanceScopeId"] | null;
      before: components["schemas"]["AgentCodeAnchor"];
      after: components["schemas"]["AgentCodeAnchor"];
      code_relation: components["schemas"]["CodeRelation"];
      evidence_method: components["schemas"]["EvidenceMethod"];
      source_trust: components["schemas"]["SourceTrust"];
      exclusive_write_scope: boolean;
      coverage: components["schemas"]["AgentCoverage"];
      capture_status: components["schemas"]["CaptureStatusV1"];
      execution_outcome: components["schemas"]["ExecutionOutcomeV1"];
      /** Format: uint */
      event_batch_count: number;
      /** Format: uint */
      tool_count: number;
      /** Format: uint */
      association_count: number;
      supersedes: components["schemas"]["AgentObjectId"] | null;
    };
    ProvenanceScopeId: string;
    AgentCodeAnchor: {
      operations: components["schemas"]["OperationId"][];
      commits: components["schemas"]["CommitId"][];
    };
    /** @enum {string} */
    CodeRelation: "changed" | "no_change" | "not_observed";
    /** @enum {string} */
    EvidenceMethod:
      | "controlled_scope"
      | "hook_boundary"
      | "transcript_recovery"
      | "historical_import"
      | "manual_assertion";
    /** @enum {string} */
    SourceTrust:
      | "authenticated_controller"
      | "local_user_hook"
      | "locally_trusted_plugin"
      | "approved_harness_api"
      | "local_transcript"
      | "imported_archive"
      | "untrusted_assertion";
    /** @enum {string} */
    CaptureStatusV1: "complete" | "partial" | "missing";
    /** @enum {string} */
    ExecutionOutcomeV1:
      "succeeded" | "failed" | "cancelled" | "interrupted" | "unknown";
    AgentCheckpointDetail: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["AgentReadSnapshot"];
      checkpoint: components["schemas"]["AgentCheckpointSummary"];
      session_snapshot_id: components["schemas"]["AgentObjectId"];
      transcript_id: components["schemas"]["AgentObjectId"];
      event_batches: components["schemas"]["AgentObjectId"][];
      tools: components["schemas"]["AgentToolLink"][];
      associations: components["schemas"]["AgentObjectId"][];
    };
    ReviewThreadPage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["ReviewReadSnapshot"];
      threads: components["schemas"]["ReviewThreadSummary"][];
      next_cursor: string | null;
    };
    ReviewReadSnapshot: {
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      manifest_revision: number;
      review_root: components["schemas"]["RecordDigest"];
      /** Format: uint64 */
      review_state_version: number;
    };
    ReviewThreadSummary: {
      thread_id: components["schemas"]["ReviewThreadId"];
      snapshot_id: components["schemas"]["ReviewObjectId"];
      original_anchor_id: components["schemas"]["ReviewObjectId"];
      anchor: components["schemas"]["ArtifactAnchorV1"];
      event_page_root_id: components["schemas"]["ReviewObjectId"];
      status: components["schemas"]["ThreadStatusV1"];
      selected_workflow_action: components["schemas"]["ReviewActionId"] | null;
      selected_workflow_actions: components["schemas"]["ReviewActionId"][];
      /** Format: uint64 */
      thread_version: number;
      /** Format: uint64 */
      content_version: number;
      /** Format: uint64 */
      workflow_version: number;
      /** Format: uint */
      comment_count: number;
      /** Format: uint */
      comment_conflict_count: number;
      /** Format: uint64 */
      last_event_sequence: number;
      presentation?: components["schemas"]["ReviewSummaryPresentation"] | null;
    };
    ArtifactAnchorV1: {
      /** Format: uint32 */
      schema_version: number;
      repository_id: components["schemas"]["RepositoryId"];
      target: components["schemas"]["ArtifactTargetV1"];
      created_context: components["schemas"]["CreatedContextV1"];
      sensitivity: components["schemas"]["SensitivityV1"];
    };
    ArtifactTargetV1:
      | {
          /** @constant */
          target: "code_text";
          value: components["schemas"]["CodeTextAnchorV1"];
        }
      | {
          /** @constant */
          target: "code_artifact";
          value: components["schemas"]["CodeArtifactAnchorV1"];
        }
      | {
          /** @constant */
          target: "agent_text";
          value: components["schemas"]["AgentTextAnchorV1"];
        }
      | {
          /** @constant */
          target: "agent_artifact";
          value: components["schemas"]["AgentArtifactAnchorV1"];
        }
      | {
          /** @constant */
          target: "revision";
          value: {
            operation_id: components["schemas"]["OperationId"];
            commit_id: components["schemas"]["CommitId"];
            change_id: components["schemas"]["ChangeId"] | null;
          };
        }
      | {
          /** @constant */
          target: "operation";
          value: {
            operation_id: components["schemas"]["OperationId"];
          };
        }
      | {
          /** @constant */
          target: "repository";
        };
    CodeTextAnchorV1: {
      operation_id: components["schemas"]["OperationId"];
      commit_id: components["schemas"]["CommitId"];
      change_id: components["schemas"]["ChangeId"] | null;
      path: string;
      file_id: components["schemas"]["FileId"];
      value_kind: components["schemas"]["CodeValueKind"];
      text: components["schemas"]["TextSelectionV1"];
      diff_context: components["schemas"]["DiffContextV1"] | null;
    };
    /** @enum {string} */
    CodeValueKind:
      | "file"
      | "binary_file"
      | "empty_file"
      | "symlink"
      | "gitlink"
      | "conflict_term"
      | "conflict_artifact";
    TextSelectionV1: {
      encoding: components["schemas"]["TextEncoding"];
      start_byte: components["schemas"]["ByteOffset"];
      end_byte: components["schemas"]["ByteOffset"];
      selected_sha256: components["schemas"]["ReviewDigest"];
    };
    /** @enum {string} */
    TextEncoding: "utf8";
    ByteOffset: string;
    ReviewDigest: string;
    DiffContextV1: {
      before_operation_id: components["schemas"]["OperationId"];
      before_commit_id: components["schemas"]["CommitId"];
      before_path: string | null;
      after_operation_id: components["schemas"]["OperationId"];
      after_commit_id: components["schemas"]["CommitId"];
      after_path: string | null;
      side: components["schemas"]["DiffSide"];
      comparison: components["schemas"]["ComparisonKind"];
    };
    /** @enum {string} */
    DiffSide: "before" | "after";
    /** @enum {string} */
    ComparisonKind: "explicit_pair";
    CodeArtifactAnchorV1: {
      operation_id: components["schemas"]["OperationId"];
      commit_id: components["schemas"]["CommitId"];
      change_id: components["schemas"]["ChangeId"] | null;
      path: string;
      object: components["schemas"]["ObjectKey"];
      value_kind: components["schemas"]["CodeValueKind"];
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
    SymlinkId: string;
    CopyId: string;
    ViewId: string;
    AgentTextAnchorV1: {
      session_id: components["schemas"]["AgentSessionId"];
      session_snapshot: components["schemas"]["AgentObjectRef"];
      owner: components["schemas"]["AgentContentOwnerV1"];
      block_id: components["schemas"]["BlockId"];
      block_version: components["schemas"]["AgentObjectRef"];
      part: components["schemas"]["AgentContentPart"];
      content: components["schemas"]["AgentObjectRef"];
      representation_version: string;
      sanitizer_version: string;
      normalizer_version: string;
      source_coverage: components["schemas"]["SourceCoverageV1"];
      text: components["schemas"]["TextSelectionV1"];
    };
    AgentContentOwnerV1:
      | {
          tool_call_id: components["schemas"]["ToolCallId"];
          snapshot: components["schemas"]["AgentObjectRef"];
          attempt_id: components["schemas"]["ToolAttemptId"];
          /** @constant */
          kind: "tool";
        }
      | {
          message_id: components["schemas"]["MessageId"];
          snapshot: components["schemas"]["AgentObjectRef"];
          /** @constant */
          kind: "message";
        }
      | {
          block_id: components["schemas"]["BlockId"];
          snapshot: components["schemas"]["AgentObjectRef"];
          /** @constant */
          kind: "plan";
        };
    ToolAttemptId: string;
    BlockId: string;
    /** @enum {string} */
    AgentContentPart:
      "input" | "output" | "message" | "plan" | "exposed_summary";
    /** @enum {string} */
    SourceCoverageV1:
      "complete" | "partial" | "redacted" | "truncated" | "unknown";
    AgentArtifactAnchorV1: {
      session_id: components["schemas"]["AgentSessionId"];
      session_snapshot: components["schemas"]["AgentObjectRef"];
      owner: components["schemas"]["AgentContentOwnerV1"];
    };
    CreatedContextV1: {
      /** Format: uint64 */
      publication_position: number | null;
      /** Format: uint64 */
      code_state_version: number | null;
      agent_root: components["schemas"]["RecordDigest"] | null;
    };
    /** @enum {string} */
    SensitivityV1: "public" | "repository" | "restricted";
    /** @enum {string} */
    ThreadStatusV1: "open" | "resolved";
    ReviewActionId: string;
    ReviewSummaryPresentation: {
      created_by: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      created_at_millis: number;
      /** Format: int64 */
      updated_at_millis: number;
      excerpt: string | null;
    };
    ReviewThreadDetail: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["ReviewReadSnapshot"];
      thread: components["schemas"]["ReviewThreadSummary"];
      anchor: components["schemas"]["ReviewAnchor"];
      comments: components["schemas"]["ReviewComment"][];
      comment_conflicts: components["schemas"]["ReviewCommentConflict"][];
      events: components["schemas"]["ReviewEvent"][];
    };
    ReviewAnchor: {
      object_id: components["schemas"]["ReviewObjectId"];
      value: components["schemas"]["ArtifactAnchorV1"];
    };
    ReviewComment: {
      comment_id: components["schemas"]["CommentId"];
      revision_id: components["schemas"]["ReviewObjectId"];
      parent_comment_id: components["schemas"]["CommentId"] | null;
      author: components["schemas"]["PrincipalId"];
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      created_at_millis: number;
      previous_revision_id: components["schemas"]["ReviewObjectId"] | null;
      superseded_revision_ids: components["schemas"]["ReviewObjectId"][];
      content: components["schemas"]["ReviewCommentContent"];
    };
    ReviewCommentContent:
      | {
          body_id: components["schemas"]["ReviewObjectId"];
          format: components["schemas"]["ReviewBodyFormatV1"];
          text: string;
          sanitizer_version: string;
          /** @constant */
          state: "visible";
        }
      | {
          reason: string | null;
          /** @constant */
          state: "tombstoned";
        }
      | {
          reason: string;
          /** @constant */
          state: "hidden";
        };
    /** @enum {string} */
    ReviewBodyFormatV1: "plain_text" | "safe_markdown";
    ReviewCommentConflict: {
      comment_id: components["schemas"]["CommentId"];
      parent_comment_id: components["schemas"]["CommentId"] | null;
      alternatives: components["schemas"]["ReviewCommentVersion"][];
    };
    ReviewCommentVersion: {
      revision_id: components["schemas"]["ReviewObjectId"];
      author: components["schemas"]["PrincipalId"];
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      created_at_millis: number;
      previous_revision_id: components["schemas"]["ReviewObjectId"] | null;
      superseded_revision_ids: components["schemas"]["ReviewObjectId"][];
      content: components["schemas"]["ReviewCommentContent"];
    };
    ReviewEvent: {
      event_id: components["schemas"]["ReviewObjectId"];
      causal_parent_ids: components["schemas"]["ReviewObjectId"][];
      /** Format: uint64 */
      sequence: number;
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      accepted_at_millis: number;
      event: components["schemas"]["ReviewEventKind"];
    };
    ReviewEventKind:
      | {
          anchor_id: components["schemas"]["ReviewObjectId"];
          initial_comment_id: components["schemas"]["CommentId"];
          initial_revision_id: components["schemas"]["ReviewObjectId"];
          /** @constant */
          action: "created";
        }
      | {
          comment_id: components["schemas"]["CommentId"];
          parent_comment_id: components["schemas"]["CommentId"] | null;
          revision_id: components["schemas"]["ReviewObjectId"];
          /** @constant */
          action: "reply_added";
        }
      | {
          comment_id: components["schemas"]["CommentId"];
          previous_revision_id: components["schemas"]["ReviewObjectId"];
          revision_id: components["schemas"]["ReviewObjectId"];
          /** @constant */
          action: "comment_revised";
        }
      | {
          comment_id: components["schemas"]["CommentId"];
          superseded_revision_ids: components["schemas"]["ReviewObjectId"][];
          revision_id: components["schemas"]["ReviewObjectId"];
          /** @constant */
          action: "comment_conflict_resolved";
        }
      | {
          action_id: components["schemas"]["ReviewActionId"];
          reason: string | null;
          /** @constant */
          action: "resolved";
        }
      | {
          action_id: components["schemas"]["ReviewActionId"];
          reason: string | null;
          /** @constant */
          action: "reopened";
        };
    ReviewCommandOutcome:
      | {
          receipt: components["schemas"]["ReviewCommandReceipt"];
          /** @constant */
          status: "committed";
        }
      | {
          request_id: components["schemas"]["RequestId"];
          /** @constant */
          status: "outcome_unknown";
        };
    ReviewCommandReceipt: {
      /** Format: uint32 */
      version: number;
      request_id: components["schemas"]["RequestId"];
      publication_id: components["schemas"]["PublicationId"];
      request_digest: components["schemas"]["IntentDigest"];
      accepted: components["schemas"]["Snapshot"];
      result: components["schemas"]["ReviewCommandResultV1"];
    };
    RequestId: string;
    PublicationId: string;
    IntentDigest: string;
    Snapshot: {
      epoch: components["schemas"]["EpochId"];
      position: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      code_state_version?: number;
      heads: components["schemas"]["OperationId"][];
    };
    ReviewCommandResultV1: {
      /** Format: uint32 */
      schema_version: number;
      thread_id: components["schemas"]["ReviewThreadId"];
      comment_id: components["schemas"]["CommentId"] | null;
      action_id: components["schemas"]["ReviewActionId"] | null;
      thread_snapshot: components["schemas"]["ReviewObjectRef"];
      comment_revision: components["schemas"]["ReviewObjectRef"] | null;
      event: components["schemas"]["ReviewObjectRef"];
    };
    ReviewCommandStatus:
      | {
          receipt: components["schemas"]["ReviewCommandReceipt"];
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
    CollabPage: {
      snapshot: components["schemas"]["CollabReadSnapshotV1"];
      items: components["schemas"]["CollabPageItems"];
      next_cursor: string | null;
    };
    CollabReadSnapshotV1: {
      /** Format: uint32 */
      version: number;
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      manifest_digest: components["schemas"]["RecordDigest"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      code_state_version: number;
      operation_heads: components["schemas"]["OperationId"][];
      thread_root: components["schemas"]["RecordDigest"] | null;
      /** Format: uint64 */
      thread_state_version: number;
      reaction_root: components["schemas"]["RecordDigest"] | null;
      /** Format: uint64 */
      reaction_state_version: number;
      agent_root: components["schemas"]["RecordDigest"] | null;
      review_root: components["schemas"]["RecordDigest"] | null;
    };
    CollabPageItems:
      | {
          threads: components["schemas"]["ThreadSummaryV1"][];
          /** @constant */
          type: "threads";
        }
      | {
          thread_id: components["schemas"]["ThreadId"];
          events: components["schemas"]["ThreadEventItemV1"][];
          /** @constant */
          type: "events";
        }
      | {
          target: components["schemas"]["ReactionTargetV1"];
          revision: components["schemas"]["TargetRevisionV1"];
          reactions: components["schemas"]["ReactionAggregateV1"][];
          /** Format: uint32 */
          conflict_count: number;
          actors: components["schemas"]["ReactionActorRowV1"][];
          /** @constant */
          type: "reactions";
        }
      | {
          versions: {
            [key: string]: components["schemas"]["CollabObjectRef"];
          };
          definitions: components["schemas"]["CustomEmojiVersionV1"][];
          /** @constant */
          type: "emojis";
        };
    ThreadSummaryV1: {
      reference: components["schemas"]["CollabObjectRef"];
      value: components["schemas"]["ThreadVersionV1"];
    };
    ThreadVersionV1: {
      /** Format: uint32 */
      schema_version: number;
      thread_id: components["schemas"]["ThreadId"];
      title: string;
      status: components["schemas"]["ThreadStatusV12"];
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      accepted_at_millis: number;
      previous: components["schemas"]["CollabObjectRef"] | null;
      event: components["schemas"]["CollabObjectRef"];
      baseline: components["schemas"]["ThreadCodeAnchorV1"] | null;
    };
    /** @enum {string} */
    ThreadStatusV12: "created" | "active" | "idle" | "archived" | "tombstoned";
    /** @description Native IDs and portable evidence remain distinct; neither is a device path. */
    ThreadCodeAnchorV1:
      | {
          /** @constant */
          profile: "native";
          anchor: components["schemas"]["CodeAnchorV1"];
        }
      | {
          /** @constant */
          profile: "portable";
          anchor: components["schemas"]["CodeAnchorV2"];
        };
    CodeAnchorV1: {
      operations: components["schemas"]["OperationId"][];
      commits: components["schemas"]["CommitId"][];
    };
    CodeAnchorV2: {
      operations: components["schemas"]["CodeEvidenceRefV1"][];
      commits: components["schemas"]["CodeCommitEvidenceV2"][];
    };
    CodeEvidenceRefV1: {
      repository: components["schemas"]["RepositoryId"];
      profile: components["schemas"]["CodeProfileV1"];
      operation: components["schemas"]["OperationId"];
      capsule: components["schemas"]["CodeCapsuleId"];
    };
    /** @enum {string} */
    CodeProfileV1: "native_v1" | "git_sha1_jj045";
    CodeCapsuleId: string;
    CodeCommitEvidenceV2: {
      evidence: components["schemas"]["CodeEvidenceRefV1"];
      commit: components["schemas"]["CodeCommitIdV1"];
    };
    CodeCommitIdV1:
      | {
          /** @constant */
          profile: "native_v1";
          id: components["schemas"]["CommitId"];
        }
      | {
          /** @constant */
          profile: "git_sha1_jj045";
          id: components["schemas"]["GitSha1Id"];
        };
    GitSha1Id: string;
    ThreadEventItemV1: {
      message: components["schemas"]["HumanMessageRevisionV1"] | null;
      reference: components["schemas"]["CollabObjectRef"];
      value: components["schemas"]["ThreadEventV1"];
    };
    HumanMessageRevisionV1: {
      /** Format: uint32 */
      schema_version: number;
      message_id: components["schemas"]["HumanMessageId"];
      thread_id: components["schemas"]["ThreadId"];
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      accepted_at_millis: number;
      previous: components["schemas"]["CollabObjectRef"] | null;
      reply_to?: components["schemas"]["CollabObjectRef"] | null;
      body: string | null;
    };
    ThreadEventV1: {
      /** Format: uint32 */
      schema_version: number;
      id: components["schemas"]["ThreadEventId"];
      thread_id: components["schemas"]["ThreadId"];
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      accepted_at_millis: number;
      previous: components["schemas"]["CollabObjectRef"] | null;
      event: components["schemas"]["ThreadEventKindV1"];
    };
    ThreadEventId: string;
    ThreadEventKindV1:
      | {
          /** @constant */
          type: "created";
        }
      | {
          previous_title: string;
          /** @constant */
          type: "renamed";
        }
      | {
          previous_status: components["schemas"]["ThreadStatusV12"];
          action: components["schemas"]["ThreadActionV1"];
          /** @constant */
          type: "lifecycle";
        }
      | {
          revision: components["schemas"]["CollabObjectRef"];
          /** @constant */
          type: "message";
        }
      | {
          version: components["schemas"]["CollabObjectRef"];
          /** @constant */
          type: "link";
        };
    /** @enum {string} */
    ThreadActionV1: "activate" | "idle" | "archive" | "tombstone" | "restore";
    /** @enum {string} */
    TargetRevisionV1: "current" | "earlier";
    ReactionAggregateV1: {
      value: components["schemas"]["ReactionValueV1"];
      /** Format: uint32 */
      count: number;
      viewer_reacted: boolean;
    };
    ReactionValueV1:
      | {
          /** @constant */
          type: "like";
        }
      | {
          /** @constant */
          type: "dislike";
        }
      | {
          sequence: string;
          /** @constant */
          type: "emoji";
        }
      | {
          emoji_id: components["schemas"]["CustomEmojiId"];
          version: components["schemas"]["CollabObjectRef"];
          /** @constant */
          type: "custom";
        };
    CustomEmojiId: string;
    ReactionActorRowV1: {
      actor: components["schemas"]["PrincipalId"];
      active: components["schemas"]["CollabObjectRef"][];
      value: components["schemas"]["ReactionValueV1"] | null;
      conflict: boolean;
    };
    CustomEmojiVersionV1: {
      /** Format: uint32 */
      schema_version: number;
      emoji_id: components["schemas"]["CustomEmojiId"];
      shortcode: string;
      label: string;
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      accepted_at_millis: number;
      previous: components["schemas"]["CollabObjectRef"] | null;
      asset: components["schemas"]["CollabObjectRef"];
      preview: components["schemas"]["CollabObjectRef"];
      retired: boolean;
    };
    ThreadDetailV1: {
      snapshot: components["schemas"]["CollabReadSnapshotV1"];
      thread: components["schemas"]["ThreadSummaryV1"];
    };
    HumanMessageDetailV1: {
      snapshot: components["schemas"]["CollabReadSnapshotV1"];
      target: components["schemas"]["ReactionTargetV1"];
      message: components["schemas"]["HumanMessageRevisionV1"];
    };
    CollabCommandOutcome:
      | {
          receipt: components["schemas"]["CollabCommandReceipt"];
          /** @constant */
          status: "committed";
        }
      | {
          request_id: components["schemas"]["RequestId"];
          /** @constant */
          status: "outcome_unknown";
        };
    CollabCommandReceipt: {
      /** Format: uint32 */
      version: number;
      request_id: components["schemas"]["RequestId"];
      publication_id: components["schemas"]["PublicationId"];
      request_digest: components["schemas"]["IntentDigest"];
      accepted: components["schemas"]["Snapshot"];
      actor: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      accepted_at_millis: number;
      result: components["schemas"]["CollabCommandResultV1"];
    };
    CollabCommandResultV1:
      | {
          result: components["schemas"]["ThreadCommandResultV1"];
          /** @constant */
          domain: "thread";
        }
      | {
          result: components["schemas"]["ReactionCommandResultV1"];
          /** @constant */
          domain: "reaction";
        }
      | {
          version: components["schemas"]["CollabObjectRef"];
          /** @constant */
          domain: "emoji";
        };
    ThreadCommandResultV1: {
      thread_id: components["schemas"]["ThreadId"];
      version: components["schemas"]["CollabObjectRef"];
      event: components["schemas"]["CollabObjectRef"];
      message: components["schemas"]["CollabObjectRef"] | null;
      link: components["schemas"]["CollabObjectRef"] | null;
      reassigned_from: components["schemas"]["ThreadId"] | null;
    };
    ReactionCommandResultV1: {
      /** Format: uint32 */
      schema_version: number;
      slot: components["schemas"]["CollabObjectRef"];
      active: components["schemas"]["CollabObjectRef"][];
      value: components["schemas"]["ReactionValueV1"] | null;
      conflict: boolean;
    };
    CollabCommandStatus:
      | {
          receipt: components["schemas"]["CollabCommandReceipt"];
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
    WorkBrowsePage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["WorkReadSnapshot"];
      projects: components["schemas"]["WorkProjectSummary"][];
      tasks: components["schemas"]["WorkTaskSummary"][];
      /** Format: uint */
      total_matches: number;
      next_cursor: string | null;
    };
    WorkReadSnapshot: {
      repo_id: components["schemas"]["RepositoryId"];
      epoch: components["schemas"]["EpochId"];
      published_seq: components["schemas"]["WalPosition"];
      /** Format: uint64 */
      manifest_revision: number;
      work_root: components["schemas"]["RecordDigest"] | null;
      /** Format: uint64 */
      work_state_version: number;
    };
    WorkProjectSummary: {
      id: components["schemas"]["ProjectId"];
      title: string;
      excerpt: string;
      /** Format: uint64 */
      version: number;
      repositories: {
        [key: string]: boolean;
      };
      task_counts: components["schemas"]["WorkTaskCounts"];
      actor: components["schemas"]["WorkSummaryActor"];
    };
    ProjectId: string;
    WorkTaskCounts: {
      /** Format: uint */
      active: number;
      /** Format: uint */
      done: number;
      /** Format: uint */
      total: number;
    };
    WorkSummaryActor: {
      created_by: components["schemas"]["PrincipalId"];
      updated_by: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      created_at_millis: number;
      /** Format: int64 */
      updated_at_millis: number;
    };
    WorkTaskSummary: {
      id: components["schemas"]["TaskId"];
      project: components["schemas"]["ProjectId"];
      project_title: string;
      title: string;
      state: components["schemas"]["TaskStateV1"];
      repository: components["schemas"]["RepositoryId"] | null;
      parent: components["schemas"]["TaskId"] | null;
      parent_title: string | null;
      /** Format: uint */
      comment_count: number;
      /** Format: uint */
      subtask_count: number;
      /** Format: uint64 */
      version: number;
      actor: components["schemas"]["WorkSummaryActor"];
    };
    TaskId: string;
    /** @enum {string} */
    TaskStateV1: "open" | "in_progress" | "blocked" | "done";
    WorkPage: {
      /** Format: uint32 */
      version: number;
      snapshot: components["schemas"]["WorkReadSnapshot"];
      projects: components["schemas"]["ProjectV1"][];
      tasks: components["schemas"]["TaskV1"][];
      comments: components["schemas"]["WorkCommentV1"][];
      next_cursor: string | null;
    };
    ProjectV1: {
      /** Format: uint32 */
      schema_version: number;
      id: components["schemas"]["ProjectId"];
      title: string;
      body: string;
      /** Format: uint64 */
      version: number;
      /** @description False preserves a retired membership and its historical task targets. */
      repositories: {
        [key: string]: boolean;
      };
      actor: components["schemas"]["WorkActorV1"];
    };
    WorkActorV1: {
      created_by: components["schemas"]["PrincipalId"];
      updated_by: components["schemas"]["PrincipalId"];
      /** Format: int64 */
      created_at_millis: number;
      /** Format: int64 */
      updated_at_millis: number;
    };
    TaskV1: {
      /** Format: uint32 */
      schema_version: number;
      id: components["schemas"]["TaskId"];
      project: components["schemas"]["ProjectId"];
      title: string;
      body: string;
      state: components["schemas"]["TaskStateV1"];
      repository: components["schemas"]["RepositoryId"] | null;
      parent: components["schemas"]["TaskId"] | null;
      /** Format: uint64 */
      version: number;
      actor: components["schemas"]["WorkActorV1"];
    };
    WorkCommentV1: {
      /** Format: uint32 */
      schema_version: number;
      id: components["schemas"]["WorkCommentId"];
      task: components["schemas"]["TaskId"];
      reply_to: components["schemas"]["WorkCommentId"] | null;
      /** @description None is an explicit tombstone; historical bodies stay in selected WAL roots. */
      body: string | null;
      /** Format: uint64 */
      version: number;
      actor: components["schemas"]["WorkActorV1"];
    };
    WorkCommentId: string;
    WorkCommandOutcome:
      | {
          receipt: components["schemas"]["WorkCommandReceipt"];
          /** @constant */
          status: "committed";
        }
      | {
          request_id: components["schemas"]["RequestId"];
          /** @constant */
          status: "outcome_unknown";
        };
    WorkCommandReceipt: {
      /** Format: uint32 */
      version: number;
      request_id: components["schemas"]["RequestId"];
      publication_id: components["schemas"]["PublicationId"];
      request_digest: components["schemas"]["IntentDigest"];
      accepted: components["schemas"]["Snapshot"];
      result: components["schemas"]["WorkCommandResultV1"];
    };
    WorkCommandResultV1: {
      project: components["schemas"]["ProjectV1"] | null;
      task: components["schemas"]["TaskV1"] | null;
      comment: components["schemas"]["WorkCommentV1"] | null;
    };
    WorkCommandStatus:
      | {
          receipt: components["schemas"]["WorkCommandReceipt"];
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
      remote: string | null;
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
    Window: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["Page"];
      previous_cursor: string | null;
      next_cursor: string | null;
    };
    Page: {
      /** Format: uint32 */
      version: number;
      operation: components["schemas"]["OperationId"];
      references: components["schemas"]["Reference"][];
      previous: string | null;
      next: string | null;
      stats: components["schemas"]["GraphStats"];
    };
    Reference:
      | {
          name: string;
          terms: (components["schemas"]["CommitId"] | null)[];
          conflicted: boolean;
          workspaces: string[];
          targets: components["schemas"]["Target"][];
          /** @constant */
          kind: "bookmark";
        }
      | {
          change: components["schemas"]["ChangeId"];
          divergent: boolean;
          targets: components["schemas"]["Target"][];
          /** @constant */
          kind: "change";
        };
    Target: {
      commit: components["schemas"]["CommitId"];
      change: components["schemas"]["ChangeId"];
      parents: components["schemas"]["CommitId"][];
      description: string;
      author: components["schemas"]["NativeSignature"];
      committer: components["schemas"]["NativeSignature"];
      conflicted: boolean;
      workspaces: string[];
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
    Window2: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["Page2"];
      previous_cursor: string | null;
      next_cursor: string | null;
    };
    Page2: {
      /** Format: uint32 */
      version: number;
      operation: components["schemas"]["OperationId"];
      parent: components["schemas"]["OperationId"];
      groups: components["schemas"]["Group"][];
      previous: string | null;
      next: string | null;
      stats: components["schemas"]["GraphStats"];
    };
    Group: {
      change: components["schemas"]["ChangeId"];
      before: components["schemas"]["CommitId"][];
      after: components["schemas"]["CommitId"][];
      comparison: components["schemas"]["Comparison"];
    };
    Comparison:
      | {
          from: components["schemas"]["CommitId"];
          to: components["schemas"]["CommitId"];
          /** @constant */
          status: "exact";
        }
      | {
          before: components["schemas"]["CommitId"][];
          after: components["schemas"]["CommitId"][];
          /** @constant */
          status: "select";
        }
      | {
          /** @constant */
          status: "unavailable";
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
    Window3: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["Page3"];
      previous_cursor: string | null;
      next_cursor: string | null;
    };
    Page3: {
      /** Format: uint32 */
      version: number;
      commit: components["schemas"]["CommitId"];
      path: string;
      recursive: boolean;
      entries: components["schemas"]["Entry"][];
      previous: string | null;
      next: string | null;
      stats: components["schemas"]["GraphStats"];
    };
    Entry: {
      path: string;
      terms: (components["schemas"]["NativeTreeValue"] | null)[];
      conflicted: boolean;
      last_change: components["schemas"]["LastChange"];
    };
    LastChange:
      | {
          commit: components["schemas"]["CommitId"];
          change: components["schemas"]["ChangeId"];
          description: string;
          author: components["schemas"]["NativeSignature"];
          committer: components["schemas"]["NativeSignature"];
          /** @constant */
          status: "available";
        }
      | {
          reason: components["schemas"]["LastChangeUnavailable"];
          /** @constant */
          status: "unavailable";
        };
    /** @enum {string} */
    LastChangeUnavailable: "merge" | "budget";
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
    Window4: {
      snapshot: components["schemas"]["ReadSnapshot"];
      data: components["schemas"]["Page4"];
      previous_cursor: string | null;
      next_cursor: string | null;
    };
    Page4: {
      /** Format: uint32 */
      version: number;
      from: components["schemas"]["CommitId"];
      to: components["schemas"]["CommitId"];
      path: string;
      changes: components["schemas"]["FileDiffChange"][];
      previous: string | null;
      next: string | null;
      stats: components["schemas"]["GraphStats"];
    };
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
      conflict_resolution?:
        components["schemas"]["ConflictResolutionReport"] | null;
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
    /** @description Complete remaining native paths on the target and every changed result. */
    ConflictResolutionReport: {
      target: components["schemas"]["Change3"];
      selected_paths: string[];
      remaining: {
        [key: string]: string[];
      };
    };
    Change3: {
      before: components["schemas"]["CommitId"];
      after: components["schemas"]["CommitId"];
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
    ConflictUpload: {
      /** Format: uint32 */
      version: number;
      id: components["schemas"]["ConflictUploadId"];
      file: components["schemas"]["FileId"];
      /** Format: uint64 */
      bytes: number;
      binding: components["schemas"]["Binding"];
    };
    ConflictUploadId: string;
    Binding: {
      expected: components["schemas"]["Snapshot"];
      commit: components["schemas"]["CommitId"];
      path: string;
      fingerprint: components["schemas"]["ConflictFingerprint"];
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
  bootstrap: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
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
          "application/json": components["schemas"]["Bootstrap"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  bootstrap_head: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  repositories: {
    parameters: {
      query?: {
        cursor?: string;
        limit?: number;
      };
      header?: never;
      path?: never;
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
          "application/json": components["schemas"]["RepositoryPage"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  repositories_head: {
    parameters: {
      query?: {
        cursor?: string;
        limit?: number;
      };
      header?: never;
      path?: never;
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  local_search_settings: {
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
          "application/json": components["schemas"]["LocalSearchSettings"];
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
      /** @description index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Search is not configured for this repository. */
      501: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description search_not_ready, busy, unavailable, corrupt_index, or incomplete history. */
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
  local_search_settings_head: {
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
      /** @description index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Search is not configured for this repository. */
      501: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description search_not_ready, busy, unavailable, corrupt_index, or incomplete history. */
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
  save_search_settings: {
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
          expected_revision: string;
          configuration: {
            enabled: boolean;
            index_source: boolean;
            scopes: {
              [key: string]:
                | {
                    name: string;
                    /** @constant */
                    kind: "bookmark";
                  }
                | {
                    name: string;
                    /** @constant */
                    kind: "workspace";
                  };
            };
            include_prefixes: string[];
            exclude_prefixes: string[];
            /** Format: uint64 */
            index_bytes: number;
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
          "application/json": components["schemas"]["LocalSearchSettings"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  control_search: {
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
          expected_revision: string;
          /** @enum {string} */
          action: "apply" | "retry" | "rebuild";
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
          "application/json": components["schemas"]["LocalSearchSettings"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  local_sync_status: {
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
          "application/json": components["schemas"]["LocalSyncStatus"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  local_sync_status_head: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  local_sync_control: {
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
          /** @enum {string} */
          action: "pause" | "resume" | "retry";
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
          "application/json": components["schemas"]["LocalSyncStatus"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  local_workspaces: {
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
          "application/json": components["schemas"]["WorkspaceLinks"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  local_workspaces_head: {
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  pick_workspace_folder: {
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
        "application/json": Record<string, never>;
      };
    };
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["FolderSelection"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  link_workspace: {
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
          /** Format: uint64 */
          expected_revision: number;
          path: string;
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
          "application/json": components["schemas"]["WorkspaceLinks"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  search: {
    parameters: {
      query: {
        q: string;
        mode?: "visible_commits" | "historical";
        at_op?: string;
        scope?: string;
        page_size?: number;
        minimum_position?: number;
        wait_millis?: number;
        cursor?: string;
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
          "application/json": components["schemas"]["SearchPage"];
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
      /** @description index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Search is not configured for this repository. */
      501: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description search_not_ready, busy, unavailable, corrupt_index, or incomplete history. */
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
  search_head: {
    parameters: {
      query: {
        q: string;
        mode?: "visible_commits" | "historical";
        at_op?: string;
        scope?: string;
        page_size?: number;
        minimum_position?: number;
        wait_millis?: number;
        cursor?: string;
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
      /** @description index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Search is not configured for this repository. */
      501: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description search_not_ready, busy, unavailable, corrupt_index, or incomplete history. */
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
  search_status: {
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
          "application/json": components["schemas"]["SearchStatus"];
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
      /** @description index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description Search is not configured for this repository. */
      501: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ErrorBody"];
        };
      };
      /** @description search_not_ready, busy, unavailable, corrupt_index, or incomplete history. */
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
  search_status_head: {
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
      /** @description index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Search is not configured for this repository. */
      501: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description search_not_ready, busy, unavailable, corrupt_index, or incomplete history. */
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
  agent_sessions: {
    parameters: {
      query?: {
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
          "application/json": components["schemas"]["AgentSessionPage"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_sessions_head: {
    parameters: {
      query?: {
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_session: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        session: components["schemas"]["AgentSessionId"];
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
          "application/json": components["schemas"]["AgentSessionDetail"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_session_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        session: components["schemas"]["AgentSessionId"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_transcript: {
    parameters: {
      query: {
        source: string;
        generation: number;
        cursor?: string;
        limit?: number;
        max_bytes: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        session: components["schemas"]["AgentSessionId"];
        transcript: components["schemas"]["AgentObjectId"];
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
          "application/json": components["schemas"]["AgentTranscriptPage"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_transcript_head: {
    parameters: {
      query: {
        source: string;
        generation: number;
        cursor?: string;
        limit?: number;
        max_bytes: number;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        session: components["schemas"]["AgentSessionId"];
        transcript: components["schemas"]["AgentObjectId"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_associations: {
    parameters: {
      query?: {
        session?: string;
        tool_session?: string;
        tool?: string;
        operation?: string;
        commit?: string;
        checkpoint?: string;
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
          "application/json": components["schemas"]["AgentRelationPage"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_associations_head: {
    parameters: {
      query?: {
        session?: string;
        tool_session?: string;
        tool?: string;
        operation?: string;
        commit?: string;
        checkpoint?: string;
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_checkpoints: {
    parameters: {
      query?: {
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
          "application/json": components["schemas"]["AgentCheckpointPage"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_checkpoints_head: {
    parameters: {
      query?: {
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_checkpoint: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        checkpoint: components["schemas"]["AgentCheckpointId"];
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
          "application/json": components["schemas"]["AgentCheckpointDetail"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_checkpoint_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        checkpoint: components["schemas"]["AgentCheckpointId"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_checkpoint_version: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        checkpoint: components["schemas"]["AgentCheckpointId"];
        version: components["schemas"]["AgentObjectId"];
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
          "application/json": components["schemas"]["AgentCheckpointDetail"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  agent_checkpoint_version_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        checkpoint: components["schemas"]["AgentCheckpointId"];
        version: components["schemas"]["AgentObjectId"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_threads: {
    parameters: {
      query?: {
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
          "application/json": components["schemas"]["ReviewThreadPage"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_threads_head: {
    parameters: {
      query?: {
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_thread: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread: components["schemas"]["ReviewThreadId"];
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
          "application/json": components["schemas"]["ReviewThreadDetail"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_thread_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread: components["schemas"]["ReviewThreadId"];
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
      /** @description cursor_reset after expiry, process restart, or repository epoch replacement. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_command_apply: {
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
          command:
            | {
                anchor: {
                  /** Format: uint32 */
                  schema_version: number;
                  repository_id: string;
                  target:
                    | {
                        /** @constant */
                        target: "code_text";
                        value: {
                          operation_id: string;
                          commit_id: string;
                          change_id?: string | null;
                          path: string;
                          file_id: string;
                          /** @enum {string} */
                          value_kind:
                            | "file"
                            | "binary_file"
                            | "empty_file"
                            | "symlink"
                            | "gitlink"
                            | "conflict_term"
                            | "conflict_artifact";
                          text: {
                            /** @enum {string} */
                            encoding: "utf8";
                            start_byte: string;
                            end_byte: string;
                            selected_sha256: string;
                          };
                          diff_context?: {
                            before_operation_id: string;
                            before_commit_id: string;
                            before_path?: string | null;
                            after_operation_id: string;
                            after_commit_id: string;
                            after_path?: string | null;
                            /** @enum {string} */
                            side: "before" | "after";
                            /** @enum {string} */
                            comparison: "explicit_pair";
                          } | null;
                        };
                      }
                    | {
                        /** @constant */
                        target: "code_artifact";
                        value: {
                          operation_id: string;
                          commit_id: string;
                          change_id?: string | null;
                          path: string;
                          object:
                            | {
                                /** @constant */
                                kind: "commit";
                                id: string;
                              }
                            | {
                                /** @constant */
                                kind: "tree";
                                id: string;
                              }
                            | {
                                /** @constant */
                                kind: "file";
                                id: string;
                              }
                            | {
                                /** @constant */
                                kind: "symlink";
                                id: string;
                              }
                            | {
                                /** @constant */
                                kind: "copy";
                                id: string;
                              }
                            | {
                                /** @constant */
                                kind: "view";
                                id: string;
                              }
                            | {
                                /** @constant */
                                kind: "operation";
                                id: string;
                              };
                          /** @enum {string} */
                          value_kind:
                            | "file"
                            | "binary_file"
                            | "empty_file"
                            | "symlink"
                            | "gitlink"
                            | "conflict_term"
                            | "conflict_artifact";
                        };
                      }
                    | {
                        /** @constant */
                        target: "agent_text";
                        value: {
                          session_id: string;
                          session_snapshot: {
                            /** @enum {string} */
                            kind:
                              | "index_page"
                              | "session_snapshot"
                              | "turn_snapshot"
                              | "message_snapshot"
                              | "tool_snapshot"
                              | "intent"
                              | "observation_batch"
                              | "event_batch"
                              | "transcript_manifest"
                              | "transcript_page"
                              | "transcript_chunk"
                              | "content"
                              | "checkpoint_version"
                              | "association"
                              | "alias"
                              | "relation_page"
                              | "source_state"
                              | "projection_catalog"
                              | "catalog_conflict";
                            id: string;
                          };
                          owner:
                            | {
                                tool_call_id: string;
                                snapshot: {
                                  /** @enum {string} */
                                  kind:
                                    | "index_page"
                                    | "session_snapshot"
                                    | "turn_snapshot"
                                    | "message_snapshot"
                                    | "tool_snapshot"
                                    | "intent"
                                    | "observation_batch"
                                    | "event_batch"
                                    | "transcript_manifest"
                                    | "transcript_page"
                                    | "transcript_chunk"
                                    | "content"
                                    | "checkpoint_version"
                                    | "association"
                                    | "alias"
                                    | "relation_page"
                                    | "source_state"
                                    | "projection_catalog"
                                    | "catalog_conflict";
                                  id: string;
                                };
                                attempt_id: string;
                                /** @constant */
                                kind: "tool";
                              }
                            | {
                                message_id: string;
                                snapshot: {
                                  /** @enum {string} */
                                  kind:
                                    | "index_page"
                                    | "session_snapshot"
                                    | "turn_snapshot"
                                    | "message_snapshot"
                                    | "tool_snapshot"
                                    | "intent"
                                    | "observation_batch"
                                    | "event_batch"
                                    | "transcript_manifest"
                                    | "transcript_page"
                                    | "transcript_chunk"
                                    | "content"
                                    | "checkpoint_version"
                                    | "association"
                                    | "alias"
                                    | "relation_page"
                                    | "source_state"
                                    | "projection_catalog"
                                    | "catalog_conflict";
                                  id: string;
                                };
                                /** @constant */
                                kind: "message";
                              }
                            | {
                                block_id: string;
                                snapshot: {
                                  /** @enum {string} */
                                  kind:
                                    | "index_page"
                                    | "session_snapshot"
                                    | "turn_snapshot"
                                    | "message_snapshot"
                                    | "tool_snapshot"
                                    | "intent"
                                    | "observation_batch"
                                    | "event_batch"
                                    | "transcript_manifest"
                                    | "transcript_page"
                                    | "transcript_chunk"
                                    | "content"
                                    | "checkpoint_version"
                                    | "association"
                                    | "alias"
                                    | "relation_page"
                                    | "source_state"
                                    | "projection_catalog"
                                    | "catalog_conflict";
                                  id: string;
                                };
                                /** @constant */
                                kind: "plan";
                              };
                          block_id: string;
                          block_version: {
                            /** @enum {string} */
                            kind:
                              | "index_page"
                              | "session_snapshot"
                              | "turn_snapshot"
                              | "message_snapshot"
                              | "tool_snapshot"
                              | "intent"
                              | "observation_batch"
                              | "event_batch"
                              | "transcript_manifest"
                              | "transcript_page"
                              | "transcript_chunk"
                              | "content"
                              | "checkpoint_version"
                              | "association"
                              | "alias"
                              | "relation_page"
                              | "source_state"
                              | "projection_catalog"
                              | "catalog_conflict";
                            id: string;
                          };
                          /** @enum {string} */
                          part:
                            | "input"
                            | "output"
                            | "message"
                            | "plan"
                            | "exposed_summary";
                          content: {
                            /** @enum {string} */
                            kind:
                              | "index_page"
                              | "session_snapshot"
                              | "turn_snapshot"
                              | "message_snapshot"
                              | "tool_snapshot"
                              | "intent"
                              | "observation_batch"
                              | "event_batch"
                              | "transcript_manifest"
                              | "transcript_page"
                              | "transcript_chunk"
                              | "content"
                              | "checkpoint_version"
                              | "association"
                              | "alias"
                              | "relation_page"
                              | "source_state"
                              | "projection_catalog"
                              | "catalog_conflict";
                            id: string;
                          };
                          representation_version: string;
                          sanitizer_version: string;
                          normalizer_version: string;
                          /** @enum {string} */
                          source_coverage:
                            | "complete"
                            | "partial"
                            | "redacted"
                            | "truncated"
                            | "unknown";
                          text: {
                            /** @enum {string} */
                            encoding: "utf8";
                            start_byte: string;
                            end_byte: string;
                            selected_sha256: string;
                          };
                        };
                      }
                    | {
                        /** @constant */
                        target: "agent_artifact";
                        value: {
                          session_id: string;
                          session_snapshot: {
                            /** @enum {string} */
                            kind:
                              | "index_page"
                              | "session_snapshot"
                              | "turn_snapshot"
                              | "message_snapshot"
                              | "tool_snapshot"
                              | "intent"
                              | "observation_batch"
                              | "event_batch"
                              | "transcript_manifest"
                              | "transcript_page"
                              | "transcript_chunk"
                              | "content"
                              | "checkpoint_version"
                              | "association"
                              | "alias"
                              | "relation_page"
                              | "source_state"
                              | "projection_catalog"
                              | "catalog_conflict";
                            id: string;
                          };
                          owner:
                            | {
                                tool_call_id: string;
                                snapshot: {
                                  /** @enum {string} */
                                  kind:
                                    | "index_page"
                                    | "session_snapshot"
                                    | "turn_snapshot"
                                    | "message_snapshot"
                                    | "tool_snapshot"
                                    | "intent"
                                    | "observation_batch"
                                    | "event_batch"
                                    | "transcript_manifest"
                                    | "transcript_page"
                                    | "transcript_chunk"
                                    | "content"
                                    | "checkpoint_version"
                                    | "association"
                                    | "alias"
                                    | "relation_page"
                                    | "source_state"
                                    | "projection_catalog"
                                    | "catalog_conflict";
                                  id: string;
                                };
                                attempt_id: string;
                                /** @constant */
                                kind: "tool";
                              }
                            | {
                                message_id: string;
                                snapshot: {
                                  /** @enum {string} */
                                  kind:
                                    | "index_page"
                                    | "session_snapshot"
                                    | "turn_snapshot"
                                    | "message_snapshot"
                                    | "tool_snapshot"
                                    | "intent"
                                    | "observation_batch"
                                    | "event_batch"
                                    | "transcript_manifest"
                                    | "transcript_page"
                                    | "transcript_chunk"
                                    | "content"
                                    | "checkpoint_version"
                                    | "association"
                                    | "alias"
                                    | "relation_page"
                                    | "source_state"
                                    | "projection_catalog"
                                    | "catalog_conflict";
                                  id: string;
                                };
                                /** @constant */
                                kind: "message";
                              }
                            | {
                                block_id: string;
                                snapshot: {
                                  /** @enum {string} */
                                  kind:
                                    | "index_page"
                                    | "session_snapshot"
                                    | "turn_snapshot"
                                    | "message_snapshot"
                                    | "tool_snapshot"
                                    | "intent"
                                    | "observation_batch"
                                    | "event_batch"
                                    | "transcript_manifest"
                                    | "transcript_page"
                                    | "transcript_chunk"
                                    | "content"
                                    | "checkpoint_version"
                                    | "association"
                                    | "alias"
                                    | "relation_page"
                                    | "source_state"
                                    | "projection_catalog"
                                    | "catalog_conflict";
                                  id: string;
                                };
                                /** @constant */
                                kind: "plan";
                              };
                        };
                      }
                    | {
                        /** @constant */
                        target: "revision";
                        value: {
                          operation_id: string;
                          commit_id: string;
                          change_id?: string | null;
                        };
                      }
                    | {
                        /** @constant */
                        target: "operation";
                        value: {
                          operation_id: string;
                        };
                      }
                    | {
                        /** @constant */
                        target: "repository";
                      };
                  created_context: {
                    /** Format: uint64 */
                    publication_position?: number | null;
                    /** Format: uint64 */
                    code_state_version?: number | null;
                    agent_root?: string | null;
                  };
                  /** @enum {string} */
                  sensitivity: "public" | "repository" | "restricted";
                };
                body: {
                  /** Format: uint32 */
                  schema_version: number;
                  /** @enum {string} */
                  format: "plain_text" | "safe_markdown";
                  text: string;
                  sanitizer_version: string;
                };
                /** @constant */
                command: "create_thread";
              }
            | {
                thread_id: string;
                /** Format: uint64 */
                expected_workflow_version: number;
                parent_comment_id?: string | null;
                body: {
                  /** Format: uint32 */
                  schema_version: number;
                  /** @enum {string} */
                  format: "plain_text" | "safe_markdown";
                  text: string;
                  sanitizer_version: string;
                };
                /** @constant */
                command: "append_reply";
              }
            | {
                comment_id: string;
                expected_revision_id: string;
                replacement_body: {
                  /** Format: uint32 */
                  schema_version: number;
                  /** @enum {string} */
                  format: "plain_text" | "safe_markdown";
                  text: string;
                  sanitizer_version: string;
                };
                /** @constant */
                command: "edit_comment";
              }
            | {
                comment_id: string;
                expected_revision_ids: string[];
                replacement_body: {
                  /** Format: uint32 */
                  schema_version: number;
                  /** @enum {string} */
                  format: "plain_text" | "safe_markdown";
                  text: string;
                  sanitizer_version: string;
                };
                /** @constant */
                command: "resolve_comment_conflict";
              }
            | {
                comment_id: string;
                expected_revision_id: string;
                reason?: string | null;
                /** @constant */
                command: "tombstone_comment";
              }
            | {
                comment_id: string;
                expected_revision_id: string;
                moderation_reason: string;
                /** @constant */
                command: "hide_comment";
              }
            | {
                thread_id: string;
                expected_thread_version: string;
                reason?: string | null;
                /** @constant */
                command: "resolve_thread";
              }
            | {
                thread_id: string;
                expected_thread_version: string;
                reason?: string | null;
                /** @constant */
                command: "reopen_thread";
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
          "application/json": components["schemas"]["ReviewCommandOutcome"];
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
      /** @description stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_command_status: {
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
          "application/json": components["schemas"]["ReviewCommandStatus"];
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
      /** @description stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  review_command_status_head: {
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
      /** @description stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_threads: {
    parameters: {
      query?: {
        limit?: number;
        cursor?: string;
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
          "application/json": components["schemas"]["CollabPage"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_threads_head: {
    parameters: {
      query?: {
        limit?: number;
        cursor?: string;
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_thread: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread_id: components["schemas"]["ThreadId"];
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
          "application/json": components["schemas"]["ThreadDetailV1"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_thread_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread_id: components["schemas"]["ThreadId"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_events: {
    parameters: {
      query?: {
        limit?: number;
        cursor?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread_id: components["schemas"]["ThreadId"];
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
          "application/json": components["schemas"]["CollabPage"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_events_head: {
    parameters: {
      query?: {
        limit?: number;
        cursor?: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread_id: components["schemas"]["ThreadId"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_message: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread_id: components["schemas"]["ThreadId"];
        message_id: components["schemas"]["HumanMessageId"];
        message_revision: components["schemas"]["CollabObjectId"];
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
          "application/json": components["schemas"]["HumanMessageDetailV1"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_message_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        thread_id: components["schemas"]["ThreadId"];
        message_id: components["schemas"]["HumanMessageId"];
        message_revision: components["schemas"]["CollabObjectId"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_thread_apply: {
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
          command:
            | {
                thread_id: string;
                title: string;
                baseline?:
                  | (
                      | {
                          /** @constant */
                          profile: "native";
                          anchor: {
                            operations: string[];
                            commits: string[];
                          };
                        }
                      | {
                          /** @constant */
                          profile: "portable";
                          anchor: {
                            operations: {
                              repository: string;
                              /** @enum {string} */
                              profile: "native_v1" | "git_sha1_jj045";
                              operation: string;
                              capsule: string;
                            }[];
                            commits: {
                              evidence: {
                                repository: string;
                                /** @enum {string} */
                                profile: "native_v1" | "git_sha1_jj045";
                                operation: string;
                                capsule: string;
                              };
                              commit:
                                | {
                                    /** @constant */
                                    profile: "native_v1";
                                    id: string;
                                  }
                                | {
                                    /** @constant */
                                    profile: "git_sha1_jj045";
                                    id: string;
                                  };
                            }[];
                          };
                        }
                    )
                  | null;
                /** @constant */
                type: "create";
              }
            | {
                thread_id: string;
                expected: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                title: string;
                /** @constant */
                type: "rename";
              }
            | {
                thread_id: string;
                expected: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                /** @enum {string} */
                action:
                  "activate" | "idle" | "archive" | "tombstone" | "restore";
                /** @constant */
                type: "lifecycle";
              }
            | {
                thread_id: string;
                expected_thread: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                message_id: string;
                expected_revision?: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                } | null;
                reply_to?: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                } | null;
                body?: string | null;
                /** @constant */
                type: "message";
              }
            | {
                thread_id: string;
                expected_thread: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                link_id: string;
                target:
                  | {
                      session_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      primary: boolean;
                      /** @constant */
                      type: "session";
                    }
                  | {
                      /** @description Native IDs and portable evidence remain distinct; neither is a device path. */
                      anchor:
                        | {
                            /** @constant */
                            profile: "native";
                            anchor: {
                              operations: string[];
                              commits: string[];
                            };
                          }
                        | {
                            /** @constant */
                            profile: "portable";
                            anchor: {
                              operations: {
                                repository: string;
                                /** @enum {string} */
                                profile: "native_v1" | "git_sha1_jj045";
                                operation: string;
                                capsule: string;
                              }[];
                              commits: {
                                evidence: {
                                  repository: string;
                                  /** @enum {string} */
                                  profile: "native_v1" | "git_sha1_jj045";
                                  operation: string;
                                  capsule: string;
                                };
                                commit:
                                  | {
                                      /** @constant */
                                      profile: "native_v1";
                                      id: string;
                                    }
                                  | {
                                      /** @constant */
                                      profile: "git_sha1_jj045";
                                      id: string;
                                    };
                              }[];
                            };
                          };
                      /** @enum {string} */
                      evidence_method:
                        | "controlled_scope"
                        | "hook_boundary"
                        | "transcript_recovery"
                        | "historical_import"
                        | "manual_assertion";
                      evidence?: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      } | null;
                      /** @constant */
                      type: "code";
                    }
                  | {
                      review_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "anchor"
                          | "comment_body"
                          | "comment_revision"
                          | "thread_event"
                          | "event_page"
                          | "thread_snapshot"
                          | "relation_page"
                          | "status_page";
                        id: string;
                      };
                      /** @constant */
                      type: "review";
                    };
                observed_primary?: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                } | null;
                /** @constant */
                type: "link";
              }
            | {
                thread_id: string;
                expected_thread: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                expected_link: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                /** @constant */
                type: "detach";
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
          "application/json": components["schemas"]["CollabCommandOutcome"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_status_threads: {
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
          "application/json": components["schemas"]["CollabCommandStatus"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_status_threads_head: {
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_reactions: {
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
          /** @description Identity, immutable content and validation witnesses are explicit, never a UI position. */
          target:
            | {
                thread_id: string;
                /** @constant */
                type: "thread";
              }
            | {
                thread_id: string;
                message_id: string;
                revision: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                };
                /** @constant */
                type: "human_message";
              }
            | {
                session_id: string;
                message_id: string;
                event_id: string;
                batch: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "session_snapshot"
                    | "turn_snapshot"
                    | "message_snapshot"
                    | "tool_snapshot"
                    | "intent"
                    | "observation_batch"
                    | "event_batch"
                    | "transcript_manifest"
                    | "transcript_page"
                    | "transcript_chunk"
                    | "content"
                    | "checkpoint_version"
                    | "association"
                    | "alias"
                    | "relation_page"
                    | "source_state"
                    | "projection_catalog"
                    | "catalog_conflict";
                  id: string;
                };
                content: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "session_snapshot"
                    | "turn_snapshot"
                    | "message_snapshot"
                    | "tool_snapshot"
                    | "intent"
                    | "observation_batch"
                    | "event_batch"
                    | "transcript_manifest"
                    | "transcript_page"
                    | "transcript_chunk"
                    | "content"
                    | "checkpoint_version"
                    | "association"
                    | "alias"
                    | "relation_page"
                    | "source_state"
                    | "projection_catalog"
                    | "catalog_conflict";
                  id: string;
                };
                /** @constant */
                type: "agent_message";
              }
            | {
                session_id: string;
                tool_id: string;
                snapshot: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "session_snapshot"
                    | "turn_snapshot"
                    | "message_snapshot"
                    | "tool_snapshot"
                    | "intent"
                    | "observation_batch"
                    | "event_batch"
                    | "transcript_manifest"
                    | "transcript_page"
                    | "transcript_chunk"
                    | "content"
                    | "checkpoint_version"
                    | "association"
                    | "alias"
                    | "relation_page"
                    | "source_state"
                    | "projection_catalog"
                    | "catalog_conflict";
                  id: string;
                };
                /** @constant */
                type: "tool_invocation";
              }
            | {
                session_id: string;
                tool_id: string;
                snapshot: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "session_snapshot"
                    | "turn_snapshot"
                    | "message_snapshot"
                    | "tool_snapshot"
                    | "intent"
                    | "observation_batch"
                    | "event_batch"
                    | "transcript_manifest"
                    | "transcript_page"
                    | "transcript_chunk"
                    | "content"
                    | "checkpoint_version"
                    | "association"
                    | "alias"
                    | "relation_page"
                    | "source_state"
                    | "projection_catalog"
                    | "catalog_conflict";
                  id: string;
                };
                content: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "session_snapshot"
                    | "turn_snapshot"
                    | "message_snapshot"
                    | "tool_snapshot"
                    | "intent"
                    | "observation_batch"
                    | "event_batch"
                    | "transcript_manifest"
                    | "transcript_page"
                    | "transcript_chunk"
                    | "content"
                    | "checkpoint_version"
                    | "association"
                    | "alias"
                    | "relation_page"
                    | "source_state"
                    | "projection_catalog"
                    | "catalog_conflict";
                  id: string;
                };
                /** @constant */
                type: "tool_output";
              }
            | {
                thread_id: string;
                /** @constant */
                type: "review_thread";
              }
            | {
                thread_id: string;
                comment_id: string;
                revision: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "anchor"
                    | "comment_body"
                    | "comment_revision"
                    | "thread_event"
                    | "event_page"
                    | "thread_snapshot"
                    | "relation_page"
                    | "status_page";
                  id: string;
                };
                /** @constant */
                type: "review_comment";
              }
            | {
                session_id: string;
                checkpoint_id: string;
                version: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "session_snapshot"
                    | "turn_snapshot"
                    | "message_snapshot"
                    | "tool_snapshot"
                    | "intent"
                    | "observation_batch"
                    | "event_batch"
                    | "transcript_manifest"
                    | "transcript_page"
                    | "transcript_chunk"
                    | "content"
                    | "checkpoint_version"
                    | "association"
                    | "alias"
                    | "relation_page"
                    | "source_state"
                    | "projection_catalog"
                    | "catalog_conflict";
                  id: string;
                };
                /** @constant */
                type: "agent_checkpoint";
              };
          /**
           * Format: uint32
           * @default 50
           */
          limit?: number;
          cursor?: string | null;
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
          "application/json": components["schemas"]["CollabPage"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_reaction_apply: {
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
          /** @description A typed reaction command never accepts an actor from an API caller. */
          command:
            | {
                /** @description Identity, immutable content and validation witnesses are explicit, never a UI position. */
                target:
                  | {
                      thread_id: string;
                      /** @constant */
                      type: "thread";
                    }
                  | {
                      thread_id: string;
                      message_id: string;
                      revision: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "thread_version"
                          | "thread_event"
                          | "human_message_revision"
                          | "thread_link"
                          | "reaction"
                          | "reaction_retraction"
                          | "reaction_slot"
                          | "custom_emoji_version"
                          | "image_asset";
                        id: string;
                      };
                      /** @constant */
                      type: "human_message";
                    }
                  | {
                      session_id: string;
                      message_id: string;
                      event_id: string;
                      batch: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      content: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "agent_message";
                    }
                  | {
                      session_id: string;
                      tool_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "tool_invocation";
                    }
                  | {
                      session_id: string;
                      tool_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      content: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "tool_output";
                    }
                  | {
                      thread_id: string;
                      /** @constant */
                      type: "review_thread";
                    }
                  | {
                      thread_id: string;
                      comment_id: string;
                      revision: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "anchor"
                          | "comment_body"
                          | "comment_revision"
                          | "thread_event"
                          | "event_page"
                          | "thread_snapshot"
                          | "relation_page"
                          | "status_page";
                        id: string;
                      };
                      /** @constant */
                      type: "review_comment";
                    }
                  | {
                      session_id: string;
                      checkpoint_id: string;
                      version: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "agent_checkpoint";
                    };
                value:
                  | {
                      /** @constant */
                      type: "like";
                    }
                  | {
                      /** @constant */
                      type: "dislike";
                    }
                  | {
                      sequence: string;
                      /** @constant */
                      type: "emoji";
                    }
                  | {
                      emoji_id: string;
                      version: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "thread_version"
                          | "thread_event"
                          | "human_message_revision"
                          | "thread_link"
                          | "reaction"
                          | "reaction_retraction"
                          | "reaction_slot"
                          | "custom_emoji_version"
                          | "image_asset";
                        id: string;
                      };
                      /** @constant */
                      type: "custom";
                    };
                observed: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                }[];
                /** @constant */
                type: "set";
              }
            | {
                /** @description Identity, immutable content and validation witnesses are explicit, never a UI position. */
                target:
                  | {
                      thread_id: string;
                      /** @constant */
                      type: "thread";
                    }
                  | {
                      thread_id: string;
                      message_id: string;
                      revision: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "thread_version"
                          | "thread_event"
                          | "human_message_revision"
                          | "thread_link"
                          | "reaction"
                          | "reaction_retraction"
                          | "reaction_slot"
                          | "custom_emoji_version"
                          | "image_asset";
                        id: string;
                      };
                      /** @constant */
                      type: "human_message";
                    }
                  | {
                      session_id: string;
                      message_id: string;
                      event_id: string;
                      batch: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      content: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "agent_message";
                    }
                  | {
                      session_id: string;
                      tool_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "tool_invocation";
                    }
                  | {
                      session_id: string;
                      tool_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      content: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "tool_output";
                    }
                  | {
                      thread_id: string;
                      /** @constant */
                      type: "review_thread";
                    }
                  | {
                      thread_id: string;
                      comment_id: string;
                      revision: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "anchor"
                          | "comment_body"
                          | "comment_revision"
                          | "thread_event"
                          | "event_page"
                          | "thread_snapshot"
                          | "relation_page"
                          | "status_page";
                        id: string;
                      };
                      /** @constant */
                      type: "review_comment";
                    }
                  | {
                      session_id: string;
                      checkpoint_id: string;
                      version: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "agent_checkpoint";
                    };
                observed: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                }[];
                /** @constant */
                type: "clear";
              }
            | {
                /** @description Identity, immutable content and validation witnesses are explicit, never a UI position. */
                target:
                  | {
                      thread_id: string;
                      /** @constant */
                      type: "thread";
                    }
                  | {
                      thread_id: string;
                      message_id: string;
                      revision: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "thread_version"
                          | "thread_event"
                          | "human_message_revision"
                          | "thread_link"
                          | "reaction"
                          | "reaction_retraction"
                          | "reaction_slot"
                          | "custom_emoji_version"
                          | "image_asset";
                        id: string;
                      };
                      /** @constant */
                      type: "human_message";
                    }
                  | {
                      session_id: string;
                      message_id: string;
                      event_id: string;
                      batch: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      content: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "agent_message";
                    }
                  | {
                      session_id: string;
                      tool_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "tool_invocation";
                    }
                  | {
                      session_id: string;
                      tool_id: string;
                      snapshot: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      content: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "tool_output";
                    }
                  | {
                      thread_id: string;
                      /** @constant */
                      type: "review_thread";
                    }
                  | {
                      thread_id: string;
                      comment_id: string;
                      revision: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "anchor"
                          | "comment_body"
                          | "comment_revision"
                          | "thread_event"
                          | "event_page"
                          | "thread_snapshot"
                          | "relation_page"
                          | "status_page";
                        id: string;
                      };
                      /** @constant */
                      type: "review_comment";
                    }
                  | {
                      session_id: string;
                      checkpoint_id: string;
                      version: {
                        /** @enum {string} */
                        kind:
                          | "index_page"
                          | "session_snapshot"
                          | "turn_snapshot"
                          | "message_snapshot"
                          | "tool_snapshot"
                          | "intent"
                          | "observation_batch"
                          | "event_batch"
                          | "transcript_manifest"
                          | "transcript_page"
                          | "transcript_chunk"
                          | "content"
                          | "checkpoint_version"
                          | "association"
                          | "alias"
                          | "relation_page"
                          | "source_state"
                          | "projection_catalog"
                          | "catalog_conflict";
                        id: string;
                      };
                      /** @constant */
                      type: "agent_checkpoint";
                    };
                value?:
                  | (
                      | {
                          /** @constant */
                          type: "like";
                        }
                      | {
                          /** @constant */
                          type: "dislike";
                        }
                      | {
                          sequence: string;
                          /** @constant */
                          type: "emoji";
                        }
                      | {
                          emoji_id: string;
                          version: {
                            /** @enum {string} */
                            kind:
                              | "index_page"
                              | "thread_version"
                              | "thread_event"
                              | "human_message_revision"
                              | "thread_link"
                              | "reaction"
                              | "reaction_retraction"
                              | "reaction_slot"
                              | "custom_emoji_version"
                              | "image_asset";
                            id: string;
                          };
                          /** @constant */
                          type: "custom";
                        }
                    )
                  | null;
                observed: {
                  /** @enum {string} */
                  kind:
                    | "index_page"
                    | "thread_version"
                    | "thread_event"
                    | "human_message_revision"
                    | "thread_link"
                    | "reaction"
                    | "reaction_retraction"
                    | "reaction_slot"
                    | "custom_emoji_version"
                    | "image_asset";
                  id: string;
                }[];
                /** @constant */
                type: "resolve_conflict";
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
          "application/json": components["schemas"]["CollabCommandOutcome"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_status_reactions: {
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
          "application/json": components["schemas"]["CollabCommandStatus"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_status_reactions_head: {
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_emojis: {
    parameters: {
      query?: {
        limit?: number;
        cursor?: string;
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
          "application/json": components["schemas"]["CollabPage"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_emojis_head: {
    parameters: {
      query?: {
        limit?: number;
        cursor?: string;
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_emoji_define: {
    parameters: {
      query: {
        version: number;
        request_id: string;
        expected?: string;
        shortcode: string;
        label: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        emoji_id: components["schemas"]["CustomEmojiId"];
      };
      cookie?: never;
    };
    /** @description At most 2097152 encoded bytes; only bounded PNG/JPEG/GIF/WebP are admitted. Static preview is retained by the service. */
    requestBody: {
      content: {
        "application/octet-stream": string;
      };
    };
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["CollabCommandOutcome"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_emoji_retire: {
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
          emoji_id: string;
          expected: string;
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
          "application/json": components["schemas"]["CollabCommandOutcome"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  "collab_status_reaction-emojis": {
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
          "application/json": components["schemas"]["CollabCommandStatus"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  "collab_status_reaction-emojis_head": {
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_image: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        asset: components["schemas"]["CollabObjectId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized retained raster. Source and static preview IDs are independent immutable assets. No byte ranges or external URLs; snapshot/content identity is in headers. */
      200: {
        headers: {
          "X-JJ-Store-Repository"?: components["schemas"]["RepositoryId"];
          "X-JJ-Store-Epoch"?: components["schemas"]["EpochId"];
          "X-JJ-Store-Position"?: number;
          "X-JJ-Store-Revision"?: components["schemas"]["RecordDigest"];
          "X-JJ-Store-Reaction-Asset"?: components["schemas"]["CollabObjectId"];
          [name: string]: unknown;
        };
        content: {
          "image/png": string;
          "image/jpeg": string;
          "image/gif": string;
          "image/webp": string;
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  collab_image_head: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
        asset: components["schemas"]["CollabObjectId"];
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Authorized retained raster. Source and static preview IDs are independent immutable assets. No byte ranges or external URLs; snapshot/content identity is in headers. */
      200: {
        headers: {
          "X-JJ-Store-Repository"?: components["schemas"]["RepositoryId"];
          "X-JJ-Store-Epoch"?: components["schemas"]["EpochId"];
          "X-JJ-Store-Position"?: number;
          "X-JJ-Store-Revision"?: components["schemas"]["RecordDigest"];
          "X-JJ-Store-Reaction-Asset"?: components["schemas"]["CollabObjectId"];
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
      /** @description cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_browse: {
    parameters: {
      query?: {
        kind?: "projects" | "tasks";
        project?: string;
        repository?: string;
        parent?: string;
        state?: "active" | "open" | "in_progress" | "blocked" | "done" | null;
        q?: string;
        limit?: number;
        cursor?: string;
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
          "application/json": components["schemas"]["WorkBrowsePage"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_browse_head: {
    parameters: {
      query?: {
        kind?: "projects" | "tasks";
        project?: string;
        repository?: string;
        parent?: string;
        state?: "active" | "open" | "in_progress" | "blocked" | "done" | null;
        q?: string;
        limit?: number;
        cursor?: string;
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_page: {
    parameters: {
      query?: {
        project?: string;
        task?: string;
        limit?: number;
        cursor?: string;
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
          "application/json": components["schemas"]["WorkPage"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_page_head: {
    parameters: {
      query?: {
        project?: string;
        task?: string;
        limit?: number;
        cursor?: string;
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_apply: {
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
          command:
            | {
                id: string;
                title: string;
                body: string;
                /** @constant */
                command: "create_project";
              }
            | {
                id: string;
                /** Format: uint64 */
                expected_version: number;
                title: string;
                body: string;
                /** @constant */
                command: "update_project";
              }
            | {
                project: string;
                /** Format: uint64 */
                expected_version: number;
                repository: string;
                active: boolean;
                /** @constant */
                command: "set_repository";
              }
            | {
                id: string;
                project: string;
                /** Format: uint64 */
                expected_project_version: number;
                title: string;
                body: string;
                repository?: string | null;
                parent?: string | null;
                /** @constant */
                command: "create_task";
              }
            | {
                id: string;
                /** Format: uint64 */
                expected_version: number;
                /** Format: uint64 */
                expected_project_version: number;
                title: string;
                body: string;
                /** @enum {string} */
                state: "open" | "in_progress" | "blocked" | "done";
                repository?: string | null;
                parent?: string | null;
                /** @constant */
                command: "update_task";
              }
            | {
                id: string;
                task: string;
                /** Format: uint64 */
                expected_task_version: number;
                reply_to?: string | null;
                body: string;
                /** @constant */
                command: "add_comment";
              }
            | {
                id: string;
                /** Format: uint64 */
                expected_version: number;
                /** Format: uint64 */
                expected_task_version: number;
                body: string;
                /** @constant */
                command: "edit_comment";
              }
            | {
                id: string;
                /** Format: uint64 */
                expected_version: number;
                /** Format: uint64 */
                expected_task_version: number;
                /** @constant */
                command: "tombstone_comment";
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
          "application/json": components["schemas"]["WorkCommandOutcome"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_status: {
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
          "application/json": components["schemas"]["WorkCommandStatus"];
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  work_status_head: {
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
      /** @description stale_work, cursor_reset, intent_conflict or work_replication_unsupported. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  references: {
    parameters: {
      query?: {
        at_op?: string;
        cursor?: string;
        q?: string;
        kind?: "bookmarks" | "changes" | "all";
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
          "application/json": components["schemas"]["Window"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  references_head: {
    parameters: {
      query?: {
        at_op?: string;
        cursor?: string;
        q?: string;
        kind?: "bookmarks" | "changes" | "all";
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  operation_changes: {
    parameters: {
      query: {
        parent: string;
        cursor?: string;
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
          "application/json": components["schemas"]["Window2"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  operation_changes_head: {
    parameters: {
      query: {
        parent: string;
        cursor?: string;
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  paths: {
    parameters: {
      query?: {
        at_op?: string;
        cursor?: string;
        path?: string;
        recursive?: boolean;
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
          "application/json": components["schemas"]["Window3"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  paths_head: {
    parameters: {
      query?: {
        at_op?: string;
        cursor?: string;
        path?: string;
        recursive?: boolean;
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  diff_paths: {
    parameters: {
      query: {
        at_op: string;
        from: string;
        to: string;
        path?: string;
        cursor?: string;
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
          "application/json": components["schemas"]["Window4"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  diff_paths_head: {
    parameters: {
      query: {
        at_op: string;
        from: string;
        to: string;
        path?: string;
        cursor?: string;
        limit?: number;
        anchor?: string;
        direction?: "forward" | "backward";
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
            /** Format: uint64 */
            code_state_version?: number;
            heads: string[];
          };
          command:
            | {
                bookmark: string;
                expected: (string | null)[];
                source: string;
                destination: string;
                contents: string;
                description: string;
                /** @constant */
                type: "edit_file";
              }
            | {
                commit: string;
                paths: {
                  path: string;
                  fingerprint: string;
                  resolution:
                    | {
                        upload: string;
                        executable: boolean;
                        /** Format: uint32 */
                        copy_from_term?: number | null;
                        /** @constant */
                        type: "uploaded_file";
                      }
                    | {
                        /** Format: uint32 */
                        term: number;
                        /** @constant */
                        type: "select_term";
                      }
                    | {
                        /** @constant */
                        type: "delete";
                      };
                }[];
                /** @constant */
                type: "resolve_conflicts";
              }
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
      /** @description stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
            /** Format: uint64 */
            code_state_version?: number;
            heads: string[];
          };
          command:
            | {
                bookmark: string;
                expected: (string | null)[];
                source: string;
                destination: string;
                contents: string;
                description: string;
                /** @constant */
                type: "edit_file";
              }
            | {
                commit: string;
                paths: {
                  path: string;
                  fingerprint: string;
                  resolution:
                    | {
                        upload: string;
                        executable: boolean;
                        /** Format: uint32 */
                        copy_from_term?: number | null;
                        /** @constant */
                        type: "uploaded_file";
                      }
                    | {
                        /** Format: uint32 */
                        term: number;
                        /** @constant */
                        type: "select_term";
                      }
                    | {
                        /** @constant */
                        type: "delete";
                      };
                }[];
                /** @constant */
                type: "resolve_conflicts";
              }
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
      /** @description stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
  conflict_upload: {
    parameters: {
      query: {
        epoch: string;
        position: number;
        code_state_version: number;
        operation: string;
        commit: string;
        path: string;
        fingerprint: string;
      };
      header?: never;
      path: {
        repo: components["schemas"]["RepositoryId"];
      };
      cookie?: never;
    };
    requestBody: {
      content: {
        "application/octet-stream": string;
      };
    };
    responses: {
      /** @description Authorized result at the reported snapshot. */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["ConflictUpload"];
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
      /** @description stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt. */
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
      /** @description Unsupported request media type or content encoding. */
      415: {
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
