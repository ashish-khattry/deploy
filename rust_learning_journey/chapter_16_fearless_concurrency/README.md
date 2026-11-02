# Chapter 16: Fearless Concurrency 🦀⚡

This directory contains 10 production-grade architectural programs mastering **Chapter 16 (Fearless Concurrency)** from the Official Rust Book. It covers OS threads, `JoinHandle` synchronization, `move` closures, `mpsc` message-passing channels, shared-state concurrency via `Mutex<T>` and `Arc<T>`, and the compiler marker traits `Send` and `Sync`.

## 🚀 Programs Index & Architecture Covered

1. **`01_basic_thread_spawn`** - Spawning concurrent 1:1 OS threads using `thread::spawn` and observing thread scheduling.
2. **`02_thread_join_handle`** - Blocking main thread termination using `JoinHandle::join` with crash-proof `Result` handling.
3. **`03_move_closure_ownership`** - Safely transferring heap ownership (`Vec<T>`) across thread boundaries using `move` closures.
4. **`04_mpsc_single_message`** - Unidirectional thread communication and ownership transfer using `std::sync::mpsc::channel`.
5. **`05_mpsc_stream_iterator`** - Streaming multiple values over a channel and consuming the receiver (`rx`) as an iterator.
6. **`06_mpsc_multiple_producers`** - Cloning channel transmitters (`tx.clone()`) to aggregate data from multiple concurrent producer threads.
7. **`07_mpsc_non_blocking_try_recv`** - Non-blocking channel polling using `try_recv()` and handling `TryRecvError` states.
8. **`08_mutex_single_thread_guard`** - Mutual exclusion basics, `MutexGuard` dereferencing (`DerefMut`), and automatic `Drop` unlocking.
9. **`09_arc_mutex_multithreaded_counter`** - Thread-safe multiple ownership and interior mutability across 10 threads using `Arc<Mutex<i32>>`.
10. **`10_send_sync_custom_architect`** - Demonstrating automatic `Send` and `Sync` marker trait composition on a custom multi-threaded `AuditLogger` struct.