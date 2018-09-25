use std::iter;
use std::sync::{ Mutex, Arc };
use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use serde_json::Value as JSONValue;

use stdweb::web::{ IEventTarget, Element, HtmlElement, document };
use stdweb::traits::IEvent;
use stdweb::web::event::{ ClickEvent, ChangeEvent };
use stdweb::web::html_element::SelectElement;
use stdweb::unstable::TryInto;

use controller::global::Global;
use dom::domutil;
use dom::event::{
    EventListener, EventControl, EventType, EventData, ICustomEvent,
    Target
};
use debug::testcards;
use debug::pane::console::DebugConsole;
use debug::pane::buttons::{ DebugButtons, ButtonAction };
use types::Todo;

pub struct BodyEventListener {
    val: u32,
}

impl BodyEventListener {
    fn new() -> BodyEventListener {
        BodyEventListener {
            val: 0
        }
    }
}

impl EventListener<()> for BodyEventListener {    
    fn receive(&mut self, el: &Target, ev: &EventData, _p: &()) {
        self.val += 1;
        debug!("test event","{} {:?} {:?}",self.val,el,ev);
        if let EventData::CustomEvent(_,n,_) = ev {
            if n == "bpane-start" {
                setup_stage_debug();
            }
        }
    }
}

pub struct DebugPanelListener {
    panel: Weak<DebugPanel>
}

impl EventListener<()> for DebugPanelListener {
    fn receive(&mut self, _el: &Target, ev: &EventData, _p: &()) {
        if let EventData::CustomEvent(_,n,e) = ev {
            if n == "refresh" {
                let mut keys = None;
                let detail = e.details();        
                if let Some(panel) = self.panel.upgrade() {
                    if let Some(JSONValue::Object(m)) = detail {
                        if let Some(JSONValue::Array(ref keys_in)) = m.get("keys") {
                            keys = Some(Vec::<String>::new());
                            for k in keys_in {
                                if let JSONValue::String(k) = k {
                                    keys.as_mut().unwrap().push(k.to_string());
                                }
                            }
                        }
                    }
                    if let Some(keys) = keys {
                        panel.render_dropdown(keys);
                    }
                }
            }
        }
    }
}

pub struct DebugPanelImpl {
    base: Element,
    console2: Option<DebugConsole>,
    buttons: DebugButtons,
    evc: Option<EventControl<()>>,
}

impl DebugPanelImpl {
    pub fn new(base: &Element) -> Rc<RefCell<DebugPanelImpl>> {
        debug!("global","new debug panel");
        debug!("debug panel","new debug panel");
        let mut bec = EventControl::new(Box::new(BodyEventListener::new()),());
        bec.add_event(EventType::KeyPressEvent);
        bec.add_event(EventType::MouseClickEvent);
        bec.add_event(EventType::CustomEvent("custom".to_string()));
        bec.add_event(EventType::CustomEvent("dropdown".to_string()));
        bec.add_event(EventType::CustomEvent("bpane-start".to_string()));
        bec.add_element(&domutil::query_select("body"),());
        Rc::new(RefCell::new(DebugPanelImpl {
            base: base.clone(),
            evc: None,
            buttons: DebugButtons::new(),
            console2: None
        }))
    }
    
    pub fn init(&mut self, p: Weak<DebugPanel>) {
        let cons_el = domutil::query_selector(&self.base,"#console2");
        self.console2 = Some(DebugConsole::new(&cons_el,&self.base));
        self.console2.as_mut().unwrap().select("hello");
        self.console2.as_mut().unwrap().add("hello","world");
        self.evc = Some(EventControl::new(Box::new(DebugPanelListener{ panel: p.clone() }),()));
        self.evc.as_mut().unwrap().add_event(EventType::CustomEvent("refresh".to_string()));
        self.evc.as_mut().unwrap().add_element(&self.base,());
        self.add_event();
    }
     
    fn render_dropdown(&self, keys: &Vec<String>) {
        let mut keys : Vec<String> = keys.iter().map(|s| s.to_string()).collect();
        let sel_el = domutil::query_select("#console .folder");
        domutil::inner_html(&sel_el,"");
        if self.console2.is_some() {
            keys.sort();
            keys.splice(..0,iter::once("- select -".to_string()));
            for e in keys {
                let opt_el = domutil::append_element(&sel_el,"option");
                domutil::text_content(&opt_el,&e);
            }
        }
    }

    fn add_event(&mut self) {
        let sel_el = domutil::query_select("#console .folder");
        sel_el.add_event_listener(|e: ChangeEvent| {
            let node : SelectElement = e.target().unwrap().try_into().ok().unwrap();
            if let Some(name) = node.value() {
                debug_panel_select(&name);
            }
        });
    }
    
    fn clear_buttons(&mut self) {
        self.buttons.clear_buttons();
    }

    fn render_buttons(&mut self) {
        self.buttons.render_buttons();
    }
    
    fn add_button(&mut self, name: &str, cb: Rc<RefCell<ButtonAction>>) {
        self.buttons.add_button(name,cb);        
    }
    
    fn trigger_button(&mut self, idx: usize) {
        self.buttons.trigger_button(idx);
    }
}

pub struct DebugPanel(Todo<Rc<RefCell<DebugPanelImpl>>>);

impl DebugPanel {
    fn new(base: &Element) -> Rc<DebugPanel> {
        let out = Rc::new(DebugPanel(Todo::new(DebugPanelImpl::new(base))));
        let q = Rc::downgrade(&out.clone());
        out.0.run(move |p| p.borrow_mut().init(q.clone()));
        out
    }
    
    fn render_dropdown(&self, keys: Vec<String>) {
        self.0.run(move |p| p.borrow_mut().render_dropdown(&keys));
    }
    
    fn clear_buttons(&self) {
        self.0.run(|p| p.borrow_mut().clear_buttons());
    }
    
    fn render_buttons(&self) {
        self.0.run(|p| { p.borrow_mut().render_buttons(); });
    }

    fn add_button(&self, name: &str, cb: Rc<RefCell<ButtonAction>>) {
        let name = name.to_string();
        self.0.run(move |p| { p.borrow_mut().add_button(&name,cb.clone())});
    }
    
    fn trigger_button(&self, idx: usize) {
        self.0.run(move |p| { p.borrow_mut().trigger_button(idx) });
    }
}

const STAGE : &str = r##"
<div id="bpane-container">
    <div id="bpane-canv">
        <h1>Debug Mode</h1>
    </div>
    <div id="bpane-right">
        <div id="console">
            <select class="testcard">
                <option value="">- testcards -</option>
                <option value="polar">Polar Testcard</option>
                <option value="draw">Jank Testcard</option>
                <option value="onoff">On/Off Testcard</option>
                <option value="button">Button Testcard</option>
                <option value="text">Text Testcard</option>
            </select>
            <select class="folder"></select>
            <button class="mark">mark!</button>
            <button class="start">start</button>
            <pre id="console2" class="content"></pre>
        </div>
        <div class="buttons"></div>
        <div id="managedcanvasholder"></div>
    </div>
</div>
"##;

const STAGE_CSS : &str = r##"
html, body {
    margin: 0px;
    padding: 0px;
    height: 100%;
    width: 100%;
    overflow: hidden;
}
#bpane-container {
    display: flex;
    height: 100%;
}
#bpane-right {
    width: 20%;
}


#console .content {
    height: 85%;
    overflow: scroll;
    border: 1px solid #ccc;
}

#managedcanvasholder {
    display: block;
    border: 2px solid red;
    display: inline-block;
    overflow: scroll;
    width: 100%;
}

#bpane-canv canvas {
    height: 100%;
    width: 100%;
}

#bpane-canv {
    width: 80%;
    height: 100%;
}

#bpane-canv canvas {
    width: 100%;
    height: 100%;
}

#stage {
    height: 100%;
}

#console {
    height: 50%;
}
@import url('https://fonts.googleapis.com/css?family=Roboto');
"##;

thread_local! {
    static DEBUG_PANEL : RefCell<Option<Rc<DebugPanel>>> = RefCell::new(None);
}

fn setup_testcard(g: &Arc<Mutex<Global>>, name: &str) {
    debug!("global","setup testcard {}",name);
    if name.len() > 0 {
        g.lock().unwrap().reset();
        testcards::testcard(g.clone(),name);
    }
}

fn setup_events() {
    let pane_el: HtmlElement = domutil::query_select("#bpane-canv").try_into().unwrap();
    let g = Arc::new(Mutex::new(Global::new(&pane_el)));
    let sel_el = domutil::query_select("#console .testcard");
    sel_el.add_event_listener(move |e: ChangeEvent| {
        let node : SelectElement = e.target().unwrap().try_into().ok().unwrap();
        if let Some(name) = node.value() {
            debug_panel_buttons_clear();
            setup_testcard(&g,&name);
        }
    });
    let mark_el = domutil::query_select("#console .mark");
    mark_el.add_event_listener(|_e: ClickEvent| {
        debug_panel_entry_mark();
    });
}

pub fn setup_stage_debug() {
    if let Some(stage) = domutil::query_selector_new("#stage") {
        domutil::inner_html(&stage,STAGE);
        let el = domutil::append_element(&domutil::query_select("head"),"style");
        domutil::inner_html(&el,STAGE_CSS);
        DEBUG_PANEL.with(|p| {
            *p.borrow_mut() = Some(DebugPanel::new(&stage));
        });
        setup_events();
        let mark_el = domutil::query_select("#console .start");
        mark_el.add_event_listener(|_e: ClickEvent| {
            let cel = domutil::query_select("body");
            domutil::send_custom_event(&cel,"bpane-start",&json!({}));
        });
    }
}

fn panel_op(cb: &mut FnMut(&DebugPanel) -> ()) {
    DEBUG_PANEL.with(|p| {
        let p : &Option<Rc<DebugPanel>> = &p.borrow();
        if let Some(ref po) = &p {
            cb(&po);
        }
    })
}

#[allow(dead_code)]
pub fn debug_panel_entry_reset(name: &str) {
    let cel = domutil::query_select("#console2");
    domutil::send_custom_event(&cel,"reset",&json!({ "name": name }));
}

pub fn debug_panel_entry_mark() {
    let cel = domutil::query_select("#console2");
    domutil::send_custom_event(&cel,"mark",&json!({}));
}

pub fn debug_panel_entry_add(name: &str, value: &str) {
    let cel = domutil::query_selector_new("#console2");
    if let Some(cel) = cel {
        domutil::send_custom_event(&cel,"add",&json!({
            "name": name,
            "value": value
        }));
    }
}

pub fn debug_panel_select(name: &str) {
    let cel = domutil::query_select("#console2");
    domutil::send_custom_event(&cel,"select",&json!({ "name": name }));
}

pub fn debug_panel_buttons_clear() {
    panel_op(&mut |p| {
        p.clear_buttons();
        p.render_buttons();
    });
}

pub fn debug_panel_button_add(name: &str, cb: Rc<RefCell<ButtonAction>>) {
    DEBUG_PANEL.with(|p| {
        let p : &Option<Rc<DebugPanel>> = &p.borrow();
        if let Some(ref po) = &p {
            po.add_button(name,cb);
            po.render_buttons();
        }
    });
}

pub fn debug_panel_trigger_button(idx: usize) {
    DEBUG_PANEL.with(|p| {
        let p : &Option<Rc<DebugPanel>> = &p.borrow();
        if let Some(ref po) = &p {
            po.trigger_button(idx);
        }
    });
}

pub fn setup_global() {
    let mut bec = EventControl::new(Box::new(BodyEventListener::new()),());
    bec.add_event(EventType::CustomEvent("bpane-start".to_string()));
    bec.add_element(&domutil::query_select("body"),());
}
