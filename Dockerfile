FROM scratch AS qualified-payload
COPY .release/image/ /payload/

FROM gcr.io/distroless/cc-debian12:nonroot@sha256:9dac0a79194e45a7da0158a9c6da57b217585af0786db3845d1f0ec1a0dd182f
LABEL org.opencontainers.image.source="https://github.com/stevejuma/jj-store" \
      org.opencontainers.image.description="Authenticated native JJ server with external RustFS" \
      org.opencontainers.image.licenses="Apache-2.0"
COPY --from=qualified-payload /payload/jj-store /usr/local/bin/jj-store
COPY --from=qualified-payload /payload/web/ /usr/share/jj-store/web/
COPY --from=qualified-payload /payload/licenses/ /usr/share/licenses/jj-store/
COPY --from=qualified-payload /payload/LICENSE.md /usr/share/licenses/jj-store/LICENSE.md
USER 65532:65532
WORKDIR /var/lib/jj-store
EXPOSE 7443
STOPSIGNAL SIGTERM
ENTRYPOINT ["/usr/local/bin/jj-store"]
CMD ["run", "--config", "/var/lib/jj-store/server.toml"]
