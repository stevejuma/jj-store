# Selected adapter reads

A checked manifest supplies a read-only selected-record API returning each exact
WAL reference with its verified publication/receipt chain. Its page remains
bound to that manifest during later writes or checkpoint revisions. Selected
records retain their own complete inventory root, so an earlier record cannot
enumerate objects committed by a later publication. Callers supply current
authorization, deadline and page/inventory budgets; this API grants no
publication method.

Selected adapter records expose the authenticated principal from their verified
receipt, without exposing raw receipt result bytes. This accessor changes no
persistence format and never derives actors from operation descriptions.
