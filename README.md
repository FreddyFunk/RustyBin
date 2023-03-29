# RustyBin
RustyBin is a simple yet effective REST API solution that allows for the easy storage and management of files with a size of a couple megabytes. It aims to provide a fast and seamless experience for users seeking to store single files.

# Usage
RustyBin manages files by a `file_id` which is represented as an universally unique identifier ([UUID v4](https://crates.io/crates/uuid)) via the API. This allows users to store files with identical filenames or upload identical files and overwrite them later with different files.

A `file_id` could look like this: *c3785d35-52cd-48e3-8af5-0f4c23e14c1a*

## Download files
RustyBin will not return any mime type.

```bash
# curl
curl -O http://127.0.0.1:8080/file/file_id
# wget
wget http://127.0.0.1:8080/file/file_id
```

## Upload files
The following example assumes that the file "document.pdf" is located in the current working directory. RustyBin will return the generated `file_id` in the response body.

```bash
# curl
curl -X POST -H "Content-Type: multipart/form-data" -F "file=@document.pdf" http://127.0.0.1:8080/file
```

## Overwrite files
The following example assumes that the file "document.pdf" is located in the current working directory. Make sure that `allow_overwrite` is set to true in the RustyBin configuration file. Otherwise PUT requests will be rejected.

```bash
# curl
curl -X PUT -H "Content-Type: multipart/form-data" -F "file=@document.pdf" http://127.0.0.1:8080/file/file_id
```

## Delete files
 Make sure that `allow_delete` is set to true in the RustyBin configuration file. Otherwise DELTE requests will be rejected.

```bash
# curl
curl -X DELETE http://127.0.0.1:8080/file/file_id
```
# Configuration
RustyBin will be able to run with its default configuration. The following configurations be aplied:
```yaml
server_port: 8080
worker_count: <<system_cpu_count>>
data_path: ./data/
log_level: DEBUG
allow_overwrite: false
allow_delete: false
```
Individual configurations can be overwritten in a `config.yml` file that needs to be located in the same directory as the RustyBin binary.

# Build
To create RustyBin easily, you can use a podman or docker. The Dockerfile uses a process called "multi-stage build" that has two parts: the first part creates a setup for building, and the second part creates a minimal setup for running RustyBin.

When you build the container image, RustyBin will be built in "release mode", meaning it will be optimized for performance. Additionally, all unit tests will be run and must pass for a successful build.

## Release
```bash
podman build .
```

## Development
```bash
# build
cargo build

# start RustyBin
cargo run
```
