FROM rust:latest

RUN git clone https://github.com/lycheeverse/lychee
WORKDIR /lychee
RUN git pull --force
RUN cargo install cargo-deb

RUN apt-get update && apt-get install -y libc6-dev libssl-dev
# ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
# RUN mkdir /root/.cargo
# RUN echo "[target.arm-unknown-linux-gnueabihf]" >> /root/.cargo/config
# RUN echo "strip = { path = \"arm-linux-gnueabihf-strip\" }" >> /root/.cargo/config
# RUN echo "objcopy = { path = \"arm-linux-gnueabihf-objcopy\" }" >> /root/.cargo/config

COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]