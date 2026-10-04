<script lang="ts">
  import type { Model } from "../../api/client.js";
  import { captureProfiles } from "../../controller/capture-types.js";
  let { value }: { value: Model<"CaptureStatus"> } = $props();
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-locations-heading"
>
  <header><h2 id="capture-locations-heading">Harness locations</h2></header>
  <dl class="store-settings__rows">
    {#each value.runtime?.harnesses ?? [] as harness (harness.profile)}
      <div>
        <dt>
          {captureProfiles[harness.profile]}
          <p class="store-settings__row-description">
            {harness.product_version} · Configured root directories
          </p>
        </dt>
        <dd class="store-settings__stack">
          {#if harness.configured_roots}
            {#each harness.configured_roots as path (path)}<code>{path}</code
              >{:else}<span
                >No directory roots configured for this harness.</span
              >{/each}
          {:else}<span
              >Configured directory paths are not reported by this backend.</span
            >{/if}
        </dd>
      </div>
    {:else}<div>
        <dt>Configured root directories</dt>
        <dd>
          Harness configuration is unavailable while the worker is unavailable.
        </dd>
      </div>{/each}
  </dl>
</section>
