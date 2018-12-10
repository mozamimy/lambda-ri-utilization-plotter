FROM lambci/lambda:provided

USER root

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    CARGO_BUILD_TARGET_DIR=/tmp/target \
    PATH=/opt/cargo/bin:$PATH

RUN yum -y update
RUN rpm --rebuilddb && yum -y groupinstall "Development Tools" && yum -y install openssl-devel

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
RUN rustup component add rustfmt-preview

WORKDIR /workspace

ENTRYPOINT []
