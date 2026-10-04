import { resolve, isAbsolute } from "node:path";
import { createServer } from "vite";

// The exact runtime-root argument is also used to verify process ownership.
if (process.argv[2] !== "--runtime-root" || !isAbsolute(process.argv[3] ?? ""))
  throw new Error(
    "Vite development runner requires an absolute --runtime-root",
  );
const repositoryRoot = resolve(import.meta.dirname, "..");
const server = await createServer({
  configFile: resolve(repositoryRoot, "web/vite.config.ts"),
});
let closing = false;
async function stop() {
  if (closing) return;
  closing = true;
  await server.close();
}
process.on("SIGTERM", () => void stop());
process.on("SIGINT", () => void stop());
await server.listen();
server.printUrls();
