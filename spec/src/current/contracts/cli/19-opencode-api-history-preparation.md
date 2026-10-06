# OpenCode API history preparation

The retained library can prepare an exact API message-prefix import boundary
without source enrollment, private intent writes or publication. It
independently verifies selected history and preserves accepted boundaries across
appended messages. Each step reads at most one existing bounded API page and
releases raw records. Empty history has zero logical messages and zero byte
offset; API byte positions MUST NOT be invented. Exact
endpoint/session/directory/version and selected-prefix checks govern replay.
This boundary alone does not qualify public selectors, import commands, worker
delivery or configured startup.
