export interface Span {
  start: number;
  end: number;
}
export interface DeltaRecord {
  start: number;
  end?: number;
  id: string;
  kind: string;
  name?: string;
  input?: string;
  output?: string;
  status?: string;
  title?: string;
  createdAt?: number;
  titleEnd?: number;
  contentEnd?: number;
  layout?: Record<string, Span>;
  text?: string;
  commentBatchId?: string;
  replyContext?: { threadId: string; commentIds: readonly string[] };
}
export interface DeltaDiscussion {
  id: string;
  target: { recordId: string; from: number; to: number; quote: string };
  comments: {
    id: string;
    author: string;
    text: string;
    status: string;
    batchId?: string;
    replyItemId?: string;
    replyTo?: string;
  }[];
}
export interface DeltaChatFixture {
  version: number;
  source: { title: string; sha256: string };
  buffer: string;
  records: readonly DeltaRecord[];
  draftStart: number;
  draftText?: string;
  discussions?: readonly DeltaDiscussion[];
}
