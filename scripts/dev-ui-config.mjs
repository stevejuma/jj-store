/** @param {Record<string, string | undefined>} environment */
export function uiDevelopmentSettings(environment = process.env) {
  /** @param {string} value @param {string} name */
  const port = (value, name) => {
    if (!/^\d+$/u.test(value) || Number(value) < 1 || Number(value) > 65535)
      throw new Error(`${name} must be a port between 1 and 65535`);
    return Number(value);
  };
  const uiPort = port(environment.JJ_STORE_DEV_PORT ?? "7460", "UI port");
  const backendPort = port(
    environment.JJ_STORE_DEV_BACKEND_PORT ?? String(uiPort + 2),
    "backend port",
  );
  if (uiPort === backendPort)
    throw new Error("UI and backend ports must be different");
  return {
    uiPort,
    backendPort,
    origin: `http://127.0.0.1:${uiPort}/`,
    backendOrigin: `http://127.0.0.1:${backendPort}/`,
  };
}

/** @param {Record<string, string | undefined>} environment */
export function uiDevelopmentServer(environment = process.env) {
  const settings = uiDevelopmentSettings(environment);
  const scripts =
    environment.JJ_STORE_DEV_BROWSER_ANNOTATIONS === "0"
      ? "'self'"
      : "'self' 'unsafe-inline'";
  return {
    host: "127.0.0.1",
    port: settings.uiPort,
    strictPort: true,
    cors: false,
    fs: {
      deny: [
        ".env",
        ".env.*",
        "*.{crt,pem,key}",
        "**/.git/**",
        "**/.jj/**",
        "**/.local/**",
        "**/.scratch/**",
        "**/target/**",
      ],
    },
    proxy: {
      "/v1": { target: settings.backendOrigin, changeOrigin: false },
    },
    headers: {
      "Content-Security-Policy": [
        "default-src 'self'",
        `script-src ${scripts}`,
        "style-src 'self' 'unsafe-inline'",
        `connect-src 'self' ws://127.0.0.1:${settings.uiPort}`,
        "img-src 'self' data: https:",
        "worker-src 'self' blob:",
        "object-src 'none'",
        "frame-ancestors 'none'",
        "base-uri 'self'",
      ].join("; "),
      "Referrer-Policy": "no-referrer",
      "X-Content-Type-Options": "nosniff",
    },
  };
}

/**
 * @param {string} source
 * @param {ReturnType<typeof uiDevelopmentSettings>} settings
 */
export function rewriteUiBackendConfiguration(source, settings) {
  return source
    .replace(/^bind = ".*"$/mu, `bind = "127.0.0.1:${settings.backendPort}"`)
    .replace(/^public_url = ".*"$/mu, `public_url = "${settings.origin}"`)
    .replace(/^web_root = .*\n?/mu, "")
    .replace(/^development_browser_annotations = .*\n?/mu, "")
    .replace(
      /^(format = .+)$/mu,
      "$1\ndevelopment_browser_annotations = false",
    );
}
