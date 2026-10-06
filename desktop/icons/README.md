# Desktop branding

The two blue jays and orange gear come from the user's supplied
`jj-store-logo.jpeg`, accepted for jj-store branding on 2026-10-06. The original
JPEG SHA-256 is
`7df0fb3f0b6d9b6366db9a6e1ad6d36ef706d183ac77a8a012834da24874e3ce`.

`logo-light.svg` and `logo-dark.svg` are maintained vector sources with
transparent surrounds. Each contains 184 paths traced from the original with
VTracer 0.6.5. The dark variant adds a pale blue outer stroke so the navy
silhouette remains visible on dark surfaces. Interior pale bird markings remain
opaque.

The native default icon is the outlined variant: `icon.png` is the 1024 px RGBA
source used by Tauri's default window/tray icon, and `icon.icns` supplies the
macOS bundle's multiple normal and Retina sizes, up to 1024 px. `icon-light.png`
and `icon-light.icns` retain the light variant. Bundle icon selection is static;
it does not change device or application theme settings.

The exports were rendered from these SVGs with Chrome 154 and Playwright 1.63.0.
macOS `iconutil` assembled the ICNS containers. Each icon has transparent
corners and clear margins. The detailed bird markings are clearest at 48 px and
above. These assets have no runtime dependency on the Desktop logo kit or any
external image, font, Community component or generated draft.
