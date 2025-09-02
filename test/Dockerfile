FROM debian:trixie

COPY test/files/etc /etc

COPY target/release/libnss_nobody.so /tmp/libnss_nobody.so.2

RUN install -m 0644 /tmp/libnss_nobody.so.2 /lib && \
    /sbin/ldconfig -n /lib /usr/lib
