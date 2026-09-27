<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import LogInIcon from "@lucide/svelte/icons/log-in";
  import LogOutIcon from "@lucide/svelte/icons/log-out";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  let {
    member,
    communityName = "jj-store",
    compact = false,
    authenticated = true,
    onAction,
  }: {
    member: CommunityMemberModel;
    communityName?: string;
    compact?: boolean;
    authenticated?: boolean;
    onAction: (action: "sign-in" | "sign-out") => void;
  } = $props();
  let menuOpen = $state(false),
    tooltipOpen = $state(false);
</script>

<div class="community-profile-menu" data-compact={compact || undefined}>
  <Tooltip.Provider>
    <DropdownMenu.Root
      open={menuOpen}
      onOpenChange={(open) => {
        menuOpen = open;
        if (open) tooltipOpen = false;
      }}
    >
      <Tooltip.Root open={tooltipOpen} disabled={!compact || menuOpen}>
        <Tooltip.Trigger>
          {#snippet child({ props: tooltipProps })}
            <span
              {...tooltipProps}
              class="community-profile-menu__tooltip-anchor"
              onpointerenter={() => (tooltipOpen = true)}
              onpointerleave={() => (tooltipOpen = false)}
            >
              <DropdownMenu.Trigger>
                {#snippet child({ props })}
                  <Button
                    {...props}
                    type="button"
                    variant="ghost"
                    class="community-profile-menu__trigger"
                    aria-label={`Open profile menu for ${member.name}`}
                  >
                    <CommunityMemberAvatar {member} showPresence decorative />
                    {#if !compact}
                      <span class="community-profile-menu__identity">
                        <strong>{member.name}</strong>
                        <small>
                          {#if authenticated && member.status}
                            {member.status}
                          {:else if !authenticated}
                            Sign in
                          {:else}
                            {communityName}
                          {/if}
                        </small>
                      </span>
                    {/if}
                  </Button>
                {/snippet}
              </DropdownMenu.Trigger>
            </span>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={8}
          >{member.name}</Tooltip.Content
        >
      </Tooltip.Root>

      <DropdownMenu.Content
        side="top"
        align="start"
        sideOffset={8}
        class="community-profile-menu__content"
        aria-label="Profile menu"
      >
        <DropdownMenu.Group class="community-profile-menu__menu-header">
          <CommunityMemberAvatar {member} showPresence decorative />
          <DropdownMenu.Label class="community-profile-menu__menu-identity">
            <strong>{member.name}</strong>
          </DropdownMenu.Label>
        </DropdownMenu.Group>

        <DropdownMenu.Group>
          {#if authenticated}<DropdownMenu.Item
              onSelect={() => onAction("sign-out")}
              ><LogOutIcon data-icon="inline-start" aria-hidden="true" />Sign
              out</DropdownMenu.Item
            >
          {:else}<DropdownMenu.Item onSelect={() => onAction("sign-in")}
              ><LogInIcon data-icon="inline-start" aria-hidden="true" />Sign in</DropdownMenu.Item
            >{/if}
        </DropdownMenu.Group>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </Tooltip.Provider>
</div>

<style>
  .community-profile-menu {
    --community-member-avatar-presence-ring: var(--sidebar, var(--background));

    padding: 0.5rem 0.75rem 0.75rem;
  }

  .community-profile-menu[data-compact] {
    padding-inline: 0.5rem;
  }

  .community-profile-menu__tooltip-anchor {
    display: flex;
    width: 100%;
  }

  :global(.community-profile-menu__trigger[data-ui-component="button"]) {
    width: 100%;
    min-width: 0;
    height: auto;
    min-height: 3rem;
    justify-content: flex-start;
    gap: 0.75rem;
    padding: 0.375rem 0.5rem;
    border-radius: 0.75rem;
    color: var(--sidebar-foreground, var(--foreground));
    transition:
      color 120ms ease,
      background-color 120ms ease,
      box-shadow 120ms ease;
  }

  :global(.community-profile-menu__trigger[data-ui-component="button"]:hover),
  :global(
    .community-profile-menu__trigger[data-ui-component="button"]:focus-visible
  ),
  :global(
    .community-profile-menu__trigger[data-ui-component="button"][data-state="open"]
  ) {
    color: var(--sidebar-accent-foreground, var(--foreground));
    background: color-mix(
      in srgb,
      var(--sidebar-accent, var(--accent)) 88%,
      transparent
    );
    box-shadow: inset 0 0 0 1px
      color-mix(in srgb, var(--foreground) 7%, transparent);
  }

  :global(
    .community-profile-menu[data-compact]
      .community-profile-menu__trigger[data-ui-component="button"]
  ) {
    width: 2.5rem;
    min-height: 2.5rem;
    justify-content: center;
    margin-inline: auto;
    padding: 0;
  }

  .community-profile-menu__identity,
  :global(.community-profile-menu__menu-identity) {
    display: grid;
    min-width: 0;
    text-align: start;
  }

  .community-profile-menu__identity {
    gap: 0.125rem;
  }

  .community-profile-menu__identity strong,
  .community-profile-menu__identity small {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .community-profile-menu__identity strong {
    font-size: var(--community-text-body);
    line-height: 1.1;
  }

  .community-profile-menu__identity small {
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  :global(.community-profile-menu__content) {
    width: min(18rem, calc(100vw - 1rem));
    border-radius: 1rem;
    padding: 0.5rem;
  }

  :global(.community-profile-menu__menu-header) {
    display: grid;
    grid-template-columns: 2.25rem minmax(0, 1fr);
    grid-template-rows: auto 1fr;
    align-items: center;
    column-gap: 0.75rem;
    min-height: 3rem;
    padding: 0.375rem 0.5rem;
  }

  :global(
    .community-profile-menu__menu-header
      > [data-ui-component="community-member-avatar"]
  ) {
    grid-row: 1 / span 2;
  }

  :global(.community-profile-menu__menu-identity) {
    grid-column: 2;
    align-self: start;
    justify-items: start;
    padding: 0;
  }

  :global(.community-profile-menu__menu-identity strong) {
    color: var(--foreground);
    font-size: var(--community-text-body);
  }

  :global(.community-profile-menu__status-action) {
    min-height: 2.75rem;
    border: 1px solid var(--border);
    border-radius: 0.75rem;
  }

  :global(
    .community-profile-menu__presence-trigger[data-ui-part="dropdown-menu-sub-trigger"]
  ) {
    grid-column: 2;
    align-self: end;
    justify-self: start;
    min-height: 1.125rem;
    gap: 0.35rem;
    border-radius: 999px;
    padding: 0.0625rem 0.45rem;
    color: var(--muted-foreground);
    background: color-mix(in srgb, var(--muted-foreground) 10%, transparent);
    font-size: var(--community-text-secondary);
    line-height: 1;
  }

  :global(
    .community-profile-menu__presence-trigger[data-ui-part="dropdown-menu-sub-trigger"]
      [data-ui-part="dropdown-menu-sub-trigger-icon"]
  ) {
    display: none;
  }

  .community-profile-menu__presence-dot,
  .community-profile-menu__presence-option-dot {
    flex: none;
    width: 0.45rem;
    height: 0.45rem;
    border-radius: 999px;
    background: var(--muted-foreground);
  }

  :global(
    .community-profile-menu__presence-trigger[data-presence="online"]
      .community-profile-menu__presence-dot
  ),
  .community-profile-menu__presence-option-dot[data-presence="online"] {
    background: var(--success, #08b94e);
  }

  :global(
    .community-profile-menu__presence-trigger[data-presence="away"]
      .community-profile-menu__presence-dot
  ),
  .community-profile-menu__presence-option-dot[data-presence="away"] {
    background: var(--warning, #d99a00);
  }

  :global(.community-profile-menu__presence-content) {
    width: 11rem;
    z-index: calc(var(--ui-workspace-overlay-z-index, 50) + 20);
  }

  :global(
    .community-profile-menu__presence-content
      [data-ui-part="dropdown-menu-radio-item"]
  ) {
    gap: 0.5rem;
  }

  :global(.community-profile-menu__community-icon) {
    display: grid;
    width: 1.25rem;
    height: 1.25rem;
    flex: none;
    place-items: center;
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    background: color-mix(in srgb, var(--muted) 72%, transparent);
    font-size: var(--community-text-secondary);
    text-align: center;
  }

  :global(
    .community-profile-menu__community-trigger[data-ui-part="dropdown-menu-sub-trigger"]
  ) {
    min-height: 2.25rem;
    gap: 0.5rem;
    border-radius: 0.5rem;
    padding: 0.5rem 0.75rem;
  }

  :global(.community-profile-menu__community-content) {
    width: 15rem;
    z-index: calc(var(--ui-workspace-overlay-z-index, 50) + 20);
    padding: 0.25rem;
  }

  :global(.community-profile-menu__community-label) {
    display: grid;
    min-width: 0;
    flex: 1;
  }

  :global(.community-profile-menu__community-label small) {
    overflow: hidden;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    font-weight: 400;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
