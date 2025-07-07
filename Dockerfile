FROM rust:slim-trixie

# Install dependencies
RUN apt-get update && apt-get install -y \
    gcc g++ podman uidmap fuse-overlayfs slirp4netns iptables \
    && rm -rf /var/lib/apt/lists/*

# Create a minimal /etc/containers/storage.conf
RUN mkdir -p /etc/containers && \
    echo '[storage]\ndriver="vfs"\n' > /etc/containers/storage.conf

# Optional: set env vars to force vfs and disable userns
ENV _CONTAINERS_USERNS_CONFIGURED="" \
    STORAGE_DRIVER=vfs \
    CONTAINERS_STORAGE_CONF=/etc/containers/storage.conf

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/zmanger-rce"]
