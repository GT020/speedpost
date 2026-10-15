// Speedpost - a small Postman-style HTTP client built with Xilem.
// Layered as MVVM:
//   model/     - domain types + HTTP service
//   viewmodel/ - view-model: RequestVm / ResponseVm / HistoryVm + root App
//   widgets/   - view layer, one widget module per screen region
//   app.rs     - root view composition (wires worker to the view-model)
//   theme.rs   - palette
//   util.rs    - formatting helpers

mod app;
mod icons;
mod model;
mod theme;
mod util;
mod viewmodel;
mod widgets;

use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WindowOptions, Xilem};

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        viewmodel::App::new(),
        app::app_logic,
        WindowOptions::new("Speedpost").with_min_inner_size(xilem::dpi::LogicalSize::new(720., 480.)),
    )
    .with_font(icons::FONT.to_vec());
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}

mod storage;
