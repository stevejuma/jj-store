import FileText from "@lucide/svelte/icons/file-text";
import Shield from "@lucide/svelte/icons/shield";
import Box from "@lucide/svelte/icons/box";
import Sliders from "@lucide/svelte/icons/sliders-horizontal";
import Users from "@lucide/svelte/icons/users";
import Upload from "@lucide/svelte/icons/cloud-upload";
import Network from "@lucide/svelte/icons/network";
import Clock from "@lucide/svelte/icons/clock";
import Database from "@lucide/svelte/icons/database";
import Radio from "@lucide/svelte/icons/radio";
import Activity from "@lucide/svelte/icons/activity";
import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
import Gauge from "@lucide/svelte/icons/gauge";
import Hash from "@lucide/svelte/icons/hash";
import type { Component } from "svelte";

export const settingsRowIcons: Readonly<Record<string, Component>> = {
  Name: Box,
  Saved: Database,
  "Currently active": Activity,
  Operation: Activity,
  "Repository ID": Box,
  Permission: Shield,
  "Selected operation": GitCommit,
  Epoch: Clock,
  "Published position": Database,
  "Manifest revision": Hash,
  Conflicts: Users,
  Profile: FileText,
  "Native profile": FileText,
  Authentication: Shield,
  "Supported APIs": Box,
  "Typed mutations": Sliders,
  "Conflict actions": Users,
  "Mutation request limit": FileText,
  "Conflict upload limit": Upload,
  "Requests / workers": Network,
  Deadline: Clock,
  "Page / index commits": Database,
  "JSON / file limits": FileText,
  "Event streams / replay": Radio,
  "Event page / polling": Clock,
  "Code state version": Hash,
  "Companion metadata": FileText,
  "Folder generation": Hash,
  Worker: Activity,
  "Local repository": Database,
  Upstream: Network,
  "Offline access": Shield,
  "Operation heads": GitCommit,
  "Local pending positions": Gauge,
  "Peer pending positions": Gauge,
  "Checkpoint revision": Hash,
  "Search repository / indexed position": Database,
  Replica: Network,
  "Log epoch": Clock,
  "Through / observed upper bound": Gauge,
  Configured: Sliders,
  Health: Activity,
  "Repository position": Database,
  "Durable position": Database,
  "Indexed position": Database,
  "Index generation": Hash,
  Request: FileText,
  "Integrity check": Shield,
  Checkpoint: Database,
  "Push target epoch": Clock,
  "Pull target epoch": Clock,
  "Push target replica": Network,
  "Pull target replica": Network,
  "Push target through": Gauge,
  "Pull target through": Gauge,
};
