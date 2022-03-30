FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.axia.image.authors="devops-team@axia.io" \
	io.axia.image.vendor="Axia Technologies" \
	io.axia.image.title="axia/subkey" \
	io.axia.image.description="Subkey: key generating utility for Substrate." \
	io.axia.image.source="https://github.com/axiatech/substrate/blob/${VCS_REF}/.maintain/docker/subkey.Dockerfile" \
	io.axia.image.revision="${VCS_REF}" \
	io.axia.image.created="${BUILD_DATE}" \
	io.axia.image.documentation="https://github.com/axiatech/substrate/tree/${VCS_REF}/subkey"

# show backtraces
ENV RUST_BACKTRACE 1

# add user
RUN useradd -m -u 1000 -U -s /bin/sh -d /subkey subkey

# add subkey binary to docker image
COPY ./subkey /usr/local/bin

USER subkey

# check if executable works in this container
RUN /usr/local/bin/subkey --version

ENTRYPOINT ["/usr/local/bin/subkey"]
