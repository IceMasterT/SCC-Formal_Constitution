# SPDX-License-Identifier: Apache-2.0
# Copyright 2025-2026 Ian Farquharson
FROM rust:1.78-bookworm
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates python3 \
    && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /artifact
COPY . /artifact
CMD ["bash", "scripts/run_release_gates.sh"]
