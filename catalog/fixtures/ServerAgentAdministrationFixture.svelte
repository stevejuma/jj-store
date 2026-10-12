<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreServerAgentAdministration from "../../web/src/lib/ui/store-agent-execution/StoreServerAgentAdministration.svelte";
  import { ServerAgentAdministration } from "../../web/src/lib/controller/server-agent-administration.svelte.js";
  import {
    newAgentDefinition,
    type ServerAgentReport,
  } from "../../web/src/lib/controller/server-agent-administration-types.js";
  let {
    identityOperator = false,
    loseSave = false,
    discoveryPending = false,
  }: {
    identityOperator?: boolean;
    loseSave?: boolean;
    discoveryPending?: boolean;
  } = $props();
  const principal = "11".repeat(16),
    other = "22".repeat(16),
    repository = "33".repeat(16);
  const definition = newAgentDefinition();
  definition.setup.id = "builder";
  definition.setup.name = "Builder";
  definition.principals = [principal];
  let report: ServerAgentReport = {
    revision: "aa".repeat(32),
    desired: {
      acp_executable: "/opt/jj-store/bin/jj-acp",
      jj_executable: "/opt/jj-store/bin/jj",
      agents: [definition],
    },
    effective: null,
    restart_required: true,
    principals: [principal, other],
  };
  let saves = $state(0),
    enrollments = $state(0),
    grants = $state(0);
  const manager = new ServerAgentAdministration(
    () => true,
    () => "synthetic-session",
    async (input, init) => {
      const path = String(input),
        body = init?.body ? JSON.parse(String(init.body)) : undefined;
      const json = (value: unknown) =>
        new Response(JSON.stringify(value), {
          headers: { "content-type": "application/json" },
        });
      if (path === "/v1/operator/access")
        return json({
          agents: !identityOperator,
          identities: identityOperator,
        });
      if (path === "/v1/operator/agents") {
        if (body) {
          saves++;
          report = {
            ...report,
            revision: "bb".repeat(32),
            desired: body.configuration,
          };
          if (loseSave) {
            loseSave = false;
            throw Error("lost response");
          }
        }
        return json(report);
      }
      if (path.startsWith("/v1/directory/profiles"))
        return json([
          { principal, display_name: "Maya Chen" },
          { principal: other, display_name: "Noah Patel" },
        ]);
      if (path === "/v1/directory/identities") {
        if (body) {
          enrollments++;
          return json({ request_id: body.request_id });
        }
        return json([]);
      }
      if (path.endsWith("/agent-access")) {
        if (body) {
          grants++;
          return json({ request_id: body.request_id, published: true });
        }
        return json({
          epoch: "44".repeat(16),
          position: 0,
          code_state_version: 0,
          heads: [],
        });
      }
      throw Error(`Unexpected fixture path ${path}`);
    },
  );
  manager.access = untrack(() =>
    discoveryPending
      ? null
      : {
          agents: !identityOperator,
          identities: identityOperator,
        },
  );
  onDestroy(() => manager.destroy());
</script>

<Button onclick={() => (manager.open = true)}>Manage server access</Button>
<output aria-label="Configuration save count">{saves}</output>
<output aria-label="Enrollment count">{enrollments}</output>
<output aria-label="Write grant count">{grants}</output>
<StoreServerAgentAdministration
  {manager}
  repositories={[
    {
      id: repository,
      name: "Code repository",
      permission: "admin",
      profile: "native",
    },
    {
      id: "66".repeat(16),
      name: "Local repository",
      permission: "admin",
      profile: "git_sha1_jj045",
    },
    { id: "55".repeat(16), name: "Read only repository", permission: "read" },
  ]}
/>
