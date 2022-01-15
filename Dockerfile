FROM quay.io/gattytto/rust:latest as builder

# Create appuser
ENV USER=rust
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM quay.io/nushell/nu-base:latest AS nubase
#FROM quay.io/gattytto/rst-sleep:latest AS sleepbase

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /
COPY --from=builder /target/x86_64-unknown-linux-musl/release/myip ./
COPY --from=nubase /usr/local/bin/nu /nu
#COPY --from=sleepbase /sleep /sleep

USER rust:rust

CMD ["/myip"]
