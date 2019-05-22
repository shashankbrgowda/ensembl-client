use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::{ Rc, Weak };
use std::sync::{ Arc, Mutex };

use serde_json::Value as JSONValue;
use stdweb::unstable::TryInto;
use stdweb::web::event::IEvent;
use stdweb::web::{ HtmlElement, Element, IHtmlElement, window };
use url::Url;

use controller::input::{
    register_startup_events, initial_actions, actions_run,
    run_direct_events,
    Timers
};
use controller::global::{ AppRunner, App, GlobalWeak, Global };
use data::{ BackendConfigBootstrap, HttpManager, BackendConfig };
use data::blackbox::{ BlackBox, BlackBoxDriver, HttpBlackBoxDriver, NullBlackBoxDriver };
use debug::{ DebugBling, create_interactors };
use dom::{ domutil, Bling, NoBling };
use dom::event::{ EventListener, Target, EventData, EventType, EventControl, ICustomEvent };

#[derive(Clone)]
struct BootingMissed(Arc<Mutex<Vec<(String,JSONValue)>>>);

struct BootingEventListener {
    missed: BootingMissed
}

impl BootingMissed {
    fn new() -> BootingMissed {
        BootingMissed(Arc::new(Mutex::new(Vec::<(String,JSONValue)>::new())))
    }
    
    fn add(&mut self, name: &str, details: JSONValue) {
        let mut v = self.0.lock().unwrap();
        v.push((name.to_string(),details));
    }
    
    fn run_missed(&mut self, app: &mut App) {
        let mut v = self.0.lock().unwrap();
        for (event,details) in v.drain(..) {
            match &event[..] {
                "bpane" => run_direct_events(app,&details),
                _ => ()
            };
        }
    }
}

impl BootingEventListener {
    fn new(missed: &BootingMissed) -> BootingEventListener {
        BootingEventListener {
            missed: missed.clone()
        }
    }
}

impl EventListener<()> for BootingEventListener {        
    fn receive(&mut self, _el: &Target,  e: &EventData, _idx: &()) {
        if let EventData::CustomEvent(_,_,_,c) = e {
            self.missed.add(&c.event_type(),c.details().unwrap());
        }
    }    
}

pub struct Booting {
    global: Global,
    http_manager: HttpManager,
    config_url: Url,
    el: HtmlElement,
    key: String,
    debug: bool,
    missed: BootingMissed,
    ec: EventControl<()>
}

impl Booting {
    pub fn new(g: &mut Global, http_manager: &HttpManager, config_url: &Url,
            el: &HtmlElement, key: &str, debug: bool) -> Booting {
        let missed = BootingMissed::new();
        let bel = BootingEventListener::new(&missed);
        let mut out = Booting {
            global: g.clone(),
            http_manager: http_manager.clone(),
            config_url: config_url.clone(),
            el: el.clone(),
            key: key.to_string(),
            debug,
            missed: missed.clone(),
            ec: EventControl::new(Box::new(bel),())
        };
        out.ec.add_event(EventType::CustomEvent("bpane".to_string()));
        out.ec.add_element(&el.clone().into(),());
        out
    }
    
    pub fn boot(&mut self, config: &BackendConfig) {
        console!("bootstrapping");
        let mut global = self.global.clone();
        let bling : Box<Bling> = if self.debug {
            Box::new(DebugBling::new(create_interactors()))
        } else { 
            Box::new(NoBling::new())
        };
        let debug_url = config.get_debug_url();
        let reporter_driver : Box<BlackBoxDriver> = if debug_url.is_some() {
            let debug_url = self.config_url.join(&debug_url.as_ref().unwrap()).ok().unwrap();
            console!("debug-url {:?}",debug_url);
            Box::new(HttpBlackBoxDriver::new(&self.http_manager,&debug_url))
        } else {
            Box::new(NullBlackBoxDriver::new())
        };
        let ar = AppRunner::new(
            &GlobalWeak::new(&global),&self.http_manager,
            &self.el,bling,&self.config_url,config,
            BlackBox::new(reporter_driver)
        );
        {
            global.register_app_now(&self.key,ar);
        }
        let key = self.key.clone();
        global.with_apprunner(&key,|ar| {
            let app = ar.clone().state();
            actions_run(&mut app.lock().unwrap(),&initial_actions());
            console!("fire retro");
            self.ec.reset();
            self.missed.run_missed(&mut app.lock().unwrap());
        });
    }
}
