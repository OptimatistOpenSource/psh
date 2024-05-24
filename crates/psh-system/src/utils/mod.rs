// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
struct ResourceInner<T, F> {
    timestamp: Instant,
    resource: Option<T>,
    refresher: F,
}

impl<T, F> ResourceInner<T, F>
where
    F: FnMut() -> Result<T>,
{
    fn new(func: F) -> Self {
        Self {
            timestamp: Instant::now(),
            // we don't init resource here so new won't ever fail
            resource: None,
            refresher: func,
        }
    }

    fn update(&mut self) -> Result<()> {
        self.timestamp = Instant::now();
        self.resource = Some((self.refresher)()?);
        Ok(())
    }
}

impl<T, F> ResourceInner<T, F> {
    fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.resource.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Resource<T, F>(Arc<Mutex<ResourceInner<T, F>>>);

impl<T, F> Resource<T, F>
where
    F: FnMut() -> Result<T>,
{
    pub(crate) fn new(func: F) -> Self {
        Self(Arc::new(Mutex::new(ResourceInner::new(func))))
    }

    /// retrive the inner resource, interval should match the interval of user loop,
    /// and is treated as an hint of data retrival,
    /// any data within interval/10 would be considered new thus won't be updated
    pub(crate) fn get(&self, interval: Option<Duration>) -> Result<T>
    where
        T: Clone,
    {
        let now = Instant::now();
        let Ok(mut guard) = self.0.lock() else {
            return Err(Error::SyncError);
        };
        let is_outdated = match interval {
            Some(interval) => (now - guard.timestamp) * 10 > interval,
            None => true,
        };

        if is_outdated || guard.resource.is_none() {
            guard.update()?;
        }
        guard.get().ok_or(Error::EmptyValue)
    }
}

pub(crate) type Handle<T, E = Error> = Resource<T, fn() -> Result<T, E>>;
