export interface InMemoryStoreControllerOptions {
  large?: boolean;
  mutationOutcome?: "committed" | "unknown";
  mutationFailure?: string;
  reviewOutcome?: "committed" | "unknown";
}
