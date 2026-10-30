import { attest, verify } from "sigstore";
import { join } from "node:path";
import {
  loadArtifact,
  expectedArtifact,
  repository,
  workflow,
  packageName,
  writeJson,
} from "./conversations-npm-artifact.mjs";

const issuer = "https://token.actions.githubusercontent.com";
const identity = `https://github.com/${repository}/${workflow}@refs/heads/main`;
export function provenanceStatement(manifest, env = process.env) {
  if (
    env.GITHUB_REPOSITORY !== repository ||
    env.GITHUB_REF !== "refs/heads/main" ||
    env.GITHUB_SHA !== manifest.source ||
    env.GITHUB_WORKFLOW_REF !== `${repository}/${workflow}@refs/heads/main` ||
    env.GITHUB_EVENT_NAME !== "workflow_dispatch" ||
    env.GITHUB_REPOSITORY_ID !== "1419706251" ||
    env.GITHUB_REPOSITORY_OWNER_ID !== "46745434" ||
    env.RUNNER_ENVIRONMENT !== "github-hosted" ||
    !/^\d+$/.test(env.GITHUB_RUN_ID ?? "") ||
    !/^\d+$/.test(env.GITHUB_RUN_ATTEMPT ?? "")
  )
    throw Error(
      "Detached provenance requires the exact qualified main workflow",
    );
  return {
    _type: "https://in-toto.io/Statement/v1",
    subject: [
      {
        name: `pkg:npm/${packageName.replace("@", "%40")}@${manifest.version}`,
        digest: {
          sha512: Buffer.from(manifest.integrity.slice(7), "base64").toString(
            "hex",
          ),
        },
      },
    ],
    predicateType: "https://slsa.dev/provenance/v1",
    predicate: {
      buildDefinition: {
        buildType:
          "https://slsa-framework.github.io/github-actions-buildtypes/workflow/v1",
        externalParameters: {
          workflow: {
            repository: `https://github.com/${repository}`,
            ref: "refs/heads/main",
            path: workflow,
          },
        },
        internalParameters: {
          github: {
            event_name: env.GITHUB_EVENT_NAME,
            repository_id: env.GITHUB_REPOSITORY_ID,
            repository_owner_id: env.GITHUB_REPOSITORY_OWNER_ID,
          },
        },
        resolvedDependencies: [
          {
            uri: `git+https://github.com/${repository}@refs/heads/main`,
            digest: { gitCommit: manifest.source },
          },
        ],
      },
      runDetails: {
        builder: { id: "https://github.com/actions/runner/github-hosted" },
        metadata: {
          invocationId: `https://github.com/${repository}/actions/runs/${env.GITHUB_RUN_ID}/attempts/${env.GITHUB_RUN_ATTEMPT}`,
        },
      },
    },
  };
}

export async function verifySigner(bundle, check = verify) {
  const signer = await check(bundle, {
    certificateIssuer: issuer,
    certificateIdentityURI: `^${identity.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`,
  });
  if (
    signer?.identity?.extensions?.issuer !== issuer ||
    signer.identity.subjectAlternativeName !== identity
  )
    throw Error(
      "Conversations provenance signer does not match its qualified workflow",
    );
  return signer;
}

export async function prepareProvenance(
  root = process.cwd(),
  { env = process.env, sign = attest, check = verify } = {},
) {
  const { manifest, directory } = loadArtifact(root, expectedArtifact(env));
  const statement = provenanceStatement(manifest, env);
  const bundle = await sign(
    Buffer.from(JSON.stringify(statement)),
    "application/vnd.in-toto+json",
  );
  const payload = JSON.parse(
    Buffer.from(bundle?.dsseEnvelope?.payload ?? "", "base64").toString("utf8"),
  );
  if (JSON.stringify(payload) !== JSON.stringify(statement))
    throw Error("Provenance signer returned a different statement");
  await verifySigner(bundle, check);
  writeJson(join(directory, "provenance.sigstore.json"), bundle);
  writeJson(join(directory, "provenance.statement.json"), statement);
  console.log(
    "Verified detached Conversations provenance for the exact qualified artifact",
  );
}
