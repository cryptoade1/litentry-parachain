# syntax=docker/dockerfile:experimental
# Copyright 2021 Integritee AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#           http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This is a multi-stage docker file, where the first stage is used
# for building and the second deploys the built application.

### Builder Stage
##################################################
FROM litentry/litentry-tee-dev:edge AS builder
LABEL maintainer="Litentry Technologies GmbH <info@litentry.com>"

# set environment variables
ENV SGX_SDK /opt/sgxsdk
ENV PATH "$PATH:${SGX_SDK}/bin:${SGX_SDK}/bin/x64:/root/.cargo/bin"
ENV PKG_CONFIG_PATH "${PKG_CONFIG_PATH}:${SGX_SDK}/pkgconfig"
ENV LD_LIBRARY_PATH "${LD_LIBRARY_PATH}:${SGX_SDK}/sdk_libs"
ENV CARGO_NET_GIT_FETCH_WITH_CLI true
ENV SGX_MODE SW

ENV HOME=/home/ubuntu/repo

ARG WORKER_MODE_ARG
ENV WORKER_MODE=$WORKER_MODE_ARG

ARG ADDITIONAL_FEATURES_ARG
ENV ADDITIONAL_FEATURES=$ADDITIONAL_FEATURES_ARG

WORKDIR $HOME/tee-worker
COPY . $HOME

RUN make

RUN cargo test --release


### Cached Builder Stage (WIP)
##################################################
# A builder stage that uses sccache to speed up local builds with docker
# Installation and setup of sccache should be moved to the integritee-dev image, so we don't
# always need to compile and install sccache on CI (where we have no caching so far).
FROM litentry/litentry-tee-dev:edge AS builder
LABEL maintainer="Litentry Technologies GmbH <info@litentry.com>"

# set environment variables
ENV SGX_SDK /opt/sgxsdk
ENV PATH "$PATH:${SGX_SDK}/bin:${SGX_SDK}/bin/x64:/opt/rust/bin"
ENV PKG_CONFIG_PATH "${PKG_CONFIG_PATH}:${SGX_SDK}/pkgconfig"
ENV LD_LIBRARY_PATH "${LD_LIBRARY_PATH}:${SGX_SDK}/sdk_libs"
ENV CARGO_NET_GIT_FETCH_WITH_CLI true

# Default SGX MODE is software mode
ARG SGX_MODE=SW
ENV SGX_MODE=$SGX_MODE

ARG SGX_PRODUCTION=0
ENV SGX_PRODUCTION=$SGX_PRODUCTION


ENV HOME=/home/ubuntu/repo

RUN rustup default stable
RUN cargo install sccache

ENV SCCACHE_CACHE_SIZE="20G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/opt/rust/bin/sccache"

ARG WORKER_MODE_ARG
ARG ADDITIONAL_FEATURES_ARG
ENV WORKER_MODE=$WORKER_MODE_ARG
ENV ADDITIONAL_FEATURES=$ADDITIONAL_FEATURES_ARG

ARG FINGERPRINT=none

WORKDIR $HOME/tee-worker
COPY . $HOME

RUN --mount=type=cache,id=cargo-registry-cache,target=/opt/rust/registry/cache,sharing=private \
	--mount=type=cache,id=cargo-registry-index,target=/opt/rust/registry/index,sharing=private \
	--mount=type=cache,id=cargo-git,target=/opt/rust/git/db,sharing=private \
	--mount=type=cache,id=cargo-sccache-${WORKER_MODE}${ADDITIONAL_FEATURES},target=/home/ubuntu/.cache/sccache \
	echo ${FINGERPRINT} && make && make identity && cargo test --release && sccache --show-stats

### Base Runner Stage
##################################################
FROM ubuntu:22.04 AS runner

RUN apt update && apt install -y libssl-dev iproute2 curl

## ts-tests
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash
RUN apt-get install -y nodejs jq
RUN corepack enable
RUN corepack prepare yarn@3.6.1 --activate

### Deployed CLI client
##################################################
FROM runner AS deployed-client
LABEL maintainer="Litentry Technologies GmbH <info@litentry.com>"

ARG SCRIPT_DIR=/usr/local/worker-cli
ARG LOG_DIR=/usr/local/log

ENV SCRIPT_DIR ${SCRIPT_DIR}
ENV LOG_DIR ${LOG_DIR}

COPY --from=builder /home/ubuntu/repo/tee-worker/bin/litentry-cli /usr/local/bin
COPY --from=builder /home/ubuntu/repo/tee-worker/cli/*.sh /usr/local/worker-cli/

RUN chmod +x /usr/local/bin/litentry-cli ${SCRIPT_DIR}/*.sh
RUN mkdir ${LOG_DIR}

RUN ldd /usr/local/bin/litentry-cli && \
    /usr/local/bin/litentry-cli --version

ENTRYPOINT ["/usr/local/bin/litentry-cli"]


### Deployed worker service
##################################################
FROM runner AS deployed-worker
LABEL maintainer="litentry-dev"

WORKDIR /usr/local/bin

COPY --from=builder /opt/sgxsdk /opt/sgxsdk
COPY --from=builder /home/ubuntu/repo/tee-worker/bin/* /usr/local/bin
COPY --from=builder /home/ubuntu/repo/tee-worker/cli/*.sh /usr/local/worker-cli/
COPY --from=builder /lib/x86_64-linux-gnu/libsgx* /lib/x86_64-linux-gnu/
COPY --from=builder /lib/x86_64-linux-gnu/libdcap* /lib/x86_64-linux-gnu/

RUN touch spid.txt key.txt
RUN chmod +x /usr/local/bin/litentry-worker
RUN ls -al /usr/local/bin

# checks
ENV SGX_SDK /opt/sgxsdk
ENV LD_LIBRARY_PATH $LD_LIBRARY_PATH:$SGX_SDK/sdk_libs
RUN ldd /usr/local/bin/litentry-worker && \
    /usr/local/bin/litentry-worker --version

ENTRYPOINT ["/usr/local/bin/litentry-worker"]
