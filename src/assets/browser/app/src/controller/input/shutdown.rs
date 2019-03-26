use std::sync::{ Arc, Mutex };

use stdweb::unstable::TryInto;
use url::Url;

use composit::StateValue;
use controller::global::Global;
use controller::input::Action;
use debug::DEMO_SOURCES;

use dom::domutil;
use dom::event::{ EventListener, EventType, EventData, EventControl, Target };
use dom::AppEventData;
use types::Dot;

pub struct ShutdownEventListener {
    g: Arc<Mutex<Global>>
}

impl ShutdownEventListener {
    pub fn new(g: &Arc<Mutex<Global>>) -> ShutdownEventListener {
        ShutdownEventListener {
            g: g.clone()
        }
    }    
}

impl EventListener<()> for ShutdownEventListener {
    fn receive(&mut self, _el: &Target,  e: &EventData, _idx: &()) {
        let mut g = expect!(self.g.lock());
        match e {
            EventData::GenericEvent(EventType::UnloadEvent,cx) => {
                g.destroy();
            },
            _ => ()
        }
    }
}

pub fn register_shutdown_events(g: &Arc<Mutex<Global>>) {
    let uel = ShutdownEventListener::new(g);
    let mut ec_start = EventControl::new(Box::new(uel),());
    ec_start.add_event(EventType::UnloadEvent);
    ec_start.add_element(&domutil::query_select("body"),());
}
