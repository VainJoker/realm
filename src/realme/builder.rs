use std::{sync::{Arc, Mutex}, thread};

use super::{cache::RealmeCache, watcher::{Channel, Event}, Realme};
use crate::{adaptor::source::SourceType, Adaptor, RealmeError, RealmeResult, Value};

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
    // pub fn build(mut self) -> Result<Realme, RealmeError> {
    //     let (tx1, rx1) =
    //         crossbeam::channel::unbounded::<super::watcher::Event>();
    //     let (tx2, rx2) =
    //         crossbeam::channel::unbounded::<super::watcher::Event>();

    //     eprintln!("111");
    //     let mut realme = self.parse()?;
    //     eprintln!("222");
    //     let builder_clone = Arc::new(Mutex::new(self));
    //     thread::spawn(move || {
    //         match rx1.recv() {
    //             Ok(Event::Changed) => {
    //                 tx2.send(Event::Stopped).unwrap();
    //                 let mut builder = builder_clone.lock().unwrap();
    //                 realme = builder.parse().unwrap();
    //                 // return self.build();
    //             },
    //             Ok(e) => {
    //                 RealmeError::Unknown(format!("{:?}", e));
    //             }
    //             Err(e) => {
    //                 RealmeError::Unknown(format!("{:?}", e));
    //             }
    //             }
    //     });
    //     eprintln!("555");
    //     Ok(realme)
    // }
    pub fn build(mut self) -> Result<SharedRealme, RealmeError> {
        let (tx1, rx1) = crossbeam::channel::unbounded::<super::watcher::Event>();
        let (tx2, rx2) = crossbeam::channel::unbounded::<super::watcher::Event>();
        
        let realme = self.parse((tx1.clone(), rx2.clone()))?;
        let shared_realme = SharedRealme::new(realme);
        let realme_clone = shared_realme.inner.clone();
        
        let builder_arc = Arc::new(Mutex::new(self));
        
        thread::spawn(move || {
            loop {
                match rx1.recv() {
                    Ok(Event::Changed) => {
                        eprintln!("Changed");
                        let mut builder = builder_arc.lock().expect("Failed to lock builder");
                        match builder.parse((tx1.clone(), rx2.clone())) {
                            Ok(new_realme) => {
                                eprintln!("new_realme: {:?}", new_realme);
                                let mut realme = realme_clone.lock().expect("Failed to lock realme");
                                *realme = new_realme;
                            },
                            Err(e) => eprintln!("Failed to parse: {:?}", e),
                        }
                        tx2.send(Event::Stopped).unwrap();
                    },
                    Ok(Event::Stopped) => break,
                    Ok(e) => eprintln!("Unexpected event: {:?}", e),
                    Err(e) => eprintln!("Channel error: {:?}", e),
                }
            }
        });
    
        Ok(shared_realme)
    }

    
    fn parse(&mut self, chan: Channel) -> RealmeResult<Realme> {
        let mut cache = RealmeCache::new();
    
        self.adaptors.sort_by(|a, b| a.priority.cmp(&b.priority));
        for adaptor in self.adaptors.iter().rev() {
            if adaptor.source_type() == SourceType::Env {
                cache.handle_adaptor(adaptor, true)?;
            } else {
                cache.handle_adaptor(adaptor, false)?;
            }
            adaptor.watcher(chan.clone());
        }
    
        Ok(Realme {
            cache: Value::Table(cache.cache),
            builder: None,
        })
    }
}

#[derive(Debug)]
pub struct SharedRealme {
    inner: Arc<Mutex<Realme>>,
}

impl SharedRealme {
    pub fn new(realme: Realme) -> Self {
        Self {
            inner: Arc::new(Mutex::new(realme)),
        }
    }

    pub fn get(&self) -> std::sync::MutexGuard<Realme> {
        self.inner.lock().expect("Failed to lock realme")
    }
}