export function createInMemoryStoreSession() {
  let authenticated = $state(true);
  return {
    get authenticated() {
      return authenticated;
    },
    actions: {
      async connect() {
        authenticated = true;
      },
      disconnect() {
        authenticated = false;
      },
      signIn() {
        authenticated = true;
      },
      signOut() {
        authenticated = false;
      },
    },
  };
}
