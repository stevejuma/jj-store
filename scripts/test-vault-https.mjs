#!/usr/bin/env node
// Trust installation is confined to the disposable validation container.
import { spawn, execFileSync } from "node:child_process";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  copyFileSync,
  rmSync,
  realpathSync,
} from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { randomUUID } from "node:crypto";
import { httpsFixture } from "./vault-https-fixture.mjs";
if (
  process.platform !== "linux" ||
  process.getuid() !== 0 ||
  process.env.JJ_STORE_VALIDATION_WORKER !== "1"
)
  throw Error("HTTPS fixture requires the disposable Linux validation worker");
if (process.argv.length > 2) throw Error("HTTPS fixture takes no arguments");
const base = realpathSync(mkdtempSync(join(homedir(), ".jj-vault-https-")));
const certificate =
  "/usr/local/share/ca-certificates/lapis-disposable-" + randomUUID() + ".crt";
const environment = {
  ...process.env,
  HOME: join(base, "home"),
  JJ_CONFIG: "/dev/null",
  GIT_CONFIG_GLOBAL: "/dev/null",
  GIT_CONFIG_NOSYSTEM: "1",
};
const run = (bin, args) =>
  execFileSync(bin, args, {
    env: environment,
    stdio: ["ignore", "pipe", "pipe"],
  });
let server;
try {
  mkdirSync(environment.HOME, { mode: 0o700 });
  mkdirSync(join(base, "exports"));
  for (const name of ["token", "anonymous"])
    run("git", [
      "init",
      "--quiet",
      "--bare",
      "--initial-branch=main",
      join(base, "exports", name + ".git"),
    ]);
  run("openssl", [
    "req",
    "-x509",
    "-newkey",
    "rsa:2048",
    "-sha256",
    "-nodes",
    "-days",
    "1",
    "-subj",
    "/CN=Lapis disposable TLS fixture",
    "-addext",
    "basicConstraints=critical,CA:TRUE",
    "-keyout",
    join(base, "ca.key"),
    "-out",
    join(base, "ca.crt"),
  ]);
  run("openssl", [
    "req",
    "-newkey",
    "rsa:2048",
    "-nodes",
    "-subj",
    "/CN=localhost",
    "-keyout",
    join(base, "server.key"),
    "-out",
    join(base, "server.csr"),
  ]);
  writeFileSync(
    join(base, "extensions"),
    "subjectAltName=IP:127.0.0.1,DNS:localhost\nextendedKeyUsage=serverAuth\n",
  );
  run("openssl", [
    "x509",
    "-req",
    "-in",
    join(base, "server.csr"),
    "-CA",
    join(base, "ca.crt"),
    "-CAkey",
    join(base, "ca.key"),
    "-CAcreateserial",
    "-days",
    "1",
    "-sha256",
    "-extfile",
    join(base, "extensions"),
    "-out",
    join(base, "server.crt"),
  ]);
  copyFileSync(join(base, "ca.crt"), certificate);
  run("update-ca-certificates", []);
  writeFileSync(join(base, "active-token"), "first-synthetic-token", {
    mode: 0o600,
  });
  server = await httpsFixture(base, environment);
  const child = spawn(
    process.execPath,
    [
      "scripts/managed-cargo.mjs",
      "test",
      "--locked",
      "-p",
      "jj-store-jj",
      "--no-default-features",
      "--features",
      "vault-backup",
      "--test",
      "vault_https_transport",
      "--",
      "--ignored",
      "--exact",
      "real_https_tokens_anonymous_reads_and_original_publication_preserve_scope",
    ],
    {
      env: {
        ...environment,
        RUSTUP_HOME: process.env.RUSTUP_HOME ?? "/usr/local/rustup",
        CARGO_HOME: process.env.CARGO_HOME ?? "/usr/local/cargo",
        JJ_STORE_HTTPS_FIXTURE: base,
        JJ_STORE_HTTPS_ENDPOINT: server.endpoint,
      },
      stdio: "inherit",
    },
  );
  const code = await new Promise((resolve) =>
    child.once("exit", (code) => resolve(code ?? 1)),
  );
  if (code !== 0) throw Error("Real HTTPS acceptance failed");
  console.log(
    "Real HTTPS acceptance passed; disposable trust, credentials and endpoints retired.",
  );
} finally {
  await server?.close();
  rmSync(certificate, { force: true });
  run("update-ca-certificates", []);
  rmSync(base, { recursive: true, force: true });
}
