// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use crate::ops;
use crate::state::ThreadSafeState;
use crate::worker::Worker;
use crate::worker::WorkerChannels;
use deno_core;
use deno_core::ErrBox;
use deno_core::StartupData;
use futures::future::FutureExt;
use std::future::Future;
use std::ops::Deref;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// This worker is implementation of `Worker` Web API
///
/// At the moment this type of worker supports only
/// communication with parent and creating new workers.
///
/// Each `WebWorker` is either a child of `MainWorker` or other
/// `WebWorker`.
#[derive(Clone)]
pub struct WebWorker(Worker);

impl WebWorker {
  pub fn new(
    name: String,
    startup_data: StartupData,
    state: ThreadSafeState,
    external_channels: WorkerChannels,
  ) -> Self {
    let state_ = state.clone();
    let worker = Worker::new(name, startup_data, state_, external_channels);
    {
      let mut isolate = worker.isolate.try_lock().unwrap();
      ops::web_worker::init(&mut isolate, &state);
      ops::worker_host::init(&mut isolate, &state);
    }

    Self(worker)
  }
}

impl Deref for WebWorker {
  type Target = Worker;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for WebWorker {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Future for WebWorker {
  type Output = Result<(), ErrBox>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
    let inner = self.get_mut();
    inner.0.poll_unpin(cx)
  }
}
