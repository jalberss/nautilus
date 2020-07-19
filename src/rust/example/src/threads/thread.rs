//! This crate will hopefully flush out the interface and implementation for Rust threds in Nautilus
// extern crate alloc;
// use crate::threads::thread_bindings::*;
// use alloc::sync::Arc;
// use alloc::string::String;
// use alloc::boxed::Box;
// use core::cell::UnsafeCell;
// use core::intrinsics::transmute;
// use core::ptr::null_mut;
// use core::time::Duration;

// struct JoinInner<T> {
//     native: Option<Thread>,
//     thread: Thread,
//     packet: Packet<T>,
// }

// impl<T> JoinInner<T> {
//     fn join(&mut self) -> Result<T,()> {
//         self.native.take().unwrap().join();
//         unsafe {
//             (*self.packet.0.get()).take().unwrap()
//         }
//     }
// }


// pub struct JoinHandle<T>(JoinInner<T>);
// unsafe impl<T> Send for JoinHandle<T> {}
// unsafe impl<T> Sync for JoinHandle<T> {}
// impl<T> JoinHandle<T> {
//     /// Extracts a handle to the underlying thread.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new();
//     ///
//     /// let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
//     ///     // some work here
//     /// }).unwrap();
//     ///
//     /// let thread = join_handle.thread();
//     /// println!("thread id: {:?}", thread.id());
//     /// ```

//     pub fn thread(&self) -> &Thread {
//         &self.0.thread
//     }

//     /// Waits for the associated thread to finish.
//     ///
//     /// In terms of [atomic memory orderings],  the completion of the associated
//     /// thread synchronizes with this function returning. In other words, all
//     /// operations performed by that thread are ordered before all
//     /// operations that happen after `join` returns.
//     ///
//     /// If the child thread panics, [`Err`] is returned with the parameter given
//     /// to [`panic`].
//     ///
//     /// [`Err`]: ../../std/result/enum.Result.html#variant.Err
//     /// [`panic`]: ../../std/macro.panic.html
//     /// [atomic memory orderings]: ../../std/sync/atomic/index.html
//     ///
//     /// # Panics
//     ///
//     /// This function may panic on some platforms if a thread attempts to join
//     /// itself or otherwise may create a deadlock with joining threads.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new();
//     ///
//     /// let join_handle: thread::JoinHandle<_> = builder.spawn(|| {
//     ///     // some work here
//     /// }).unwrap();
//     /// join_handle.join().expect("Couldn't join on the associated thread");
//     /// ```

//     pub fn join(mut self) -> Result<T,()> {
//         self.0.join()
//     }
// }



// // In Rust, the type Result<T,E> is often shortened to a partially applied (not really), Result<T>, where E is already the Error type often std::io::Error

// // nk_thread struct defined in Thread_bindings has a lot of stuff we don't care about
// pub struct Thread {
//     tid: *mut nk_thread_id_t,
// }

// struct Packet<T>(Arc<UnsafeCell<Option<Result<T,()>>>>);

// unsafe impl Send for Thread {}
// unsafe impl Sync for Thread {}

// // pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
// //     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
// // {
// //     //Builder::new().spawn(f).expect("failed to spawn thread")
// // }


// // This below is just copy pasted from Rust stdlib many things will have to be resolved.
// #[derive(Debug)]
// pub struct Builder {
//     // A name for the thread-to-be, for identification in panic messages
//     name: Option<String>,
//     // The size of the stack for the spawned thread in bytes
//     stack_size: Option<usize>,
// }

// impl Builder {
//     /// Generates the base configuration for spawning a thread, from which
//     /// configuration methods can be chained.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new()
//     ///                               .name("foo".into())
//     ///                               .stack_size(32 * 1024);
//     ///
//     /// let handler = builder.spawn(|| {
//     ///     // thread code
//     /// }).unwrap();
//     ///
//     /// handler.join().unwrap();
//     /// ```

//     pub fn new() -> Builder {
//         Builder {
//             name: None,
//             stack_size: None,
//         }
//     }

//     /// Names the thread-to-be. Currently the name is used for identification
//     /// only in panic messages.
//     ///
//     /// The name must not contain null bytes (`\0`).
//     ///
//     /// For more information about named threads, see
//     /// [this module-level documentation][naming-threads].
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new()
//     ///     .name("foo".into());
//     ///
//     /// let handler = builder.spawn(|| {
//     ///     assert_eq!(thread::current().name(), Some("foo"))
//     /// }).unwrap();
//     ///
//     /// handler.join().unwrap();
//     /// ```
//     ///
//     /// [naming-threads]: ./index.html#naming-threads

//     pub fn name(mut self, name: String) -> Builder {
//         self.name = Some(name);
//         self
//     }

//     /// Sets the size of the stack (in bytes) for the new thread.
//     ///
//     /// The actual stack size may be greater than this value if
//     /// the platform specifies a minimal stack size.
//     ///
//     /// For more information about the stack size for threads, see
//     /// [this module-level documentation][stack-size].
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new().stack_size(32 * 1024);
//     /// ```
//     ///
//     /// [stack-size]: ./index.html#stack-size

//     pub fn stack_size(mut self, size: usize) -> Builder {
//         self.stack_size = Some(size);
//         self
//     }

//     /// Spawns a new thread by taking ownership of the `Builder`, and returns an
//     /// [`io::Result`] to its [`JoinHandle`].
//     ///
//     /// The spawned thread may outlive the caller (unless the caller thread
//     /// is the main thread; the whole process is terminated when the main
//     /// thread finishes). The join handle can be used to block on
//     /// termination of the child thread, including recovering its panics.
//     ///
//     /// For a more complete documentation see [`thread::spawn`][`spawn`].
//     ///
//     /// # Errors
//     ///
//     /// Unlike the [`spawn`] free function, this method yields an
//     /// [`io::Result`] to capture any failure to create the thread at
//     /// the OS level.
//     ///
//     /// [`spawn`]: ../../std/thread/fn.spawn.html
//     /// [`io::Result`]: ../../std/io/type.Result.html
//     /// [`JoinHandle`]: ../../std/thread/struct.JoinHandle.html
//     ///
//     /// # Panics
//     ///
//     /// Panics if a thread name was set and it contained null bytes.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new();
//     ///
//     /// let handler = builder.spawn(|| {
//     ///     // thread code
//     /// }).unwrap();
//     ///
//     /// handler.join().unwrap();
//     /// ```

//     pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>,()> where
//         F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
//     {
//         unsafe { self.spawn_unchecked(f) }
//     }

//     /// Spawns a new thread without any lifetime restrictions by taking ownership
//     /// of the `Builder`, and returns an [`io::Result`] to its [`JoinHandle`].
//     ///
//     /// The spawned thread may outlive the caller (unless the caller thread
//     /// is the main thread; the whole process is terminated when the main
//     /// thread finishes). The join handle can be used to block on
//     /// termination of the child thread, including recovering its panics.
//     ///
//     /// This method is identical to [`thread::Builder::spawn`][`Builder::spawn`],
//     /// except for the relaxed lifetime bounds, which render it unsafe.
//     /// For a more complete documentation see [`thread::spawn`][`spawn`].
//     ///
//     /// # Errors
//     ///
//     /// Unlike the [`spawn`] free function, this method yields an
//     /// [`io::Result`] to capture any failure to create the thread at
//     /// the OS level.
//     ///
//     /// # Panics
//     ///
//     /// Panics if a thread name was set and it contained null bytes.
//     ///
//     /// # Safety
//     ///
//     /// The caller has to ensure that no references in the supplied thread closure
//     /// or its return type can outlive the spawned thread's lifetime. This can be
//     /// guaranteed in two ways:
//     ///
//     /// - ensure that [`join`][`JoinHandle::join`] is called before any referenced
//     /// data is dropped
//     /// - use only types with `'static` lifetime bounds, i.e., those with no or only
//     /// `'static` references (both [`thread::Builder::spawn`][`Builder::spawn`]
//     /// and [`thread::spawn`][`spawn`] enforce this property statically)
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// #![feature(thread_spawn_unchecked)]
//     /// use std::thread;
//     ///
//     /// let builder = thread::Builder::new();
//     ///
//     /// let x = 1;
//     /// let thread_x = &x;
//     ///
//     /// let handler = unsafe {
//     ///     builder.spawn_unchecked(move || {
//     ///         println!("x = {}", *thread_x);
//     ///     }).unwrap()
//     /// };
//     ///
//     /// // caller has to ensure `join()` is called, otherwise
//     /// // it is possible to access freed memory if `x` gets
//     /// // dropped before the thread closure is executed!
//     /// handler.join().unwrap();
//     /// ```
//     ///
//     /// [`spawn`]: ../../std/thread/fn.spawn.html
//     /// [`Builder::spawn`]: ../../std/thread/struct.Builder.html#method.spawn
//     /// [`io::Result`]: ../../std/io/type.Result.html
//     /// [`JoinHandle`]: ../../std/thread/struct.JoinHandle.html

//     pub unsafe fn spawn_unchecked<'a, F, T>(self, f: F) -> Result<JoinHandle<T>,()> where
//         F: FnOnce() -> T, F: Send + 'a, T: Send + 'a
//     {
//         let Builder { name, stack_size } = self;

//         let stack_size = 2048; // stack_size.unwrap_or_else(thread::min_stack); // TODO define, REPLACE

//         let my_thread = Thread::new(name);
//         let their_thread = my_thread.clone();

//         let my_packet : Arc<UnsafeCell<Option<Result<T,()>>>>
//             = Arc::new(UnsafeCell::new(None));
//         let their_packet = my_packet.clone();

//         let main = move || {
//             if let Some(name) = their_thread.cname() {
//                 Thread::set_name(name);
//             }

//             // thread_info::set(guard::current(), their_thread); libstd::
// //            #[cfg(feature = "backtrace")]
//             // let try_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
//             //     crate::sys_common::backtrace::__rust_begin_short_backtrace(f)
//             // }));
// //            #[cfg(not(feature = "backtrace"))]
//             //let try_result = panic::catch_unwind(panic::AssertUnwindSafe(f));
            
//             //*their_packet.get() = Some(try_result);
//         };

//         Ok(JoinHandle(JoinInner {
//             // `Thread::new` takes a closure with a `'static` lifetime, since it's passed
//             // through FFI or otherwise used with low-level threading primitives that have no
//             // notion of or way to enforce lifetimes.
//             //
//             // As mentioned in the `Safety` section of this function's documentation, the caller of
//             // this function needs to guarantee that the passed-in lifetime is sufficiently long
//             // for the lifetime of the thread.
//             //
//             // Similarly, the `sys` implementation must guarantee that no references to the closure
//             // exist after the thread has terminated, which is signaled by `Thread::join`
//             // returning.
//             native: Some(Thread::new(
//                 stack_size,
//                 transmute::<Box<dyn FnOnce() + 'a>, Box<dyn FnOnce() + 'static>>(Box::new(
//                     main,
//                 )),
//             )?),
//             thread: my_thread,
//             packet: Packet(my_packet),
//         }))
//     }
// }
// // libstd/sys/unix/thread
// impl Thread {
//     // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    
//     pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>)
//                       -> Result<Thread,()> {
//         use core::mem;
//         use core::cmp;
//         use crate::threads::thread_bindings;
        
//         let p = box p;
//         let mut native: *mut nk_thread_id_t = mem::zeroed();
        
//         //assert_eq!(libc::pthread_attr_init(&mut attr), 0);

//         let stack_size = cmp::max(stack, 4096); // TODO

// //         match pthread_attr_setstacksize(&mut attr,
// //                                         stack_size) {
// //             0 => {}
// //             n => {
// //                 assert_eq!(n, libc::EINVAL);
// //                 // EINVAL means |stack_size| is either too small or not a
// //                 // multiple of the system page size.  Because it's definitely
// //                 // >= PTHREAD_STACK_MIN, it must be an alignment issue.
// //                 // Round up to the nearest page and try again.
// //                 let page_size = 4096;//os::page_size(); 
// //                 let stack_size = (stack_size + page_size - 1) &
// //                                  (-(page_size as isize - 1) as usize - 1);
// // //                assert_eq!(libc::pthread_attr_setstacksize(&mut attr,
// //   //                                                         stack_size), 0);
// //             }
// //         };

//         let ret = nk_thread_start(thread_start, &*p as *const _ as *mut _, null_mut(), 0,0, native, -1);
        
//             //let ret = libc::pthread_create(&mut native, &attr, thread_start,
//             //&*p as *const _ as *mut _);
//         //assert_eq!(libc::pthread_attr_destroy(&mut attr), 0);

//         return if ret != 0 {
//             Err(())
//         } else {
//             mem::forget(p); // ownership passed to pthread_create
//             Ok(Thread { id: native })
//         };

//         extern fn thread_start(main: *mut libc::c_void) -> *mut libc::c_void {
//             unsafe { sys_common::start_thread(main as *mut u8); }
//             null_mut()
//         }
//     }

//     pub fn yield_now() {
//         let ret = unsafe { libc::sched_yield() };
//         debug_assert_eq!(ret, 0);
//     }

//     #[cfg(any(target_os = "linux",
//               target_os = "android"))]
//     pub fn set_name(name: *const libc::c_char) {
//         const PR_SET_NAME: libc::c_int = 15;
//         // pthread wrapper only appeared in glibc 2.12, so we use syscall
//         // directly.
//         unsafe {
//             libc::prctl(PR_SET_NAME, name.as_ptr() as libc::c_ulong, 0, 0, 0);
//         }
//     }

//     #[cfg(any(target_os = "freebsd",
//               target_os = "dragonfly",
//               target_os = "openbsd"))]
//     pub fn set_name(name: *const libc::c_char) {
//         unsafe {
//             libc::pthread_set_name_np(libc::pthread_self(), name.as_ptr());
//         }
//     }

//     #[cfg(any(target_os = "macos", target_os = "ios"))]
//     pub fn set_name(name: &CStr) {
//         unsafe {
//             libc::pthread_setname_np(name.as_ptr());
//         }
//     }

//     #[cfg(target_os = "netbsd")]
//     pub fn set_name(name: &CStr) {
//         use crate::ffi::CString;
//         let cname = CString::new(&b"%s"[..]).unwrap();
//         unsafe {
//             libc::pthread_setname_np(libc::pthread_self(), cname.as_ptr(),
//                                      name.as_ptr() as *mut libc::c_void);
//         }
//     }
//     #[cfg(any(target_env = "newlib",
//               target_os = "solaris",
//               target_os = "haiku",
//               target_os = "l4re",
//               target_os = "emscripten",
//               target_os = "hermit"))]
//     pub fn set_name(_name: &CStr) {
//         // Newlib, Illumos, Haiku, and Emscripten have no way to set a thread name.
//     }
//     #[cfg(target_os = "fuchsia")]
//     pub fn set_name(_name: &CStr) {
//         // FIXME: determine whether Fuchsia has a way to set a thread name.
//     }

//     pub fn sleep(dur: Duration) {
//         use core::mem;
//         use core::cmp;

    
//         let mut secs = dur.as_secs();
//         let mut nsecs = dur.subsec_nanos() as _;

//         // If we're awoken with a signal then the return value will be -1 and
//         // nanosleep will fill in `ts` with the remaining time.
//         unsafe {
//             while secs > 0 || nsecs > 0 {
//                 let mut ts = libc::timespec {
//                     tv_sec: cmp::min(libc::time_t::max_value() as u64, secs) as libc::time_t,
//                     tv_nsec: nsecs,
//                 };
//                 secs -= ts.tv_sec as u64;
//                 if libc::nanosleep(&ts, &mut ts) == -1 {
//                     //assert_eq!(os::errno(), libc::EINTR);
//                     secs += ts.tv_sec as u64;
//                     nsecs = ts.tv_nsec;
//                 } else {
//                     nsecs = 0;
//                 }
//             }
//         }
//     }

//     pub fn join(self) {

//         use core::mem;
//         unsafe {
//             //let ret = libc::pthread_join(self.id, null_mut());
//             let ret = nk_join(self.id, null_mut());
//             mem::forget(self);
//             assert!(ret == 0,
//                     "failed to join thread: {}", Err(()));
//         }
//     }

//     pub fn id(&self) -> libc::pthread_t { self.id }

//     pub fn into_id(self) -> libc::pthread_t {
//         use core::mem;
//         let id = self.id;
//         mem::forget(self);
//         id
//     }
// }

// // Sys_common
// mod sys_common {
//     use alloc::sync::Arc;
//     use alloc::string::String;
//     use alloc::boxed::Box;
//     use core::cell::UnsafeCell;
//     use core::intrinsics::transmute;

//     #[allow(dead_code)]
//     pub unsafe fn start_thread(main: *mut u8) {
//         // Next, set up our stack overflow handler which may get triggered if we run                                       
//         // out of stack.                                                                                                   
//         // let _handler = stack_overflow::Handler::new(); HAHA idk what will happen here

//         // Finally, let's run some code.                                                                                   
//         Box::from_raw(main as *mut Box<dyn FnOnce()>)()
//     }
// } 

// mod modders {
//     pub struct Thread {
//         inner: Arc<Inner>,
//     }

//     impl Thread {
//         // Used only internally to construct a thread object without spawning
//         // Panics if the name contains nuls.
//         pub(crate) fn new(name: Option<String>) -> Thread {
//             let cname = name.map(|n| {
//                 CString::new(n).expect("thread name may not contain interior null bytes")
//             });
//             Thread {
//                 inner: Arc::new(Inner {
//                     name: cname,
//                     id: ThreadId::new(),
//                     state: AtomicUsize::new(EMPTY),
//                     lock: Mutex::new(()),
//                     cvar: Condvar::new(),
//                 })
//             }
//         }

//         /// Atomically makes the handle's token available if it is not already.
//         ///
//         /// Every thread is equipped with some basic low-level blocking support, via
//         /// the [`park`][park] function and the `unpark()` method. These can be
//         /// used as a more CPU-efficient implementation of a spinlock.
//         ///
//         /// See the [park documentation][park] for more details.
//         ///
//         /// # Examples
//         ///
//         /// ```
//         /// use std::thread;
//         /// use std::time::Duration;
//         ///
//         /// let parked_thread = thread::Builder::new()
//         ///     .spawn(|| {
//         ///         println!("Parking thread");
//         ///         thread::park();
//         ///         println!("Thread unparked");
//         ///     })
//         ///     .unwrap();
//         ///
//         /// // Let some time pass for the thread to be spawned.
//         /// thread::sleep(Duration::from_millis(10));
//         ///
//         /// println!("Unpark the thread");
//         /// parked_thread.thread().unpark();
//         ///
//         /// parked_thread.join().unwrap();
//         /// ```
//         ///
//         /// [park]: fn.park.html
//         #[stable(feature = "rust1", since = "1.0.0")]
//         pub fn unpark(&self) {
//             // To ensure the unparked thread will observe any writes we made
//             // before this call, we must perform a release operation that `park`
//             // can synchronize with. To do that we must write `NOTIFIED` even if
//             // `state` is already `NOTIFIED`. That is why this must be a swap
//             // rather than a compare-and-swap that returns if it reads `NOTIFIED`
//             // on failure.
//             match self.inner.state.swap(NOTIFIED, SeqCst) {
//                 EMPTY => return, // no one was waiting
//                 NOTIFIED => return, // already unparked
//                 PARKED => {} // gotta go wake someone up
//                 _ => panic!("inconsistent state in unpark"),
//             }

//             // There is a period between when the parked thread sets `state` to
//             // `PARKED` (or last checked `state` in the case of a spurious wake
//             // up) and when it actually waits on `cvar`. If we were to notify
//             // during this period it would be ignored and then when the parked
//             // thread went to sleep it would never wake up. Fortunately, it has
//             // `lock` locked at this stage so we can acquire `lock` to wait until
//             // it is ready to receive the notification.
//             //
//             // Releasing `lock` before the call to `notify_one` means that when the
//             // parked thread wakes it doesn't get woken only to have to wait for us
//             // to release `lock`.
//             drop(self.inner.lock.lock().unwrap());
//             self.inner.cvar.notify_one()
//         }

//         /// Gets the thread's unique identifier.
//         ///
//         /// # Examples
//         ///
//         /// ```
//         /// use std::thread;
//         ///
//         /// let other_thread = thread::spawn(|| {
//         ///     thread::current().id()
//         /// });
//         ///
//         /// let other_thread_id = other_thread.join().unwrap();
//         /// assert!(thread::current().id() != other_thread_id);
//         /// ```
//         #[stable(feature = "thread_id", since = "1.19.0")]
//         pub fn id(&self) -> ThreadId {
//             self.inner.id
//         }

//         /// Gets the thread's name.
//         ///
//         /// For more information about named threads, see
//         /// [this module-level documentation][naming-threads].
//         ///
//         /// # Examples
//         ///
//         /// Threads by default have no name specified:
//         ///
//         /// ```
//         /// use std::thread;
//         ///
//         /// let builder = thread::Builder::new();
//         ///
//         /// let handler = builder.spawn(|| {
//         ///     assert!(thread::current().name().is_none());
//         /// }).unwrap();
//         ///
//         /// handler.join().unwrap();
//         /// ```
//         ///
//         /// Thread with a specified name:
//         ///
//         /// ```
//         /// use std::thread;
//         ///
//         /// let builder = thread::Builder::new()
//         ///     .name("foo".into());
//         ///
//         /// let handler = builder.spawn(|| {
//         ///     assert_eq!(thread::current().name(), Some("foo"))
//         /// }).unwrap();
//         ///
//         /// handler.join().unwrap();
//         /// ```
//         ///
//         /// [naming-threads]: ./index.html#naming-threads
//         #[stable(feature = "rust1", since = "1.0.0")]
//         pub fn name(&self) -> Option<&str> {
//             self.cname().map(|s| unsafe { str::from_utf8_unchecked(s.to_bytes()) } )
//         }

//         fn cname(&self) -> Option<&CStr> {
//             self.inner.name.as_ref().map(|s| &**s)
//         }
//     }

// }
