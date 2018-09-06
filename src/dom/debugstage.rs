use std::cmp::Ord;
use std::collections::HashMap;
use stdweb::traits::IEvent;
use std::cell::RefCell;
use stdweb::web::IEventTarget;
use stdweb::web::event::ChangeEvent;
use stdweb::web::html_element::SelectElement;
use stdweb::unstable::TryInto;
use domutil;

pub struct DebugFolderEntry {
    name: String,
    contents: String
}

impl DebugFolderEntry {
    pub fn new(name: &str) -> DebugFolderEntry {
        DebugFolderEntry {
            name: name.to_string(),
            contents: String::new()
        }
    }
    
    pub fn set(&mut self, value: &str) {
        self.contents = value.to_string();
    }
    
    pub fn get(&mut self) -> String {
        self.contents.clone()
    }
}

pub struct DebugPanel {
    folder: HashMap<String,DebugFolderEntry>,
    selected: String,
}

const DEBUG_FOLDER : &str = "- debug folder -";

impl DebugPanel {
    pub fn new() -> DebugPanel {
        let mut out = DebugPanel {
            folder: HashMap::<String,DebugFolderEntry>::new(),
            selected: DEBUG_FOLDER.to_string()
        };
        out.add_event();
        out.update_contents();
        out
    }
    
    fn select(&mut self, name: &str) {
        self.selected = name.to_string();
        self.update_contents();
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
    
    fn update_contents(&mut self) {
        let panel_el = domutil::query_select("#console .content");
        let sel = self.selected.clone();
        let entry = self.get_entry(&sel);
        domutil::text_content(&panel_el,&entry.contents);
    }
    
    fn render_dropdown(&self) {
        let sel_el = domutil::query_select("#console .folder");
        domutil::inner_html(&sel_el,"");
        let mut keys : Vec<&DebugFolderEntry> = self.folder.values().collect();
        keys.sort_by(|a,b| a.name.cmp(&b.name));
        for e in keys {
            let opt_el = domutil::append_element(&sel_el,"option");
            domutil::text_content(&opt_el,&e.name);
        }
    }
    
    pub fn get_entry(&mut self, name: &str) -> &mut DebugFolderEntry {
        if let None = self.folder.get(name) {
            self.folder.insert(name.to_string(),DebugFolderEntry::new(name));
            self.render_dropdown();
        }
        self.folder.get_mut(name).unwrap()
    }
}

const STAGE : &str = r##"
<div id="bpane-container">
    <div id="bpane-canv">
        <canvas id="glcanvas"></canvas>
    </div>
    <div id="bpane-right">
        <div id="console">
            <select class="folder"></select>
            <pre class="content"></pre>
        </div>
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

pub fn setup_stage_debug() {
    domutil::inner_html(&domutil::query_select("#stage"),STAGE);
    let el = domutil::append_element(&domutil::query_select("head"),"style");
    domutil::inner_html(&el,STAGE_CSS);
    debug_panel.with(|p| {
        *p.borrow_mut() = Some(DebugPanel::new());
    });
}

thread_local! {
    static debug_panel : RefCell<Option<DebugPanel>> = RefCell::new(None);
}

pub fn debug_panel_entry_get(name: &str) -> String {
    debug_panel.with(|p| {
        let mut po = p.borrow_mut();
        if let Some(panel) = po.as_mut() {
            panel.get_entry(name).get()
        } else {
            "".to_string()
        }
    })
}

pub fn debug_panel_entry_set(name: &str, value: &str) {
    debug_panel.with(|p| {
        let mut po = p.borrow_mut();
        if let Some(panel) = po.as_mut() {
            panel.get_entry(name).set(value);
            panel.update_contents();
        }
    });
}

pub fn debug_panel_select(name: &str) {
    debug_panel.with(|p| {
        let mut po = p.borrow_mut();
        if let Some(panel) = po.as_mut() {
            panel.select(name);
        }
    });
}
