import * as fixture from "../fixtures/repository.js";
import {
  largeBookmarks,
  largeChoices,
  largeRevisions,
} from "../fixtures/large-repository.js";
import { repositoryChoices } from "../fixtures/snapshots.js";
import type {
  StoreMutationCommand,
  StoreMutationPrepared,
} from "../../web/src/lib/controller/store-mutation-types.js";
import { commitInMemoryMutation } from "./in-memory-store-mutation-commit.js";

export interface InMemoryMutationRouteChange {
  replacementCommit: string | null;
  createdCommit: string | null;
  deletedBookmark: string | null;
}

export function createInMemoryStoreRepository(large = false) {
  let bookmarks = $state([...(large ? largeBookmarks : fixture.bookmarks)]);
  let revisions = $state([...(large ? largeRevisions : fixture.revisions)]);
  let operationHeads = $state([fixture.IDS.opLeft, fixture.IDS.opRight]);
  const choices = $derived(
    large ? largeChoices : repositoryChoices(bookmarks, revisions),
  );

  function commit(
    command: StoreMutationCommand,
    result: StoreMutationPrepared,
    selectedCommit: string | null,
  ): InMemoryMutationRouteChange {
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
        command.type === "new" ? (result.created[0] ?? null) : null,
      deletedBookmark: command.type === "bookmark_delete" ? command.name : null,
    };
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
    commit,
  };
}
