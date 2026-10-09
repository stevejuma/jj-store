import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";
import type { WorkspaceFileSource } from "./workspace-types.js";
import type { ChatBodyBlock } from "../ui/store-agent-chat-body/model.js";
import type {
  WorkspaceBrowser,
  WorkspaceRead,
} from "./workspace-browser.svelte.js";
/** Exact retained sources are independent of the cache of current observations. */
export function registerWorkspaceSource(
  browser: WorkspaceBrowser,
  value: WorkspaceRead,
  reveal: () => void,
) {
  const file = value.file;
  if (!file || file.unavailable) {
    browser.blocks = {};
    return;
  }
  const blocks: { before?: ChatBodyBlock; after?: ChatBodyBlock } = {};
  for (const side of ["before", "after"] as const) {
    const contents = side === "before" ? file.before : file.contents;
    if (contents === null) continue;
    const comparison = value.comparison,
      before = side === "before";
    const operation = before
      ? comparison?.before_operation
      : (comparison?.after_operation ?? value.operation);
    const commit = before
      ? comparison?.before_commit
      : (comparison?.after_commit ?? value.commit);
    if (!operation || !commit) continue;
    const current = !before && (!comparison || comparison.current_files);
    const hash = bytesToHex(sha256(new TextEncoder().encode(contents)));
    const revision = `${operation}:${commit}:${current ? file.fingerprint : hash}`;
    const block: ChatBodyBlock = {
      id: `workspace:${value.session}:${value.repository}:${file.path}:${side}:${revision}`,
      revision,
      kind: "code",
      label: file.path,
      path: file.path,
      text: contents,
    };
    blocks[side] = block;
    const source: WorkspaceFileSource = {
      session: value.session,
      repository: value.repository,
      workspace: value.workspace,
      path: file.path,
      operation,
      commit,
      content_sha256: hash,
      current_fingerprint: current ? file.fingerprint : null,
      selection: null,
      diff:
        comparison?.before_operation && comparison.before_commit
          ? {
              before_operation: comparison.before_operation,
              before_commit: comparison.before_commit,
              after_operation: comparison.after_operation,
              after_commit: comparison.after_commit,
              side,
            }
          : null,
    };
    browser.conversation.workspaceSources.set(block.id, {
      block,
      source,
      reveal: () => {
        reveal();
        browser.file = value;
        browser.blocks = blocks;
        browser.presentation = side;
        browser.fileOpen = true;
        browser.inspect();
      },
    });
  }
  browser.blocks = blocks;
}
