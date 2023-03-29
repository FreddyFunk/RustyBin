####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN echo $(rustc -vV | sed -n 's|host: ||p')
RUN update-ca-certificates

# Create appuser
ENV USER=rusty_bin
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /rusty_bin

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rusty_bin

# Copy our build
COPY --from=builder /rusty_bin/target/release/rusty_bin ./

# Use an unprivileged user.
USER rusty_bin:rusty_bin

CMD ["/rusty_bin/rusty_bin"]