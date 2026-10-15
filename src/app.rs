//! Root view: composes the widgets and wires the HTTP worker to the view-model.

use xilem::WidgetView;
use xilem::core::fork;
use xilem::masonry::properties::types::AsUnit;

use xilem::view::{flex_col, split, worker};

use crate::model::{RequestOutcome, RequestTask, perform_request};
use crate::viewmodel::App;
use crate::widgets::{sidebar, title_bar, workspace};

pub fn app_logic(state: &mut App) -> impl WidgetView<App> + use<> {
    let root = flex_col((
        title_bar(),
        split(sidebar(state), workspace(state))
            .split_point(0.22)
            .min_size(180.px(), 400.px()),
    ))
    .must_fill_major_axis(true)
    .cross_axis_alignment(xilem::view::CrossAxisAlignment::Start);

    fork(
        root,
        worker(
            |proxy, mut rx: tokio::sync::mpsc::UnboundedReceiver<RequestTask>| async move {
                while let Some(task) = rx.recv().await {
                    let tab_id = task.tab_id;
                    let proxy = proxy.clone();
                    tokio::task::spawn(async move {
                        let response = perform_request(task).await;
                        drop(proxy.message(RequestOutcome { tab_id, response }));
                    });
                }
            },
            |state: &mut App, sender| {
                state.attach_sender(sender);
            },
            |state: &mut App, outcome: RequestOutcome| {
                state.on_response(outcome);
            },
        ),
    )
}
