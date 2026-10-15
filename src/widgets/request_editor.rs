//! Request editor pane: switches between the Params / Headers / Body panels.

use xilem::WidgetView;
use xilem::core::one_of::OneOf3;

use crate::model::{KVKind, ReqTab};
use crate::viewmodel::{App, TabVm};
use crate::widgets::{body_editor, kv_editor};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    match tab.request().tab() {
        ReqTab::Params => OneOf3::A(kv_editor(tab, KVKind::Params)),
        ReqTab::Headers => OneOf3::B(kv_editor(tab, KVKind::Headers)),
        ReqTab::Body => OneOf3::C(body_editor(tab)),
    }
}
