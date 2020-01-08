
#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{OP_CONFIGURE};
use std::collections::HashMap;
use wascc_codec::core::CapabilityConfiguration;

use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

capability_provider!({{project-name | capitalize }}, {{project-name | capitalize }}::new);

const CAPABILITY_ID: &str = "new:provider"; // TODO: change this to your capability ID

pub struct {{project-name | capitalize }} {    
}

impl Default for {{project-name | capitalize }} {
    fn default() -> Self {
        env_logger::init();

        {{project-name}} {            
        }
    }
}

impl {{project-name | capitalize}} {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        config: impl Into<CapabilityConfiguration>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let _config = config.into();

        Ok(vec![])
    }    
}

impl CapabilityProvider for {{project-name}} {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "New Capability Provider"
    }

    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", op, actor);

        match op {            
            OP_CONFIGURE if actor == "system" => self.configure(msg.to_vec().as_ref()),            
            _ => Err("bad dispatch".into()),
        }
    }
}
