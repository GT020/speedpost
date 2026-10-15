//! A container that paints a background while it or one of its children is hovered.
//!
//! Masonry's `Button` only changes its *border* on hover; this wrapper adds a proper
//! hover highlight (used by the sidebar request leaf nodes) without changing the button's
//! own styling.

use std::marker::PhantomData;

use xilem::core::{MessageContext, MessageResult, Mut, View, ViewId, ViewMarker, ViewPathTracker};
use xilem::masonry::accesskit::{Node, Role};
use xilem::masonry::core::{
    AccessCtx, BoxConstraints, ChildrenIds, LayoutCtx, NewWidget, NoAction, PaintCtx,
    PropertiesMut, PropertiesRef, RegisterCtx, Update, UpdateCtx, Widget, WidgetMut, WidgetPod,
};
use xilem::masonry::kurbo::{Point, Rect, Size};
use xilem::masonry::peniko::Color;
use xilem::masonry::util::fill_color;
use xilem::masonry::vello;
use xilem::{Pod, ViewCtx, WidgetView};

/// A single-child container that paints `background` while it or its child is hovered.
pub struct HoverCard {
    child: WidgetPod<dyn Widget>,
    background: Color,
    radius: f64,
}

impl HoverCard {
    /// Create a hover container wrapping `child`.
    pub fn new(child: NewWidget<impl Widget + ?Sized>, background: Color, radius: f64) -> Self {
        Self {
            child: child.erased().to_pod(),
            background,
            radius,
        }
    }

    /// Get mutable access to the wrapped child.
    pub fn child_mut<'t>(this: &'t mut WidgetMut<'_, Self>) -> WidgetMut<'t, dyn Widget> {
        this.ctx.get_mut(&mut this.widget.child)
    }

    /// Update the hover background color.
    pub fn set_background(this: &mut WidgetMut<'_, Self>, background: Color) {
        if this.widget.background != background {
            this.widget.background = background;
            this.ctx.request_paint_only();
        }
    }
}

impl Widget for HoverCard {
    type Action = NoAction;

    fn accepts_pointer_interaction(&self) -> bool {
        false
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
        let child_size = ctx.run_layout(&mut self.child, &bc.loosen());
        let size = bc.constrain(child_size);
        ctx.place_child(&mut self.child, Point::ORIGIN);
        size
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx<'_>,
        _props: &PropertiesRef<'_>,
        scene: &mut vello::Scene,
    ) {
        if !ctx.has_hovered() {
            return;
        }
        let rect = Rect::from_origin_size(Point::ORIGIN, ctx.size()).to_rounded_rect(self.radius);
        fill_color(scene, &rect, self.background);
    }

    fn update(&mut self, ctx: &mut UpdateCtx<'_>, _props: &mut PropertiesMut<'_>, event: &Update) {
        match event {
            Update::HoveredChanged(_) | Update::ChildHoveredChanged(_) => {
                ctx.request_paint_only();
            }
            _ => {}
        }
    }

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
        ChildrenIds::from_slice(&[self.child.id()])
    }
}

const HOVER_CARD_CHILD: ViewId = ViewId::new(0);

/// Wrap `child` in a container that paints `background` while it is hovered.
pub fn hover_card<State: 'static, Action: 'static, V>(
    child: V,
    background: Color,
) -> HoverCardView<V, State, Action>
where
    V: WidgetView<State, Action>,
{
    HoverCardView {
        child,
        background,
        radius: 4.0,
        phantom: PhantomData,
    }
}

/// The [`View`] created by [`hover_card`].
#[must_use = "View values do nothing unless provided to Xilem."]
pub struct HoverCardView<V, State, Action> {
    child: V,
    background: Color,
    radius: f64,
    phantom: PhantomData<fn() -> (State, Action)>,
}

impl<V, State, Action> ViewMarker for HoverCardView<V, State, Action> {}

impl<V, State, Action> View<State, Action, ViewCtx> for HoverCardView<V, State, Action>
where
    State: 'static,
    Action: 'static,
    V: WidgetView<State, Action>,
{
    type Element = Pod<HoverCard>;
    type ViewState = V::ViewState;

    fn build(&self, ctx: &mut ViewCtx, state: &mut State) -> (Self::Element, Self::ViewState) {
        let (child, child_state) = ctx.with_id(HOVER_CARD_CHILD, |ctx| {
            View::<State, Action, _>::build(&self.child, ctx, state)
        });
        let widget = HoverCard::new(child.new_widget, self.background, self.radius);
        (ctx.create_pod(widget), child_state)
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
        state: &mut State,
    ) {
        if self.background != prev.background {
            HoverCard::set_background(&mut element, self.background);
        }
        ctx.with_id(HOVER_CARD_CHILD, |ctx| {
            View::<State, Action, _>::rebuild(
                &self.child,
                &prev.child,
                view_state,
                ctx,
                HoverCard::child_mut(&mut element).downcast(),
                state,
            );
        });
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'_, Self::Element>,
    ) {
        ctx.with_id(HOVER_CARD_CHILD, |ctx| {
            View::<State, Action, _>::teardown(
                &self.child,
                view_state,
                ctx,
                HoverCard::child_mut(&mut element).downcast(),
            );
        });
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        message: &mut MessageContext,
        mut element: Mut<'_, Self::Element>,
        state: &mut State,
    ) -> MessageResult<Action> {
        match message.take_first() {
            Some(HOVER_CARD_CHILD) => self.child.message(
                view_state,
                message,
                HoverCard::child_mut(&mut element).downcast(),
                state,
            ),
            _ => MessageResult::Stale,
        }
    }
}
