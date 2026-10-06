# Browser history language qualification

Browser history readers expose standard aliases pinned to JJ 0.45.1 without
loading repository or device revset configuration. This does not change CLI
configuration precedence. The local browser regression compares the browser's
standard aliases against the pinned JJ executable, then poisons repository alias
configuration to prove reader isolation. Native lifecycle regressions qualify
filtered operation pagination, traversal limits and read-only behavior.
