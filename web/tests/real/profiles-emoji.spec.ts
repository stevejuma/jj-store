import { expect, test } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync, spawn } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
test("cold local and enrolled profiles and emoji survive real object store restart", async ({
  page,
  request,
}) => {
  test.setTimeout(240000);
  page.setDefaultTimeout(15000);
  const runtime = process.env.JJ_STORE_REAL_RUNTIME!,
    root = process.env.JJ_STORE_REAL_ROOT!,
    base = process.env.JJ_STORE_REAL_BASE_URL!;
  const localHeaders = {
    "x-jj-store-protocol": "1",
    Origin: base,
    "Sec-Fetch-Site": "same-origin",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Dest": "empty",
  };
  const errors: string[] = [];
  let directoryWrites = 0;
  page.on("request", (request) => {
    if (request.method() === "POST" && request.url().includes("/v1/directory/"))
      directoryWrites++;
  });
  page.on("pageerror", (e) => errors.push(e.message));
  await page.goto("/#/settings?section=profile&tab=details");
  await expect(
    page.getByRole("button", { name: "Edit avatar", exact: true }),
  ).toBeVisible();
  const photo = Buffer.from(
    await page.evaluate(() => {
      const canvas = document.createElement("canvas");
      canvas.width = 2500;
      canvas.height = 1800;
      const context = canvas.getContext("2d")!;
      context.fillStyle = "#6e56cf";
      context.fillRect(0, 0, canvas.width, canvas.height);
      return canvas.toDataURL("image/jpeg").split(",")[1]!;
    }),
    "base64",
  );
  await page
    .getByRole("button", { name: "Edit profile photo", exact: true })
    .click();
  await page.getByLabel("Upload avatar image", { exact: true }).setInputFiles({
    name: "large-headshot.jpg",
    mimeType: "image/jpeg",
    buffer: photo,
  });
  const avatarImage = page.getByRole("img", {
    name: "Selected avatar",
    exact: true,
  });
  await expect
    .poll(() =>
      avatarImage.evaluate((image) => [
        (image as HTMLImageElement).naturalWidth,
        (image as HTMLImageElement).naturalHeight,
      ]),
    )
    .toEqual([512, 369]);
  await expect(
    page.getByRole("button", { name: "Clear avatar", exact: true }),
  ).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Edit avatar", exact: true }),
  ).toHaveCount(0);
  await page.keyboard.press("Escape");
  await expect(
    page.getByRole("button", { name: /^Copy full Principal ID:/ }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Edit", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Display name" })
    .fill("Persisted administrator");
  await page
    .getByRole("textbox", { name: "Email", exact: true })
    .fill("private@example.com");
  await page.getByRole("button", { name: "Save profile" }).click();
  await expect(
    page.getByText("Persisted administrator", { exact: true }).first(),
  ).toBeVisible();
  const profile = await (
    await request.get("/v1/directory/profile", {
      headers: localHeaders,
    })
  ).json();
  expect(profile.details.avatar).toMatch(/^blob:[0-9a-f]{64}$/);
  await page.goto("/#/settings?section=custom-emoji&tab=library");
  const writesBeforeSelection = directoryWrites;
  await page.getByLabel("Upload emoji image file").setInputFiles({
    name: "party.jpg",
    mimeType: "image/jpeg",
    buffer: Buffer.concat([
      photo,
      Buffer.alloc(2 * 1024 * 1024 + 1 - photo.length),
    ]),
  });
  const localPreview = page.getByRole("img", { name: "Preview of :party:" });
  await expect
    .poll(() =>
      localPreview.evaluate(
        (image) => (image as HTMLImageElement).naturalWidth,
      ),
    )
    .toBe(2500);
  expect(directoryWrites).toBe(writesBeforeSelection);
  await expect(page.getByRole("button", { name: "Save emoji" })).toBeEnabled();
  await page.getByRole("button", { name: "Save emoji" }).click();
  await expect(page.getByText(":party:", { exact: true })).toBeVisible();
  const emojiImage = page
    .locator(".community-settings__emoji-list img")
    .first();
  await expect(emojiImage).toBeVisible();
  await expect
    .poll(() =>
      emojiImage.evaluate((image) => (image as HTMLImageElement).naturalWidth),
    )
    .toBe(512);
  expect(
    (
      await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
        .analyze()
    ).violations,
  ).toEqual([]);
  const local = join(runtime, "local"),
    config = join(local, "server.toml"),
    binary = join(root, "target/debug/jj-store");
  if (process.env.JJ_STORE_REAL_EMPTY) {
    process.kill(Number(process.env.JJ_STORE_REAL_PID), "SIGTERM");
    await expect
      .poll(async () => {
        try {
          return (
            await request.get("/v1/bootstrap", {
              headers: localHeaders,
            })
          ).status();
        } catch {
          return 0;
        }
      })
      .toBe(0);
    await new Promise((resolve) => setTimeout(resolve, 1000));
  } else {
    execFileSync("pnpm", ["dev:stop"], {
      cwd: root,
      env: { ...process.env, JJ_STORE_DEV_ROOT: runtime },
      stdio: "pipe",
    });
  }
  const restart = async () => {
    const process = spawn(
      binary,
      ["run", "--config", config, "--loopback-only"],
      { cwd: root, stdio: "ignore" },
    );
    await expect
      .poll(
        async () => {
          try {
            return (
              await request.get("/v1/bootstrap", {
                headers: localHeaders,
              })
            ).status();
          } catch {
            return 0;
          }
        },
        { timeout: 30000 },
      )
      .toBe(200);
    return process;
  };
  let service = await restart();
  try {
    await page.goto("/#/settings?section=profile&tab=details");
    await expect(
      page.getByText("Persisted administrator", { exact: true }).first(),
    ).toBeVisible();
    await expect
      .poll(() =>
        avatarImage.evaluate((image) => [
          (image as HTMLImageElement).naturalWidth,
          (image as HTMLImageElement).naturalHeight,
        ]),
      )
      .toEqual([512, 369]);
    const stop = async () => {
      service.kill("SIGTERM");
      await new Promise<void>((resolve) =>
        service.once("exit", () => resolve()),
      );
    };
    await stop();
    const operator = `jst1_${randomBytes(16).toString("hex")}_${randomBytes(32).toString("hex")}`;
    const user = `jst1_${randomBytes(16).toString("hex")}_${randomBytes(32).toString("hex")}`;
    const principal = randomBytes(16).toString("hex");
    execFileSync(
      binary,
      [
        "operator",
        "grant",
        "--policy",
        join(local, "operators.json"),
        "--endpoint",
        base,
        "--insecure-loopback",
        "--principal",
        randomBytes(16).toString("hex"),
        "--permission",
        "manage-identities",
      ],
      {
        cwd: root,
        env: { ...process.env, JJ_STORE_OPERATOR_TOKEN: operator },
        stdio: "pipe",
      },
    );
    let text = readFileSync(config, "utf8")
      .replace('mode = "local-trusted"', 'mode = "authenticated"')
      .replace(/^rpc_socket = .*\n/gm, "")
      .replace(/^permission = .*\n/gm, "");
    writeFileSync(config, text);
    service = await restart();
    const journal = join(runtime, "enroll-user.json");
    const args = [
      "identity",
      "--endpoint",
      base,
      "--insecure-loopback",
      "grant",
      "--principal",
      principal,
      "--token-stdin",
      "--request",
      journal,
    ];
    execFileSync(binary, args, {
      cwd: root,
      env: { ...process.env, JJ_STORE_OPERATOR_TOKEN: operator },
      input: user + "\n",
      stdio: ["pipe", "pipe", "pipe"],
    });
    expect(readFileSync(journal, "utf8")).not.toContain(user);
    const headers = {
      ...localHeaders,
      authorization: `Bearer ${user}`,
    };
    expect((await request.get("/v1/repositories", { headers })).status()).toBe(
      200,
    );
    const saved = await request.post("/v1/directory/commands", {
      headers,
      data: {
        request_id: randomBytes(16).toString("hex"),
        command: {
          type: "save_profile",
          expected: null,
          details: {
            display_name: "Enrolled user",
            description: "Persisted description",
            email: "user@example.com",
            avatar: null,
          },
        },
      },
    });
    expect(saved.status()).toBe(200);
    await page.goto("/");
    await page.getByRole("textbox", { name: "Access token" }).fill(user);
    await page.getByRole("button", { name: "Continue", exact: true }).click();
    await page.evaluate(() => {
      location.hash = "/settings?section=profile&tab=details";
    });
    await expect(
      page.getByText("Enrolled user", { exact: true }).first(),
    ).toBeVisible();
    await page.getByRole("button", { name: "Edit", exact: true }).click();
    await page
      .getByRole("textbox", { name: "Display name" })
      .fill("Enrolled browser user");
    await page.getByRole("button", { name: "Save profile" }).click();
    await expect(
      page.getByText("Enrolled browser user", { exact: true }).first(),
    ).toBeVisible();
    const publicProfile = await (
      await request.get(`/v1/directory/profiles?ids=${profile.principal}`, {
        headers,
      })
    ).json();
    expect(JSON.stringify(publicProfile)).not.toContain("email");
    const secondUser = `jst1_${randomBytes(16).toString("hex")}_${randomBytes(32).toString("hex")}`;
    const secondPrincipal = randomBytes(16).toString("hex");
    execFileSync(
      binary,
      [
        "identity",
        "--endpoint",
        base,
        "--insecure-loopback",
        "grant",
        "--principal",
        secondPrincipal,
        "--token-stdin",
        "--request",
        join(runtime, "enroll-second-user.json"),
      ],
      {
        cwd: root,
        env: { ...process.env, JJ_STORE_OPERATOR_TOKEN: operator },
        input: secondUser + "\n",
        stdio: ["pipe", "pipe", "pipe"],
      },
    );
    await page.getByRole("button", { name: "Edit", exact: true }).click();
    await page
      .getByRole("textbox", { name: "Email", exact: true })
      .fill("retained-draft@example.invalid");
    await page
      .getByRole("button", { name: "Back to jj-store", exact: true })
      .click();
    const profileMenu = page.getByRole("button", { name: /Open profile menu/ });
    if (!(await profileMenu.isVisible()))
      await page
        .getByRole("button", { name: "Expand left sidebar", exact: true })
        .click();
    await profileMenu.click();
    await page.getByRole("menuitem", { name: "Sign out", exact: true }).click();
    await page.getByRole("textbox", { name: "Access token" }).fill(secondUser);
    await page.getByRole("button", { name: "Continue", exact: true }).click();
    await page.evaluate(() => {
      location.hash = "/settings?section=profile&tab=details";
    });
    await expect(
      page.getByRole("button", { name: "Edit", exact: true }),
    ).toBeVisible();
    await page.getByRole("button", { name: "Edit", exact: true }).click();
    await expect(
      page.getByRole("textbox", { name: "Email", exact: true }),
    ).toHaveValue("");
    await expect(
      page.getByRole("textbox", { name: "Display name" }),
    ).toHaveValue("");
    await stop();
    service = await restart();
    expect(
      (await (await request.get("/v1/directory/profile", { headers })).json())
        .details.display_name,
    ).toBe("Enrolled browser user");
    expect(
      (
        await (await request.get("/v1/directory/emojis", { headers })).json()
      ).some((e: { shortcode: string }) => e.shortcode === "party"),
    ).toBe(true);
  } finally {
    if (service.exitCode === null) {
      service.kill("SIGTERM");
      await new Promise<void>((resolve) =>
        service.once("exit", () => resolve()),
      );
    }
  }
  expect(errors).toEqual([]);
});
