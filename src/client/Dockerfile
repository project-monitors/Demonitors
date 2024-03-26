FROM ubuntu:latest
COPY target/release/monitor_client /monitors_client
RUN chmod +x /monitors_client
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/monitor_client"]
