use std::collections::HashMap;
use photon_decode::EventData;
use crate::albion::{DecodeError, EventType};

pub trait Decoder {
    type Output: std::fmt::Debug;
    fn decode(&self, data: &EventData) -> Result<Self::Output, DecodeError>;
}

pub trait Handler<T> {
    fn handle(&self, decoded: &T);
}

pub struct Registry {
    decoders: HashMap<EventType, Box<dyn Fn(&EventData) -> Result<Box<dyn std::any::Any>, DecodeError>>>,
    handlers: HashMap<EventType, Vec<Box<dyn Fn(&dyn std::any::Any)>>>,
    // decoders: HashMap<EventType, Box<dyn Decoder<Output = Box<dyn Any>>>>,
    // handlers: HashMap<EventType, Vec<Box<dyn Handler<Box<dyna Any>>>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            decoders: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    // Register a decoder for a specific EventType
    pub fn register_decoder<D: 'static + Decoder + Send + Sync>(
        &mut self,
        event_type: EventType,
        decoder: D,
    ) {
        self.decoders.insert(
            event_type,
            Box::new(move |event_data: &EventData| {
                decoder.decode(event_data).map(|output| Box::new(output) as Box<dyn std::any::Any>)
            }),
        );
    }

    // Register a handler for a specific EventType
    pub fn register_handler<T: 'static + Send + Sync>(
        &mut self,
        event_type: EventType,
        handler: impl Handler<T> + 'static,
    ) {
        self.handlers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(Box::new(move |decoded: &dyn std::any::Any| {
                if let Some(decoded) = decoded.downcast_ref::<T>() {
                    handler.handle(decoded);
                }
            }));
    }

    pub fn process_event(&self, data: &EventData) -> Result<(), DecodeError> {
        if data.code != 1 { return Ok(()); }

        // Try to parse the albion event type from this photon message
        let ev_type = match EventType::try_from(data) {
            Ok(ev_type) => ev_type,
            Err(DecodeError::ParameterMissing) => return Ok(()),
            Err(e) => return Err(e),
        };
        println!("{ev_type:?}");

        // Get the handlers for this event type if any exist. If not, we don't
        // have to waste time decoding it. Bail out.
        let handlers = match self.handlers.get(&ev_type) {
            Some(h) => h,
            None    => return Ok(()),
        };

        // Someone wants to handle this event, get the decoder
        let decoder = match self.decoders.get(&ev_type) {
            Some(d) => d,
            None => unimplemented!(
                "Handler registered for albion event without decoder: {:?}",
                ev_type)
        };

        // Decode the packet and invoke the handlers
        let decoded = decoder(data)?;
        handlers.iter().for_each(|handle| handle(decoded.as_ref()));
        Ok(())
    }
}
