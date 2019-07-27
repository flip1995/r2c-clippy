FROM ubuntu:18.10
SHELL ["/bin/bash", "-c"]

RUN apt-get update
RUN apt-get install -y curl gcc

# Setup analysis user for docker
RUN groupadd -r analysis && useradd -m --no-log-init --gid analysis analysis
COPY src /analyzer
RUN chown -R analysis /analyzer
USER analysis

WORKDIR /home/analysis
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
RUN sh rustup.sh -y --default-toolchain nightly-2019-07-19
ENV PATH=/home/analysis/.cargo/bin:$PATH
RUN rustup component add clippy
WORKDIR /analyzer/analyzer
RUN cargo install --force --path .

# Setup entrypoint into the analysis code logic
WORKDIR /
CMD ["/analyzer/analyze.sh"]
