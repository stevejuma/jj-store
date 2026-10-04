import { fixtureCommit, fixtureFile } from "./data.js";
/** Large, newest-first history, isolated from mutable stack topology. */
export function fixtureTargetHistory(branch = "main", count = 240) {
  return Array.from({ length: count }, (_, index) => {
    const number = count - index;
    const commit = fixtureCommit(
      `${branch}-history-${number}`,
      `${branch === "main" ? "Main" : branch} history ${number}`,
      [fixtureFile(`src/history/entry-${number}.ts`, ["update history entry"])],
    );
    commit.immutable = true;
    commit.createdAt -= index * 60;
    return commit;
  });
}
