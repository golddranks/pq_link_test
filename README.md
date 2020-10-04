# Reproduction of pq-sys linking bug

When linking statically libpq.a that is built with support for OpenSSL,
using the resulting crate will fail in some situations.

I managed to achieve the repro under these conditions:

- The libpq used here is built to include support for OpenSSL.
- pq-sys is built statically.
- The distro is Alpine, which uses musl, but links it dynamically. They patch the x86_64-unknown-linux-musl to default to dynamic linkage.
- It happens with procedural macros. Those are always .so, dynamically linked objects.


The bug issue: https://github.com/sgrif/pq-sys/issues/25

There is a PR that links OpenSSL and fixes the problem.

A fix: https://github.com/sgrif/pq-sys/pull/29

```
$ docker build .
Sending build context to Docker daemon  87.55kB
Step 1/6 : FROM rust:1.46-alpine3.12
 ---> 16c57101cc99
Step 2/6 : RUN apk add --no-cache musl-dev 'postgresql-dev<12.0' openssl-dev openssl-libs-static
 ---> Using cache
 ---> d6da51b14841
Step 3/6 : ENV PQ_LIB_STATIC_X86_64_UNKNOWN_LINUX_MUSL=true     OPENSSL_STATIC=true
 ---> Using cache
 ---> e2835bd45255
Step 4/6 : WORKDIR /work
 ---> Using cache
 ---> 6d210b176ffc
Step 5/6 : COPY . .
 ---> e3eb51f2bf21
Step 6/6 : RUN cargo build
 ---> Running in df842e79d99e
    Updating git repository `https://github.com/Raniz85/pq-sys`
    Updating crates.io index
   Compiling pq-sys v0.4.6 (https://github.com/Raniz85/pq-sys?branch=25-openssl_sys-static#24e88457)
   Compiling test_dependency v1.0.0 (/work/test_macro)
   Compiling test v1.0.0 (/work)
error: Error relocating /work/target/debug/deps/libtest_dependency-baafb0844dd8cd05.so: X509_free: symbol not found
 --> src/main.rs:1:1
  |
1 | extern crate test_dependency;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: could not compile `test`.

To learn more, run the command again with --verbose.
The command '/bin/sh -c cargo build' returned a non-zero code: 101
```

Enable the OpenSSL linking feature in Cargo.toml to witness a working build:
```
[dependencies]
#test_dependency = { path = "test_macro" }
test_dependency = { path = "test_macro", features = ["openssl-static"] }
```

Run with:

```
$ docker build .
Sending build context to Docker daemon  87.55kB
Step 1/6 : FROM rust:1.46-alpine3.12
 ---> 16c57101cc99
Step 2/6 : RUN apk add --no-cache musl-dev 'postgresql-dev<12.0' openssl-dev openssl-libs-static
 ---> Using cache
 ---> d6da51b14841
Step 3/6 : ENV PQ_LIB_STATIC_X86_64_UNKNOWN_LINUX_MUSL=true     OPENSSL_STATIC=true
 ---> Using cache
 ---> e2835bd45255
Step 4/6 : WORKDIR /work
 ---> Using cache
 ---> 6d210b176ffc
Step 5/6 : COPY . .
 ---> Using cache
 ---> ccdf93a58ff0
Step 6/6 : RUN cargo build
 ---> Running in 8b1198bc9aeb
    Updating git repository `https://github.com/Raniz85/pq-sys`
    Updating crates.io index
 Downloading crates ...
  Downloaded cc v1.0.60
  Downloaded libc v0.2.78
  Downloaded openssl-sys v0.9.58
  Downloaded pkg-config v0.3.18
  Downloaded autocfg v1.0.1
   Compiling autocfg v1.0.1
   Compiling cc v1.0.60
   Compiling pkg-config v0.3.18
   Compiling libc v0.2.78
   Compiling pq-sys v0.4.6 (https://github.com/Raniz85/pq-sys?branch=25-openssl_sys-static#24e88457)
   Compiling openssl-sys v0.9.58
   Compiling test_dependency v1.0.0 (/work/test_macro)
   Compiling test v1.0.0 (/work)
    Finished dev [unoptimized + debuginfo] target(s) in 28.65s
Removing intermediate container 8b1198bc9aeb
 ---> 33893d77a6bd
Successfully built 33893d77a6bd
```
