use std::rc::Rc;
use std::fmt;
use std::sync::{ Arc, Mutex };

use stdweb::serde::Serde;
use serde_json::Value as JSONValue;
use stdweb::Value as Value;
use stdweb::unstable::TryInto;
use stdweb::Reference;
use stdweb::web::{ window, Element };
use stdweb::web::event::{ IEvent, IUiEvent, IMouseEvent, IKeyboardEvent };
use types::{ CPixel, cpixel };

pub struct EventControl<T> {
    handle: Arc<Mutex<Box<EventListener<T>>>>,
    mappings: Vec<EventType>,
    window: ElementEvents<T>,
    current: Vec<ElementEvents<T>>
}

impl<T: 'static> EventControl<T> {
    pub fn new(handle: Box<EventListener<T>>, p: T) -> EventControl<T> {
        EventControl {
            handle: Arc::new(Mutex::new(handle)),
            mappings: Vec::<EventType>::new(),
            window: ElementEvents::<T>::new(&Target::Window,p),
            current: Vec::<ElementEvents<T>>::new()
        }
    }
    
    pub fn reset(&mut self) {
        for ee in &mut self.current {
            ee.clear();
        }
        self.current.clear();
        self.window.clear();
    }
    
    pub fn add_event(&mut self, typ: EventType) {
        if window_event(&typ) {
            self.window.add_event(&typ,&self.handle);
        } else {
            self.mappings.push(typ);
        }
    }
    
    pub fn add_element(&mut self, el: &Element, t: T) {
        let mut m = ElementEvents::new(&Target::Element(el.clone()),t);
        for name in &self.mappings {
            m.add_event(name,&self.handle);
        }
        self.current.push(m);
    }
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum EventType {
    MouseUpEvent,
    MouseDownEvent,
    MouseClickEvent,
    MouseWheelEvent,
    MouseMoveEvent,
    KeyPressEvent,
    ResizeEvent,
    CustomEvent(String)
}

fn window_event(et: &EventType) -> bool {
    match et {
        EventType::ResizeEvent => true,
        _ => false
    }
}

#[derive(Debug)]
pub enum EventData {
    MouseEvent(EventType,MouseData),
    KeyboardEvent(EventType,KeyboardData),
    CustomEvent(EventType,String,CustomData),
    GenericEvent(EventType),
}

impl EventData {
    fn new(et: EventType, e: Reference) -> EventData {
        let e = e.clone();
        match &et {
            EventType::MouseMoveEvent |
            EventType::MouseDownEvent |
            EventType::MouseUpEvent |
            EventType::MouseClickEvent |
            EventType::MouseWheelEvent =>
                EventData::MouseEvent(et.clone(),MouseData(e)),

            EventType::KeyPressEvent =>
                EventData::KeyboardEvent(et.clone(),KeyboardData(e)),
                
            EventType::CustomEvent(n) =>
                EventData::CustomEvent(et.clone(),n.clone(),CustomData(e)),
            EventType::ResizeEvent =>
                EventData::GenericEvent(et.clone())
        }
    }
}

impl EventType {
    fn get_name(&self) -> &str {
        match self {
            EventType::MouseClickEvent => "click",
            EventType::MouseDownEvent => "mousedown",
            EventType::MouseUpEvent => "mouseup",
            EventType::MouseMoveEvent => "mousemove",
            EventType::KeyPressEvent => "keypress",
            EventType::MouseWheelEvent => "wheel",
            EventType::ResizeEvent => "resize",
            EventType::CustomEvent(n) => &n
        }
    }
}

#[derive(ReferenceType,Clone,PartialEq,Eq)]
#[reference(instance_of = "MouseData")]
pub struct MouseData(Reference);

impl MouseData {
    pub fn at(&self) -> CPixel {
        cpixel(self.client_x(),self.client_y())
    }
    
    pub fn wheel_delta(&self) -> i32 {
        let delta : i32 = js! { return @{self.as_ref()}.deltaY; }.try_into().unwrap();
        let mode : i32 = js! { return @{self.as_ref()}.deltaMode; }.try_into().unwrap();
        match mode {
            0 => delta,
            1 => delta * 40,
            _ => delta * 800
        }
    }
}

impl fmt::Debug for MouseData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MouseData {{ x={} y={} }}",self.client_x(),self.client_y())
    }
}

impl IEvent for MouseData {}
impl IUiEvent for MouseData {}
impl IMouseEvent for MouseData {}

#[derive(ReferenceType,Clone,PartialEq,Eq)]
#[reference(instance_of = "KeyboardData")]
pub struct KeyboardData(Reference);

impl fmt::Debug for KeyboardData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyboardData {{ code={} }}",self.code())
    }
}

impl IEvent for KeyboardData {}
impl IUiEvent for KeyboardData {}
impl IKeyboardEvent for KeyboardData {}

pub trait ICustomEvent {
    fn details(&self) -> Option<JSONValue>;
}

#[derive(ReferenceType,Clone,PartialEq,Eq)]
#[reference(instance_of = "CustomData")]
pub struct CustomData(Reference);

impl fmt::Debug for CustomData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomData {{ {:?} }}",self.details())
    }
}

impl ICustomEvent for CustomData {
    fn details(&self) -> Option<JSONValue> {
        let js_val : Value = js! {
            return @{self.0.as_ref()}.detail;
        }.try_into().unwrap();
        let val : Serde<JSONValue> = js_val.try_into().ok().unwrap();
        Some(val.0)
    }
}

struct JsEventKiller {
    name: String,
    cb_js: Reference
}

#[derive(Clone,Debug)]
pub enum Target {
    Element(Element),
    Window
}

pub struct ElementEvents<T> {
    el: Target,
    kills: Vec<JsEventKiller>,
    payload: Rc<T>
}

impl<T: 'static> ElementEvents<T> {
    fn new(el: &Target, p: T) -> ElementEvents<T> {
        ElementEvents {
            el: el.clone(),
            kills: Vec::<JsEventKiller>::new(),
            payload: Rc::new(p)
        }
    }
    
    fn as_ref(&self) -> Reference {
        match &self.el {
            Target::Element(el) => el.as_ref().clone(),
            Target::Window => window().as_ref().clone()
        }
    }
    
    pub fn clear(&mut self) {
        for ek in &self.kills {
            let el = self.as_ref();
            let name = &ek.name;
            let cb_js = &ek.cb_js;
            js! { @(no_return)
               var cb = @{cb_js};
               @{el}.removeEventListener(@{&name},cb);
               cb.drop();
            };
        }
    }
    
    fn add_event(&mut self, typ: &EventType, evl: &Arc<Mutex<Box<EventListener<T>>>>) {
        let name = typ.get_name().to_string();
        let el = &self.el;
        let p = self.payload.clone();
        let typc = typ.clone();
        let cb = enclose! { (el,evl) move |e: Reference| {
            let ed = EventData::new(typc.clone(),e);
            evl.lock().unwrap().receive(&el,&ed,&p);
        }};
        let v;
        {
            let elr = self.as_ref();
            v = js! {
                var cb = @{cb};
                @{elr}.addEventListener(@{&name},cb);
                return cb;
            };
        };
        let cb_js : Reference = v.try_into().unwrap();
        self.kills.push(JsEventKiller {
            name: name.to_string(),
            cb_js
        });
    }
}

pub trait EventListener<T> {
    fn receive(&mut self, _el: &Target, _ev: &EventData, _p: &T) {}
}

pub fn disable_context_menu() {
    js! { document.addEventListener("contextmenu",function(e) {
            e.stopPropagation();
            e.preventDefault();
            return false;
        },false); };
}
