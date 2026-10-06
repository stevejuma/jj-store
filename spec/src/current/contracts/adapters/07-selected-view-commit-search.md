# Selected-view commit search

Every core search request explicitly chooses historical metadata or commit
visibility at an authorized native read snapshot. Visible commits are the
complete parent closure of that view's heads, including the virtual root but
excluding predecessor-only history. The native reader applies history, work,
metadata, output and deadline budgets; it returns a complete set or an explicit
error. No global abandoned flag substitutes for selected-view semantics.

The search engine intersects that set with commit hits before ranking and
pagination. Operation metadata remains historical; source hits retain their
explicit indexed scope snapshots. Results identify the selected native view and
the independently pinned index generation. Later repository or index updates
cannot silently change either one. Lagging generations can omit new results but
cannot promote hidden commits into visible ones. Both modes still require fresh
repository authorization and matching epochs; visibility data grants no access.
