FROM node:22.22.0-bookworm@sha256:20a424ecd1d2064a44e12fe287bf3dae443aab31dc5e0c0cb6c74bef9c78911c AS node
FROM mcr.microsoft.com/playwright:v1.63.0-noble@sha256:eff16c30e6f3f4af0a03fa4b706120d5e9b0891c344a27d64559aff5900a4a27 AS ui
COPY --from=node /usr/local/ /usr/local/
ENV DEBIAN_FRONTEND=noninteractive PNPM_HOME=/opt/pnpm PLAYWRIGHT_BROWSERS_PATH=/ms-playwright
ENV PATH=/opt/tools:/opt/pnpm:$PATH
RUN apt-get update && apt-get install -y --no-install-recommends git python3 python3-venv curl ca-certificates pkg-config libdbus-1-dev lsof && rm -rf /var/lib/apt/lists/* && npm install -g pnpm@10.32.1
COPY ci-setup-jj.mjs /opt/install-jj.mjs
RUN mkdir -p /opt/tools && RUNNER_TEMP=/opt GITHUB_PATH=/tmp/jj-path node /opt/install-jj.mjs && mv /opt/jj-store-ci-tools/jj /opt/tools/jj && rm -rf /opt/jj-store-ci-tools
RUN git config --system user.name 'JJ Store validation' && git config --system user.email 'validation@localhost' && git config --system --add safe.directory '*'
RUN mkdir -p /input /results /work/jj-store
WORKDIR /work/jj-store
FROM rust:1.99.0-bookworm@sha256:114c7a4425406451c2866b6aafe69fe29b1b298832db1277d411ac73c82d04d6 AS rust
FROM ui AS full
COPY --from=rust /usr/local/cargo /usr/local/cargo
COPY --from=rust /usr/local/rustup /usr/local/rustup
ENV RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN apt-get update && apt-get install -y --no-install-recommends build-essential clang cmake libssl-dev protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN rustup component add clippy rustfmt && cargo install --locked cargo-nextest --version 0.9.148 && cargo install --locked mdbook --version 0.5.4 && python3 -m venv /opt/docs && /opt/docs/bin/pip install uv==0.9.28
ENV PATH=/opt/docs/bin:$PATH
RUN apt-get update && apt-get install -y --no-install-recommends libcurl4-openssl-dev libexpat1-dev zlib1g-dev openssh-server xz-utils && rm -rf /var/lib/apt/lists/* && mkdir -p /run/sshd
# The native transport profile checks the actual Git version before side effects.
RUN curl --fail --location --retry 3 https://www.kernel.org/pub/software/scm/git/git-2.55.0.tar.xz -o /tmp/git.tar.xz \
    && printf '%s  %s\n' '457fdb04dc8728e007d4688695e6912e6f680727920f2a40bf11eacc17505357' '/tmp/git.tar.xz' | sha256sum --check --strict \
    && tar -xJf /tmp/git.tar.xz -C /tmp \
    && make -C /tmp/git-2.55.0 -j4 prefix=/opt/git NO_TCLTK=YesPlease NO_GETTEXT=YesPlease install \
    && test "$(/opt/git/bin/git --version)" = 'git version 2.55.0' \
    && rm -rf /tmp/git.tar.xz /tmp/git-2.55.0
ENV PATH=/opt/git/bin:$PATH
# Fixture sshd disables password authentication and uses generated approved keys.
# Ubuntu's locked root account otherwise refuses those public-key logins too.
RUN passwd -d root
