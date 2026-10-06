# Bundled appearance assets

The browser bundles the pinned Shiki 4.0.2 theme data as generated semantic and
syntax palettes and Inter 5.2.8 fonts for the Buzz-derived Appearance page.
Generation uses npm theme data and the attributed owned palette adapter; no Buzz
or Community checkout is a runtime/build dependency.

The picker also includes the pre-Appearance Design Core semantic palette as the
static Original light/dark pair. Its historical snapshot remains independent of
Shiki generation; the generated import index includes both sources. Public
foundation dialog tokens and the app-owned prose composer's Mira bridge bind
surface and text roles to the resolved palette. The owned palette adapter binds
the semantic popover background to the app background across all 62 generated
styles, including search Command View, menu and Popover consumers. Compact
navigation retains its app-surface binding for the historical Original pair.
Buzz chrome reveals one viewport-height shell gradient, while glass keeps its
selected tint and blur. Standalone code/file editors retain their own editor
surfaces. These bindings stay in owned presentation and generated assets with no
foundation dependency change. Palette checks regenerate and compare the exact
source-derived bytes.

Local Appearance acceptance in main uses the existing ignored foundational
package hook against landed Design Core source. Its chat typography and native
scrolling extensions are absent from registry 0.2.0; portable manifest/lockfile
ranges remain registry-based and registry-only qualification stays separate.

The service-owned physical working-file facade uses safe descriptor-relative
filesystem operations through rustix. It owns bounded text admission, device
selection fingerprints and atomic Save; it has no upstream JJ imports and does
not compose a repository transaction. Local HTTP admission supplies the verified
folder binding. Native mutation/storage boundaries remain independent.

Tooltip sizing restoration is qualified from the independent Design Core
repository through the documented local source hook, preserving the standard
arrow and public sizing tokens without an application override.

The Projects hub delegates Agents scope and collection reads to owned
UI-controller modules. Scope discovery joins unfiltered project links; sessions
retain separate repository admission and native/local snapshots. The shared
agent-session row is owned under web/src/lib and reused by repository and
project directories. Scoped response controllers reuse the owned repository
collaboration readers, reactions and tool review actions while keeping their
data and local response route separate from the global connection and Projects
scope. Response selection cannot reload its collection, late selections cannot
replace newer results, and revoked target access clears protected responses.
Related review/repository collection dispatch remains a separate hub helper;
production never imports catalog fixtures.
