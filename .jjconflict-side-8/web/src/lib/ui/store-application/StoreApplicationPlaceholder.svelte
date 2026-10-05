<script lang="ts">
  import { repositorySections } from "./repository-sections.js";
  import type { StorePage } from "../community-navigation/model.js";
  let { repository, page }: { repository: string; page: StorePage } = $props();
</script>

<p class="store-application__placeholder">
  {repository} · {repositorySections.find((section) => section.page === page)
    ?.label ?? "Repository"}
</p>
