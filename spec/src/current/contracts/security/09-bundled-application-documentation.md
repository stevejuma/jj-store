# Bundled application documentation

JST-DOC-004 permits same-origin framed documentation only. Documentation has its
own static CSP, including inline styles required by the pinned Material theme,
with only same-origin scripts/workers/connections and no eval. Executable inline
scripts are externalized during generation. Application inline restrictions and
API authorization are unchanged; Help contains no repository data or
credentials.
