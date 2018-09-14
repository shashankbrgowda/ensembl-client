mod debugstage;
mod console;
mod buttons;

pub use debug::pane::debugstage::{
    setup_stage_debug,
    debug_panel_button_add,
    debug_panel_entry_add,
    debug_panel_entry_reset,
    debug_panel_trigger_button,
};

pub use debug::pane::buttons::{
    ButtonActionImpl
};
