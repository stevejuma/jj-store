# Frequently asked questions

## Is the custom `jj` a replacement for the upstream manual?

The custom executable adds application command families while retaining the
pinned upstream command engine. This manual documents those additions and their
interactions with application storage. Use upstream JJ documentation for general
revision expressions and ordinary code workflows; application support still
depends on the selected repository profile.

## Does registering a folder copy it into native storage?

No. Registration adds device discovery information. A local Git-backed folder
keeps its code authority and may use companion metadata. Native creation,
workspace attachment and registration are distinct operations. See
[repository contexts](getting-started/contexts.md).

## Does Save make a snapshot?

In the local Files editor, Save writes the physical working file. It does not
create a JJ snapshot or publish application metadata. Native mutations have
their own preview, apply and receipt workflow. Check the current context before
choosing an action.

## Is a project the same as a repository?

A project belongs to a metadata home and can link code repositories. The links
keep exact identities and require separate access. They neither duplicate code
nor combine publication into one transaction.

## Can I use the application offline?

Local reads and supported local mutations use durable local state. Native
upstream sync can wait until connectivity returns. Bundled Help, including
search and diagrams, needs no internet. External reference links require
connectivity.

## Why does a timed-out operation remain pending?

The commit may have succeeded before its acknowledgement was lost. Retain the
original request and resolve its status. Reissuing intent with a new ID can
apply it twice. See [publication and recovery](technical/publication.md).

## Does a Git push back up tasks and reviews?

Ordinary code push does not transfer the local companion ref. Native replication
also has domain-specific gates, including the WorkStore-home freeze. Do not
infer collaboration backup from a code push or a local successful save.

## Why can I browse a folder but not search or edit every kind of object?

Browsing, indexed search, filesystem writing, metadata commands and recoverable
code commands are separate capabilities. The
[capability reference](reference/capabilities.md) identifies current limits. A
disabled action is preferable to changing data under an unsupported provider.
