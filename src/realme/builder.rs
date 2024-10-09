use std::{
    sync::{Arc, Mutex},
    thread,
};

use super::{
    Realme,
    cache::RealmeCache,
    shared::SharedRealme,
    watcher::{Channel, Event},
};
use crate::{
    Adaptor, RealmeError, RealmeResult, Value, adaptor::source::SourceType,
};

/// A builder for creating a `Realme` instance.
///
/// This struct collects adaptors from various sources and constructs a `Realme`
/// with a configured environment.
#[derive(Debug, Default)]
pub struct RealmeBuilder {
    adaptors: Vec<Adaptor>,
}

impl RealmeBuilder {
    /// Creates a new `RealmeBuilder` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an `Adaptor` to the builder based on its source type.
    ///
    /// This method takes ownership of the builder and returns it after
    /// modifying, allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `adaptor` - The `Adaptor` to be added to the builder.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let adaptor = Adaptor::new(...);
    /// let builder = RealmeBuilder::new().load(adaptor);
    /// ```
    #[must_use]
    pub fn load(mut self, adaptor: Adaptor) -> Self {
        self.adaptors.push(adaptor);
        self
    }

    /// Constructs the `Realme` from the added adaptors.
    ///
    /// This method attempts to build the `Realme` using the adaptors provided
    /// through the `load` method. It initializes a `RealmeCache` and
    /// populates it with the adaptors' data.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the `Realme` was successfully created, or an
    /// `Err` containing a `RealmeError` if an error occurred during the
    /// building process.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = RealmeBuilder::new().load(adaptor);
    /// let realme = builder.build().expect("Failed to build Realme");
    /// ```
    pub fn build(mut self) -> Result<Realme, RealmeError> {
        let realme = self.parse(&None)?;
        Ok(realme)
    }

    pub fn shared_build(mut self) -> Result<SharedRealme, RealmeError> {
        let (tx1, rx1) =
            crossbeam::channel::unbounded::<super::watcher::Event>();
        let (tx2, rx2) =
            crossbeam::channel::unbounded::<super::watcher::Event>();

        let realme = self.parse(&Some((tx1.clone(), rx2.clone())))?;
        let shared_realme = SharedRealme::new(realme);
        let realme_clone = shared_realme.inner.clone();

        let builder_arc = Arc::new(Mutex::new(self));

        thread::spawn(move || {
            loop {
                match rx1.recv() {
                    Ok(Event::Changed) => {
                        let mut builder =
                            builder_arc.lock().expect("Failed to lock builder");
                        match builder.parse(&Some((tx1.clone(), rx2.clone()))) {
                            Ok(new_realme) => {
                                let mut realme = realme_clone
                                    .lock()
                                    .expect("Failed to lock realme");
                                *realme = new_realme;
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse: {:?}", e);
                            }
                        }
                        tx2.send(Event::Stopped).unwrap();
                    }
                    Ok(e) => {
                        tracing::error!("Unexpected event: {:?}", e);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Channel error: {:?}", e);
                        break;
                    }
                }
            }
        });

        Ok(shared_realme)
    }

    fn parse(&mut self, chan: &Option<Channel>) -> RealmeResult<Realme> {
        let mut cache = RealmeCache::new();

        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        for adaptor in self.adaptors.iter().rev() {
            if adaptor.source_type() == SourceType::Env {
                cache.handle_adaptor(adaptor, true)?;
            } else {
                cache.handle_adaptor(adaptor, false)?;
            }
            if let Some(ref chan) = chan {
                adaptor.watcher(chan.clone());
            }
        }

        Ok(Realme {
            cache: Value::Table(cache.cache),
        })
    }
}
