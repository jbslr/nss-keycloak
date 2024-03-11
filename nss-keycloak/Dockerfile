FROM almalinux:8

RUN dnf update -y && dnf install -y cargo && mkdir -p /tmp/build && dnf install -y openssl openssl-devel
WORKDIR /tmp/build
COPY --chown=root:root . /tmp/build/

RUN mkdir -p /etc/nss-keycloak/
COPY --chown=root:root example.config.toml /etc/nss-keycloak/config.toml
ENV NSSKEYCLOAK_CONFIG_FILE=/etc/nss-keycloak/config.toml

RUN cargo build -r --lib && cp /tmp/build/target/release/libnss_keycloak.so /usr/lib64/libnss_keycloak.so.2
COPY --chown=root:root example.nsswitch.conf /etc/nsswitch.conf

CMD ["tail", "-f", "/dev/null"]
