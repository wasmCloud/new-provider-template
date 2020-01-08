
#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::OP_CONFIGURE;
use wascc_codec::core::CapabilityConfiguration;

use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

capability_provider!({{project-name | capitalize }}Provider, {{project-name | capitalize }}Provider::new);

const CAPABILITY_ID: &str = "new:{{project-name}}"; // TODO: change this to an appropriate capability ID

pub struct {{project-name | capitalize }}Provider {
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
}

impl Default for {{project-name | capitalize }}Provider {
    fn default() -> Self {
        env_logger::init();

        {{project-name | capitalize}}Provider {            
        }
    }
}

impl {{project-name | capitalize}}Provider {
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

impl CapabilityProvider for {{project-name | capitalize}}Provider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "New {{ project-name | capitalize }} Capability Provider" // TODO: change this friendly name
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {            
            OP_CONFIGURE if actor == "system" => self.configure(msg.to_vec().as_ref()),            
            _ => Err("bad dispatch".into()),
        }
    }
}
