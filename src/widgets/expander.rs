#![allow(dead_code)]

//! Expander / disclosure menu with a customizable header, an animated indicator
//! and opt-in hierarchical guide lines.
//!
//! The design borrows heavily from Flutter's `ExpansionTile` / `ListTile`:
//!
//! - the entire header row is tappable (pointer, keyboard and accessibility) instead of a
//!   bare text label;
//! - hover / active / focus feedback is painted by the widget itself;
//! - the disclosure indicator animates between its collapsed and expanded shape
//!   (Flutter's `RotationTransition`), rather than swapping glyphs;
//! - expanded state is *controlled externally* by the view-model, but the widget keeps a
//!   local animation clock so transitions stay smooth even when nothing else rebuilds;
//! - indentation is identical whether guide lines are drawn or not, so toggling lines never
//!   makes the layout jump.
//!
//! Beyond the high level composition API (`expander*`), this module also contains three
//! hand-written Masonry widgets (see [`imp`]) wrapped by small `xilem_core::View`s. This is the
//! "low level" path: it gives pixel/event-level control that pure view composition cannot
//! provide (custom paint, hover state, animation and accessibility).

use xilem::core::{MessageContext, MessageResult, Mut, View, ViewId, ViewMarker, ViewPathTracker};
use xilem::masonry::peniko::Color;
use xilem::masonry::properties::types::{AsUnit, CrossAxisAlignment, MainAxisAlignment};
use xilem::view::{FlexSpacer, flex_col, flex_row, label};
use xilem::{AnyWidgetView, Pod, ViewCtx, WidgetView};

// ---------------------------------------------------------------------------
// Customizable Indicators
// ---------------------------------------------------------------------------

/// Fine chevron indicator widget ("⌄ " when expanded, "› " when collapsed).
///
/// This is a static glyph; prefer [`chevron`] for the animated vector indicator that the
/// custom header uses by default.
pub fn chevron_indicator<State: 'static, Action: 'static>(
    expanded: bool,
) -> impl WidgetView<State, Action> + use<State, Action> {
    label(if expanded { "⌄ " } else { "› " })
}

/// Solid triangle indicator widget ("▾ " when expanded, "▸ " when collapsed).
pub fn triangle_indicator<State: 'static, Action: 'static>(
    expanded: bool,
) -> impl WidgetView<State, Action> + use<State, Action> {
    label(if expanded { "▾ " } else { "▸ " })
}

/// Plus/minus indicator widget ("[-] " when expanded, "[+] " when collapsed).
pub fn plus_minus_indicator<State: 'static, Action: 'static>(
    expanded: bool,
) -> impl WidgetView<State, Action> + use<State, Action> {
    label(if expanded { "[-] " } else { "[+] " })
}

// ---------------------------------------------------------------------------
// Guide Line Configuration
// ---------------------------------------------------------------------------

/// Configuration for hierarchical tree guide lines.
///
/// Every distance is measured from the left edge of the row. A row is laid out as
/// `padding_before -> line -> padding_after -> child content`, so the total gap
/// between the left edge and the children is [`GuideLineConfig::indent`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuideLineConfig {
    /// Color of the vertical guide line.
    pub color: Color,
    /// Width of the vertical line in logical pixels (1.0px hairline).
    pub width: f64,
    /// Padding between the row's left edge and the guide line.
    pub padding_before: f64,
    /// Padding between the guide line and the child content.
    pub padding_after: f64,
    /// Vertical gap between sibling children inside a content column.
    pub child_gap: f64,
}

impl Default for GuideLineConfig {
    fn default() -> Self {
        Self {
            // Very fine, subtle hairline matching dark IDE tree guidelines
            color: Color::from_rgba8(125, 135, 148, 70),
            width: 1.0,
            padding_before: 7.0,
            padding_after: 7.0,
            child_gap: 2.0,
        }
    }
}

impl GuideLineConfig {
    /// Create a guide line configuration with the default appearance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the guide line color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the guide line width in logical pixels.
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Set the padding between the row's left edge and the guide line.
    pub fn padding_before(mut self, padding: f64) -> Self {
        self.padding_before = padding;
        self
    }

    /// Set the padding between the guide line and the child content.
    pub fn padding_after(mut self, padding: f64) -> Self {
        self.padding_after = padding;
        self
    }

    /// Set the vertical gap between sibling children inside a content column.
    pub fn child_gap(mut self, gap: f64) -> Self {
        self.child_gap = gap;
        self
    }

    /// Set the total gap between the row's left edge and the child content.
    ///
    /// This derives the padding after the line, so the guide line stays where
    /// [`GuideLineConfig::padding_before`] placed it.
    pub fn indent(mut self, indent: f64) -> Self {
        self.padding_after = (indent - self.padding_before - self.width).max(0.0);
        self
    }

    /// Total gap between the row's left edge and the child content.
    ///
    /// This is `padding_before + width + padding_after`; it stays stable whether the
    /// guide line is shown or hidden so toggling lines never shifts the layout.
    pub fn total_indent(&self) -> f64 {
        self.padding_before + self.width + self.padding_after
    }
}

// ---------------------------------------------------------------------------
// Header style (Flutter `ListTile` / `ExpansionTile` inspired)
// ---------------------------------------------------------------------------

/// Visual configuration for the header row of a expander.
///
/// Mirrors the pieces of Flutter's `ListTile` that are most useful for a tree:
/// `tilePadding`, `dense` and the hover/splash/focus decorations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeaderStyle {
    /// Horizontal padding inside the header, before the indicator and after the label.
    pub padding_h: f64,
    /// Gap between the indicator (chevron) and the title.
    pub indicator_gap: f64,
    /// Alignment of the header children along the main (horizontal) axis.
    pub main_axis_alignment: MainAxisAlignment,
    /// Alignment of the header children along the cross (vertical) axis.
    pub cross_axis_alignment: CrossAxisAlignment,
    /// Background painted while the pointer hovers the header.
    pub hover_background: Color,
    /// Background painted while the header is pressed.
    pub active_background: Color,
    /// Ring painted around the header while it has keyboard focus.
    pub focus_ring: Color,
    /// Corner radius of the hover / active background.
    pub corner_radius: f64,
}

impl Default for HeaderStyle {
    fn default() -> Self {
        Self {
            padding_h: 4.0,
            indicator_gap: 6.0,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
            hover_background: Color::from_rgba8(0, 0, 0, 16),
            active_background: Color::from_rgba8(0, 0, 0, 28),
            focus_ring: crate::theme::ORANGE,
            corner_radius: 3.0,
        }
    }
}

impl HeaderStyle {
    /// A vertically dense header with tighter padding, similar to Flutter's `dense: true`.
    pub fn dense(mut self) -> Self {
        self.padding_h = 2.0;
        self
    }

    /// Set the horizontal padding inside the header.
    pub fn padding_h(mut self, padding: f64) -> Self {
        self.padding_h = padding;
        self
    }

    /// Set the gap between the indicator (chevron) and the title.
    pub fn indicator_gap(mut self, gap: f64) -> Self {
        self.indicator_gap = gap;
        self
    }

    /// Set the main (horizontal) axis alignment of the header children.
    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = alignment;
        self
    }

    /// Set the cross (vertical) axis alignment of the header children.
    pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = alignment;
        self
    }
}

// ---------------------------------------------------------------------------
// Low level Masonry widgets
// ---------------------------------------------------------------------------

/// Hand-written Masonry widgets backing the expander views.
///
/// These are deliberately kept public so they can be composed without the convenience
/// `expander*` helpers.
pub mod imp {
    use super::HeaderStyle;
    use xilem::masonry::accesskit::{self, Node, Role};
    use xilem::masonry::core::{
        AccessCtx, AccessEvent, BoxConstraints, ChildrenIds, CursorIcon, EventCtx, LayoutCtx,
        NewWidget, NoAction, PaintCtx, PointerButton, PointerButtonEvent, PointerEvent,
        PropertiesMut, PropertiesRef, QueryCtx, RegisterCtx, TextEvent, Update, UpdateCtx, Widget,
        WidgetMut, WidgetPod,
        keyboard::{Key, NamedKey},
    };
    use xilem::masonry::kurbo::{BezPath, Point, Rect, Size};
    use xilem::masonry::peniko::Color;
    use xilem::masonry::util::{fill_color, stroke};
    use xilem::masonry::vello;

    /// Default duration of the indicator rotation, in seconds.
    const CHEVRON_DURATION: f64 = 0.18;

    // -- GuideLine ----------------------------------------------------------

    /// A leaf widget that paints a single, very fine vertical hairline.
    ///
    /// Used to visualize nesting. It fills its cross axis so it stretches to the height of
    /// the row it is placed in.
    pub struct GuideLine {
        color: Color,
        width: f64,
    }

    impl GuideLine {
        /// Create a new guide line with the given `color` and `width`.
        pub fn new(color: Color, width: f64) -> Self {
            Self { color, width }
        }

        /// Set the line color.
        pub fn set_color(this: &mut WidgetMut<'_, Self>, color: Color) {
            if this.widget.color != color {
                this.widget.color = color;
                this.ctx.request_paint_only();
            }
        }

        /// Set the line width in logical pixels.
        pub fn set_width(this: &mut WidgetMut<'_, Self>, width: f64) {
            if this.widget.width != width {
                this.widget.width = width;
                this.ctx.request_layout();
            }
        }
    }

    impl Widget for GuideLine {
        type Action = NoAction;

        fn accepts_pointer_interaction(&self) -> bool {
            false
        }

        fn layout(
            &mut self,
            _ctx: &mut LayoutCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            bc: &BoxConstraints,
        ) -> Size {
            let width = self.width.max(0.0).min(bc.max().width);
            bc.constrain(Size::new(width, 0.0))
        }

        fn paint(
            &mut self,
            ctx: &mut PaintCtx<'_>,
            _props: &PropertiesRef<'_>,
            scene: &mut vello::Scene,
        ) {
            if self.width <= 0.0 {
                return;
            }
            let rect = Rect::new(0.0, 0.0, self.width, ctx.size().height);
            fill_color(scene, &rect, self.color);
        }

        fn register_children(&mut self, _ctx: &mut RegisterCtx<'_>) {}

        fn accessibility_role(&self) -> Role {
            Role::GenericContainer
        }

        fn accessibility(
            &mut self,
            _ctx: &mut AccessCtx<'_>,
            _props: &PropertiesRef<'_>,
            _node: &mut Node,
        ) {
        }

        fn children_ids(&self) -> ChildrenIds {
            ChildrenIds::new()
        }
    }

    // -- Chevron ------------------------------------------------------------

    /// An animated disclosure chevron.
    ///
    /// Points right when collapsed and rotates 90° down when expanded, interpolating
    /// smoothly on its own animation clock (Flutter's `RotationTransition`).
    pub struct Chevron {
        color: Color,
        size: f64,
        line_width: f64,
        expanded: bool,
        progress: f64,
    }

    impl Chevron {
        /// Create a chevron for the given state and style.
        pub fn new(expanded: bool, color: Color, size: f64, line_width: f64) -> Self {
            Self {
                color,
                size,
                line_width,
                expanded,
                progress: if expanded { 1.0 } else { 0.0 },
            }
        }

        /// Update the expanded state, kicking off (or reversing) the rotation animation.
        pub fn set_expanded(this: &mut WidgetMut<'_, Self>, expanded: bool) {
            if this.widget.expanded == expanded {
                return;
            }
            this.widget.expanded = expanded;
            this.ctx.request_anim_frame();
            this.ctx.request_paint_only();
        }

        /// Update the visual style of the chevron.
        pub fn set_style(this: &mut WidgetMut<'_, Self>, color: Color, size: f64, line_width: f64) {
            if this.widget.color != color {
                this.widget.color = color;
                this.ctx.request_paint_only();
            }
            if this.widget.size != size {
                this.widget.size = size;
                this.ctx.request_layout();
            }
            if this.widget.line_width != line_width {
                this.widget.line_width = line_width;
                this.ctx.request_paint_only();
            }
        }
    }

    impl Widget for Chevron {
        type Action = NoAction;

        fn accepts_pointer_interaction(&self) -> bool {
            false
        }

        fn on_anim_frame(
            &mut self,
            ctx: &mut UpdateCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            interval: u64,
        ) {
            let target = if self.expanded { 1.0 } else { 0.0 };
            if (self.progress - target).abs() <= f64::EPSILON {
                self.progress = target;
                return;
            }

            let step = (interval as f64 / 1e9) / CHEVRON_DURATION;
            if self.progress < target {
                self.progress = (self.progress + step).min(target);
            } else {
                self.progress = (self.progress - step).max(target);
            }

            // Keep animating until we reach the target.
            if (self.progress - target).abs() > f64::EPSILON {
                ctx.request_anim_frame();
            }
            ctx.request_paint_only();
        }

        fn layout(
            &mut self,
            _ctx: &mut LayoutCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            bc: &BoxConstraints,
        ) -> Size {
            let max = bc.max();
            let side = self.size.max(0.0).min(max.width).min(max.height);
            bc.constrain(Size::new(side, side))
        }

        fn paint(
            &mut self,
            ctx: &mut PaintCtx<'_>,
            _props: &PropertiesRef<'_>,
            scene: &mut vello::Scene,
        ) {
            let size = ctx.size();
            let (w, h) = (size.width, size.height);
            if w <= 0.0 || h <= 0.0 {
                return;
            }

            let cx = w / 2.0;
            let cy = h / 2.0;
            // 0 -> points right, 1 -> rotated a quarter turn clockwise, pointing down.
            let angle = self.progress * std::f64::consts::FRAC_PI_2;
            let (sin, cos) = angle.sin_cos();
            let rotate = |p: Point| {
                let dx = p.x - cx;
                let dy = p.y - cy;
                Point::new(cx + dx * cos - dy * sin, cy + dx * sin + dy * cos)
            };

            let tip = rotate(Point::new(w * 0.62, h * 0.50));
            let top = rotate(Point::new(w * 0.32, h * 0.24));
            let bottom = rotate(Point::new(w * 0.32, h * 0.76));

            let mut path = BezPath::new();
            path.move_to(top);
            path.line_to(tip);
            path.line_to(bottom);
            stroke(scene, &path, self.color, self.line_width);
        }

        fn register_children(&mut self, _ctx: &mut RegisterCtx<'_>) {}

        fn accessibility_role(&self) -> Role {
            Role::GenericContainer
        }

        fn accessibility(
            &mut self,
            _ctx: &mut AccessCtx<'_>,
            _props: &PropertiesRef<'_>,
            _node: &mut Node,
        ) {
        }

        fn children_ids(&self) -> ChildrenIds {
            ChildrenIds::new()
        }
    }

    // -- ExpanderHeader -------------------------------------------------------

    /// The action submitted by [`ExpanderHeader`] when it is activated.
    #[derive(Debug)]
    pub struct ExpanderToggle;

    /// A single-child, interactive container used as a expander header.
    ///
    /// It behaves like an accessible toggle button: pointer, keyboard and accessibility
    /// activation all submit [`ExpanderToggle`]. Hover, press and focus states are painted
    /// by the widget itself.
    pub struct ExpanderHeader {
        child: WidgetPod<dyn Widget>,
        expanded: bool,
        style: HeaderStyle,
    }

    impl ExpanderHeader {
        /// Create a header wrapping `child`.
        pub fn new(
            child: NewWidget<impl Widget + ?Sized>,
            expanded: bool,
            style: HeaderStyle,
        ) -> Self {
            Self {
                child: child.erased().to_pod(),
                expanded,
                style,
            }
        }

        /// Get mutable access to the wrapped header content.
        pub fn child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, dyn Widget> {
            this.ctx.get_mut(&mut this.widget.child)
        }

        /// Update the externally-controlled expanded state.
        pub fn set_expanded(this: &mut WidgetMut<'_, Self>, expanded: bool) {
            if this.widget.expanded != expanded {
                this.widget.expanded = expanded;
                this.ctx.request_accessibility_update();
            }
        }

        /// Update the header style.
        pub fn set_style(this: &mut WidgetMut<'_, Self>, style: HeaderStyle) {
            if this.widget.style != style {
                this.widget.style = style;
                this.ctx.request_layout();
                this.ctx.request_paint_only();
            }
        }

        fn activate(ctx: &mut EventCtx<'_>) {
            ctx.submit_action::<ExpanderToggle>(ExpanderToggle);
        }
    }

    impl Widget for ExpanderHeader {
        type Action = ExpanderToggle;

        fn accepts_focus(&self) -> bool {
            true
        }

        fn on_pointer_event(
            &mut self,
            ctx: &mut EventCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            event: &PointerEvent,
        ) {
            match event {
                PointerEvent::Down(..) => {
                    ctx.capture_pointer();
                    ctx.request_paint_only();
                }
                PointerEvent::Up(PointerButtonEvent { button, .. }) => {
                    if matches!(button, None | Some(PointerButton::Primary))
                        && ctx.is_active()
                        && ctx.is_hovered()
                    {
                        Self::activate(ctx);
                    }
                    ctx.request_paint_only();
                }
                _ => {}
            }
        }

        fn on_text_event(
            &mut self,
            ctx: &mut EventCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            event: &TextEvent,
        ) {
            if let TextEvent::Keyboard(event) = event
                && event.state.is_up()
                && (matches!(&event.key, Key::Character(c) if c == " ")
                    || event.key == Key::Named(NamedKey::Enter))
            {
                Self::activate(ctx);
            }
        }

        fn on_access_event(
            &mut self,
            ctx: &mut EventCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            event: &AccessEvent,
        ) {
            if event.action == accesskit::Action::Click {
                Self::activate(ctx);
            }
        }

        fn update(
            &mut self,
            ctx: &mut UpdateCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            event: &Update,
        ) {
            match event {
                Update::HoveredChanged(_)
                | Update::ActiveChanged(_)
                | Update::FocusChanged(_)
                | Update::DisabledChanged(_) => ctx.request_paint_only(),
                _ => {}
            }
        }

        fn register_children(&mut self, ctx: &mut RegisterCtx<'_>) {
            ctx.register_child(&mut self.child);
        }

        fn layout(
            &mut self,
            ctx: &mut LayoutCtx<'_>,
            _props: &mut PropertiesMut<'_>,
            bc: &BoxConstraints,
        ) -> Size {
            let pad = self.style.padding_h.max(0.0);
            let inner = BoxConstraints::new(
                Size::new((bc.min().width - 2.0 * pad).max(0.0), bc.min().height),
                Size::new((bc.max().width - 2.0 * pad).max(0.0), bc.max().height),
            );

            let child_size = ctx.run_layout(&mut self.child, &inner);
            let desired = Size::new(child_size.width + 2.0 * pad, child_size.height);
            let size = bc.constrain(desired);

            ctx.place_child(&mut self.child, Point::new(pad, 0.0));
            size
        }

        fn paint(
            &mut self,
            ctx: &mut PaintCtx<'_>,
            _props: &PropertiesRef<'_>,
            scene: &mut vello::Scene,
        ) {
            let size = ctx.size();
            if size.width <= 0.0 || size.height <= 0.0 {
                return;
            }
            let rect = Rect::from_origin_size(Point::ORIGIN, size)
                .to_rounded_rect(self.style.corner_radius);

            if ctx.is_active() {
                fill_color(scene, &rect, self.style.active_background);
            } else if ctx.is_hovered() {
                fill_color(scene, &rect, self.style.hover_background);
            }

            if ctx.is_focus_target() {
                stroke(scene, &rect, self.style.focus_ring, 1.0);
            }
        }

        fn accessibility_role(&self) -> Role {
            Role::Button
        }

        fn accessibility(
            &mut self,
            _ctx: &mut AccessCtx<'_>,
            _props: &PropertiesRef<'_>,
            node: &mut Node,
        ) {
            node.add_action(accesskit::Action::Click);
            node.set_expanded(self.expanded);
        }

        fn children_ids(&self) -> ChildrenIds {
            ChildrenIds::from_slice(&[self.child.id()])
        }

        fn get_cursor(&self, _ctx: &QueryCtx<'_>, _pos: Point) -> CursorIcon {
            CursorIcon::Pointer
        }
    }
}

// ---------------------------------------------------------------------------
// Xilem View wrappers around the low level widgets
// ---------------------------------------------------------------------------

/// An animated vector chevron indicator.
///
/// See [`imp::Chevron`] for the underlying widget.
pub fn chevron<State: 'static, Action: 'static>(
    expanded: bool,
    color: Color,
) -> Chevron<State, Action> {
    Chevron {
        expanded,
        color,
        size: 14.0,
        line_width: 1.6,
        phantom: std::marker::PhantomData,
    }
}

/// The [`View`] created by [`chevron`].
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct Chevron<State, Action> {
    expanded: bool,
    color: Color,
    size: f64,
    line_width: f64,
    phantom: std::marker::PhantomData<fn() -> (State, Action)>,
}

impl<State, Action> Chevron<State, Action> {
    /// Set the side length of the square the chevron is drawn in.
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Set the stroke width of the chevron.
    pub fn line_width(mut self, line_width: f64) -> Self {
        self.line_width = line_width;
        self
    }
}

impl<State, Action> ViewMarker for Chevron<State, Action> {}

impl<State, Action> View<State, Action, ViewCtx> for Chevron<State, Action>
where
    State: 'static,
    Action: 'static,
{
    type Element = Pod<imp::Chevron>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx, _: &mut State) -> (Self::Element, Self::ViewState) {
        let widget = imp::Chevron::new(self.expanded, self.color, self.size, self.line_width);
        (ctx.create_pod(widget), ())
    }

    fn rebuild(
        &self,
        prev: &Self,
        (): &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
        _: &mut State,
    ) {
        if self.expanded != prev.expanded {
            imp::Chevron::set_expanded(&mut element, self.expanded);
        }
        if self.color != prev.color || self.size != prev.size || self.line_width != prev.line_width
        {
            imp::Chevron::set_style(&mut element, self.color, self.size, self.line_width);
        }
    }

    fn teardown(
        &self,
        (): &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<'_, Self::Element>,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        (): &mut Self::ViewState,
        _message: &mut MessageContext,
        _element: Mut<'_, Self::Element>,
        _: &mut State,
    ) -> MessageResult<Action> {
        MessageResult::Nop
    }
}

/// A very fine vertical hairline used to visualize nesting.
pub fn guide_line<State: 'static, Action: 'static>(color: Color) -> GuideLine<State, Action> {
    GuideLine {
        color,
        width: 1.0,
        phantom: std::marker::PhantomData,
    }
}

/// The [`View`] created by [`guide_line`].
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct GuideLine<State, Action> {
    color: Color,
    width: f64,
    phantom: std::marker::PhantomData<fn() -> (State, Action)>,
}

impl<State, Action> GuideLine<State, Action> {
    /// Set the line width in logical pixels.
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }
}

impl<State, Action> ViewMarker for GuideLine<State, Action> {}

impl<State, Action> View<State, Action, ViewCtx> for GuideLine<State, Action>
where
    State: 'static,
    Action: 'static,
{
    type Element = Pod<imp::GuideLine>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx, _: &mut State) -> (Self::Element, Self::ViewState) {
        (
            ctx.create_pod(imp::GuideLine::new(self.color, self.width)),
            (),
        )
    }

    fn rebuild(
        &self,
        prev: &Self,
        (): &mut Self::ViewState,
        _ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
        _: &mut State,
    ) {
        if self.color != prev.color {
            imp::GuideLine::set_color(&mut element, self.color);
        }
        if self.width != prev.width {
            imp::GuideLine::set_width(&mut element, self.width);
        }
    }

    fn teardown(
        &self,
        (): &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<'_, Self::Element>,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        (): &mut Self::ViewState,
        _message: &mut MessageContext,
        _element: Mut<'_, Self::Element>,
        _: &mut State,
    ) -> MessageResult<Action> {
        MessageResult::Nop
    }
}

const HEADER_CONTENT_VIEW_ID: ViewId = ViewId::new(0);

/// An interactive expander header wrapping `content`.
///
/// Activation (pointer, keyboard or accessibility) calls `on_toggle`. This replaces the
/// generic `button` wrapper so the header can paint hover/active/focus feedback and expose
/// its expanded state to assistive technology.
pub fn expander_header<State, Action, V, F>(
    content: V,
    expanded: bool,
    on_toggle: F,
    style: HeaderStyle,
) -> ExpanderHeader<V, F, State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action>,
    F: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    ExpanderHeader {
        content,
        expanded,
        on_toggle,
        style,
        phantom: std::marker::PhantomData,
    }
}

/// The [`View`] created by [`expander_header`].
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct ExpanderHeader<V, F, State, Action> {
    content: V,
    expanded: bool,
    on_toggle: F,
    style: HeaderStyle,
    phantom: std::marker::PhantomData<fn() -> (State, Action)>,
}

impl<V, F, State, Action> ViewMarker for ExpanderHeader<V, F, State, Action> {}

impl<V, F, State, Action> View<State, Action, ViewCtx> for ExpanderHeader<V, F, State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action>,
    F: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    type Element = Pod<imp::ExpanderHeader>;
    type ViewState = V::ViewState;

    fn build(&self, ctx: &mut ViewCtx, app_state: &mut State) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = ctx.with_id(HEADER_CONTENT_VIEW_ID, |ctx| {
            View::<State, Action, _>::build(&self.content, ctx, app_state)
        });
        let widget = imp::ExpanderHeader::new(child.new_widget, self.expanded, self.style);
        (
            ctx.with_action_widget(|ctx| ctx.create_pod(widget)),
            child_state,
        )
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
        app_state: &mut State,
    ) {
        if self.expanded != prev.expanded {
            imp::ExpanderHeader::set_expanded(&mut element, self.expanded);
        }
        if self.style != prev.style {
            imp::ExpanderHeader::set_style(&mut element, self.style);
        }
        ctx.with_id(HEADER_CONTENT_VIEW_ID, |ctx| {
            View::<State, Action, _>::rebuild(
                &self.content,
                &prev.content,
                view_state,
                ctx,
                imp::ExpanderHeader::child_mut(&mut element).downcast(),
                app_state,
            );
        });
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
    ) {
        ctx.with_id(HEADER_CONTENT_VIEW_ID, |ctx| {
            View::<State, Action, _>::teardown(
                &self.content,
                view_state,
                ctx,
                imp::ExpanderHeader::child_mut(&mut element).downcast(),
            );
        });
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        message: &mut MessageContext,
        mut element: Mut<'_, Self::Element>,
        app_state: &mut State,
    ) -> MessageResult<Action> {
        match message.take_first() {
            Some(HEADER_CONTENT_VIEW_ID) => self.content.message(
                view_state,
                message,
                imp::ExpanderHeader::child_mut(&mut element).downcast(),
                app_state,
            ),
            None => match message.take_message::<imp::ExpanderToggle>() {
                Some(_) => MessageResult::Action((self.on_toggle)(app_state)),
                None => MessageResult::Stale,
            },
            _ => MessageResult::Stale,
        }
    }
}

// ---------------------------------------------------------------------------
// Expander Views
// ---------------------------------------------------------------------------

/// Expander with customizable indicator and header widgets.
///
/// Opt-in guide lines are disabled by default.
pub fn expander<State, Action, I, H, V, F1, F2>(
    indicator: I,
    header: H,
    expanded: bool,
    header_style: HeaderStyle,
    on_expand: F1,
    on_collapse: F2,
    content: V,
) -> impl WidgetView<State, Action> + use<State, Action, I, H, V, F1, F2>
where
    State: 'static,
    Action: 'static,
    I: WidgetView<State, Action>,
    H: WidgetView<State, Action>,
    V: WidgetView<State, Action>,
    F1: Fn(&mut State) -> Action + Send + Sync + 'static,
    F2: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    expander_with_line(
        indicator,
        header,
        expanded,
        false,
        None,
        header_style,
        on_expand,
        on_collapse,
        content,
    )
}

/// Expander with customizable indicator, header, and opt-in guide line visualization.
#[allow(clippy::too_many_arguments)]
pub fn expander_with_line<State, Action, I, H, V, F1, F2>(
    indicator: I,
    header: H,
    expanded: bool,
    show_line: bool,
    line_config: Option<GuideLineConfig>,
    header_style: HeaderStyle,
    on_expand: F1,
    on_collapse: F2,
    content: V,
) -> impl WidgetView<State, Action> + use<State, Action, I, H, V, F1, F2>
where
    State: 'static,
    Action: 'static,
    I: WidgetView<State, Action>,
    H: WidgetView<State, Action>,
    V: WidgetView<State, Action>,
    F1: Fn(&mut State) -> Action + Send + Sync + 'static,
    F2: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    let title = flex_row((indicator, header))
        .gap(header_style.indicator_gap.px())
        .main_axis_alignment(header_style.main_axis_alignment)
        .cross_axis_alignment(header_style.cross_axis_alignment);
    expander_styled(
        title,
        expanded,
        show_line,
        line_config,
        header_style,
        on_expand,
        on_collapse,
        content,
    )
}

/// Expander that accepts a complete custom header view and opt-in guide line.
pub fn expander_with_header_and_line<State, Action, H, V, F1, F2>(
    header_view: H,
    expanded: bool,
    show_line: bool,
    line_config: Option<GuideLineConfig>,
    on_expand: F1,
    on_collapse: F2,
    content: V,
) -> impl WidgetView<State, Action> + use<State, Action, H, V, F1, F2>
where
    State: 'static,
    Action: 'static,
    H: WidgetView<State, Action>,
    V: WidgetView<State, Action>,
    F1: Fn(&mut State) -> Action + Send + Sync + 'static,
    F2: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    expander_styled(
        header_view,
        expanded,
        show_line,
        line_config,
        HeaderStyle::default(),
        on_expand,
        on_collapse,
        content,
    )
}

/// Fully customizable expander: custom header view, guide line configuration and header style.
#[allow(clippy::too_many_arguments)]
pub fn expander_styled<State, Action, H, V, F1, F2>(
    header_view: H,
    expanded: bool,
    show_line: bool,
    line_config: Option<GuideLineConfig>,
    header_style: HeaderStyle,
    on_expand: F1,
    on_collapse: F2,
    content: V,
) -> impl WidgetView<State, Action> + use<State, Action, H, V, F1, F2>
where
    State: 'static,
    Action: 'static,
    H: WidgetView<State, Action>,
    V: WidgetView<State, Action>,
    F1: Fn(&mut State) -> Action + Send + Sync + 'static,
    F2: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    let config = line_config.unwrap_or_default();

    let toggle = expander_header(
        header_view,
        expanded,
        move |state: &mut State| {
            if expanded {
                on_collapse(state)
            } else {
                on_expand(state)
            }
        },
        header_style,
    );

    let body: Option<Box<AnyWidgetView<State, Action>>> = if expanded {
        if show_line {
            let line = guide_line::<State, Action>(config.color).width(config.width);

            let row = flex_row((
                FlexSpacer::Fixed(config.padding_before.px()),
                line,
                FlexSpacer::Fixed(config.padding_after.px()),
                content,
            ))
            .gap(0.px())
            .cross_axis_alignment(CrossAxisAlignment::Fill);

            Some(row.boxed())
        } else {
            // When lines are off, use the exact same total indent so layout never jumps.
            let row =
                flex_row((FlexSpacer::Fixed(config.total_indent().px()), content)).gap(0.px());
            Some(row.boxed())
        }
    } else {
        None
    };

    flex_col((toggle, body))
        .gap(0.px())
        .cross_axis_alignment(CrossAxisAlignment::Fill)
}

/// Legacy / convenience constructor with custom header view and no guide lines.
pub fn expander_with<State, Action, H, V, F1, F2>(
    header_view: H,
    expanded: bool,
    on_expand: F1,
    on_collapse: F2,
    content: V,
) -> impl WidgetView<State, Action> + use<State, Action, H, V, F1, F2>
where
    State: 'static,
    Action: 'static,
    H: WidgetView<State, Action>,
    V: WidgetView<State, Action>,
    F1: Fn(&mut State) -> Action + Send + Sync + 'static,
    F2: Fn(&mut State) -> Action + Send + Sync + 'static,
{
    expander_with_header_and_line(
        header_view,
        expanded,
        false,
        None,
        on_expand,
        on_collapse,
        content,
    )
}
