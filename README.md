# actix web playground

This is a playground repo to learn more about actix and actix web for rust.

## Notes

The specific exploration is centered around having a very synchronous and/or blocking workload serviced by the actix router.

For example, if you had a workload that took 5-seconds to compute there are two options:

1. a hard block, where the worker thread cannot service other incoming requests
  * which could cause the requests to be dropped
2. an awaited block via future, where the worker thread spawns work onto another thread, and awaits the future to finish, and can service other incoming requests

This is important for CPU bound workloads that effectively act like IO bound workloads.

## Versioning

This is running with `actix-web` `3` and `tokio` `0.2`.

There are some huge version variants now that Rust stable has brought better async/await support.