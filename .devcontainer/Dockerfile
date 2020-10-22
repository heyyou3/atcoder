# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.134.0/containers/cpp/.devcontainer/base.Dockerfile
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/cpp:0-${VARIANT}

RUN apt update -y && \
    apt install -y \
    nodejs \
    npm \
    python3 \
    python3-pip \
    && \
    pip3 install online-judge-tools && \
    npm install -g atcoder-cli && \
    mkdir "$(acc config-dir)/cpp"
COPY ./templates/cpp ./.vscode /home/vscode/.config/atcoder-cli-nodejs/cpp
