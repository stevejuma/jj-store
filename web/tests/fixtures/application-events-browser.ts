import { sharedApplicationEvents } from "../../src/lib/api/shared-application-events.js";
const owners: AbortController[] = [];
let deliveries = 0;
let ready = 0;
const output = document.querySelector("output")!;
function render() {
  output.textContent = JSON.stringify({
    owners: owners.length,
    ready,
    deliveries,
  });
}
function join() {
  const owner = new AbortController();
  owners.push(owner);
  let first = true;
  void sharedApplicationEvents(null, undefined, owner.signal, () => {
    if (first) {
      first = false;
      ready++;
    }
    deliveries++;
    render();
  }).catch((error) => {
    if (!owner.signal.aborted) output.textContent = String(error);
  });
  render();
}
document.querySelector("#add")!.addEventListener("click", () => {
  for (let i = 0; i < 7; i++) join();
});
document.querySelector("#release")!.addEventListener("click", () => {
  for (const owner of owners.splice(0)) owner.abort();
  render();
});
document.querySelector("#change")!.addEventListener("click", () => {
  void fetch("/change", { method: "POST" });
});
document.querySelector("#probe")!.addEventListener("click", async () => {
  const start = performance.now();
  const state = await (await fetch("/probe")).json();
  document.querySelector("pre")!.textContent = JSON.stringify({
    ...state,
    requestMs: performance.now() - start,
  });
});
join();
