FROM gcr.io/distroless/cc

COPY /target/release/server /
ENTRYPOINT ["/server"]