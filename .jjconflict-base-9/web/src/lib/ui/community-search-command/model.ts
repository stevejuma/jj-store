export interface CommunitySearchRow {
  id: string;
  kind:
    | "thread"
    | "message"
    | "project"
    | "task"
    | "comment"
    | "repository"
    | "operation"
    | "commit"
    | "file";
  title: string;
  description: string;
  source?: string;
  timestamp: number;
}

export interface CommunitySearchAction {
  id: string;
  label: string;
  run: () => void;
  create?: boolean;
}
