# nss-nobody

## Build

```bash
./hack/build.sh
```

## Build docker test image

```bash
./hack/build-docker-test.sh
```

## Run docker test image

```bash
./hack/start-docker-test.sh
id test
```

## Install

```bash
curl -sL -o libnss_nobody.so.2 https://github.com/markafarrell/nss-nobody/releases/latest/download/libnss_nobody_$(uname -m).so.2 && \
install -m 0644 libnss_nobody.so.2 /lib && \
/sbin/ldconfig -n /lib /usr/lib &&
rm libnss_nobody.so.2
```

