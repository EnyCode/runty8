use crate::pico8::{Pico8, Pico8Impl};
use crate::ui::Element;
use crate::Event;
use crate::Resources;
use crate::{
    runtime::{draw_context::DrawData, state::InternalState},
    ui::DrawFn,
};
use std::fmt::Debug;

/// A regular pico8 app
pub trait App {
    fn init(state: &mut dyn Pico8) -> Self;
    fn update(&mut self, draw_context: &mut dyn Pico8);
    fn draw(&mut self, draw_context: &mut dyn Pico8);
}

/// An Elm-style app
// TODO: Add link to elm/explain what this is/decide if we even want this to be public
pub trait ElmApp {
    type Msg: Copy + Debug;
    fn init() -> Self;
    fn update(&mut self, msg: &Self::Msg, resources: &mut Resources);
    fn view(&mut self, resources: &Resources) -> Element<'_, Self::Msg>;
    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg>;
}

/// Wrapper structs
pub(crate) struct ElmAppCompat<A> {
    app: A,
}

impl<A: ElmApp> AppCompat for ElmAppCompat<A> {
    type Msg = A::Msg;

    fn init(_: &mut Resources, _: &InternalState, _: &mut DrawData) -> Self {
        Self { app: A::init() }
    }

    fn update(
        &mut self,
        msg: &Self::Msg,
        resources: &mut Resources,
        _: &InternalState,
        _: &mut DrawData,
    ) {
        self.app.update(msg, resources)
    }

    fn view(&mut self, resources: &mut Resources) -> Element<'_, Self::Msg> {
        self.app.view(resources)
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        self.app.subscriptions(event)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Pico8AppMsg {
    Tick,
}

pub(crate) struct Pico8AppCompat<A> {
    app: A,
}

impl<A: App> AppCompat for Pico8AppCompat<A> {
    type Msg = Pico8AppMsg;

    fn init(resources: &mut Resources, state: &InternalState, draw_data: &mut DrawData) -> Self {
        let pico8impl = Pico8Impl::new(draw_data, state, resources);

        Self {
            app: A::init(&mut pico8impl),
        }
    }

    fn update(
        &mut self,
        _: &Self::Msg,
        resources: &mut Resources,
        state: &InternalState,
        draw_data: &mut DrawData,
    ) {
        let pico8impl = Pico8Impl::new(draw_data, state, resources);
        self.app.update(&mut pico8impl);
    }

    fn view(&mut self, _: &mut Resources) -> Element<'_, Self::Msg> {
        DrawFn::new(|draw| self.app.draw(draw)).into()
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        if let Event::Tick { .. } = event {
            vec![Pico8AppMsg::Tick]
        } else {
            vec![]
        }
    }
}

/// Not intended for direct use.
pub(crate) trait AppCompat {
    type Msg: Copy + Debug;
    fn init(resources: &mut Resources, state: &InternalState, draw_data: &mut DrawData) -> Self;
    fn update(
        &mut self,
        msg: &Self::Msg,
        resources: &mut Resources,
        state: &InternalState,
        draw_data: &mut DrawData,
    );
    fn view(&mut self, resources: &mut Resources) -> Element<'_, Self::Msg>;
    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg>;
}
