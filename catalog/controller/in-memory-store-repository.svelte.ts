import * as fixture from "../fixtures/repository.js";
import {
  largeBookmarks,
  largeChoices,
  largeRevisions,
} from "../fixtures/large-repository.js";
import {
  repositoryChoices,
  repositorySnapshot,
} from "../fixtures/snapshots.js";
import type {
  StoreMutationCommand,
  StoreMutationPrepared,
} from "../../web/src/lib/controller/store-mutation-types.js";
import { commitInMemoryMutation } from "./in-memory-store-mutation-commit.js";
import type { RepositorySnapshot } from "../../web/src/lib/controller/contracts.js";

export interface InMemoryMutationRouteChange {
  replacementCommit: string | null;
  createdCommit: string | null;
  createdPath: string | null;
  deletedBookmark: string | null;
}

export function createInMemoryStoreRepository(large = false) {
  let bookmarks = $state([...(large ? largeBookmarks : fixture.bookmarks)]);
  let revisions = $state([...(large ? largeRevisions : fixture.revisions)]);
  let operationHeads = $state([fixture.IDS.opLeft, fixture.IDS.opRight]);
  const editedSnapshots = new Map<string, RepositorySnapshot>();
  const choices = $derived(
    large ? largeChoices : repositoryChoices(bookmarks, revisions),
  );

  function commit(
    command: StoreMutationCommand,
    result: StoreMutationPrepared,
    repository: string,
    selectedCommit: string | null,
  ): InMemoryMutationRouteChange {
    if (command.type === "edit_file" && result.created[0]) {
      const source =
        editedSnapshots.get(command.expected[0] ?? "") ??
        repositorySnapshot(repository, command.expected[0] ?? "");
      editedSnapshots.set(result.created[0], {
        ...source,
        id: result.created[0],
        files: [
          ...source.files.filter(
            (file) =>
              file.path !== command.source && file.path !== command.destination,
          ),
          { path: command.destination, contents: command.contents },
        ],
      });
    }
    const committed = commitInMemoryMutation(
      { bookmarks, revisions, operationHeads },
      command,
      result,
    );
    bookmarks = committed.bookmarks;
    revisions = committed.revisions;
    operationHeads = committed.operationHeads;
    return {
      replacementCommit:
        result.rewrites.find((rewrite) =>
          rewrite.predecessors.includes(selectedCommit ?? ""),
        )?.commit ?? null,
      createdCommit:
        command.type === "new" || command.type === "edit_file"
          ? (result.created[0] ?? null)
          : null,
      createdPath: command.type === "edit_file" ? command.destination : null,
      deletedBookmark: command.type === "bookmark_delete" ? command.name : null,
    };
  }

  function snapshot(repository: string, id: string) {
    return editedSnapshots.get(id) ?? repositorySnapshot(repository, id);
  }

  return {
    get bookmarks() {
      return bookmarks;
    },
    get revisions() {
      return revisions;
    },
    get operationHeads() {
      return operationHeads;
    },
    get choices() {
      return choices;
    },
    snapshot,
    commit,
  };
}
