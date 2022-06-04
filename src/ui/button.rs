use crate::{runtime::draw_context::DrawContext, Event, MouseButton};

use super::{DispatchEvent, Element, Widget};
use std::fmt::Debug;

pub struct Button<'a, Msg> {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    on_press: Option<Msg>,
    on_hover: Option<Msg>,
    state: &'a mut State,
    content: Element<'a, Msg>,
    active_mode: ActiveMode,
}

#[derive(PartialEq)]
enum ActiveMode {
    Release,
    Press,
}

#[derive(Debug, Clone)]
pub struct State {
    pressed: bool,
    mouse_pressed: bool,
    mouse_contained: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            pressed: false,
            mouse_pressed: false,
            mouse_contained: false,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Msg> Button<'a, Msg> {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        on_press: Option<Msg>,
        state: &'a mut State,
        content: impl Into<Element<'a, Msg>>,
    ) -> Self {
        Button {
            x,
            y,
            width,
            height,
            on_press,
            on_hover: None,
            state,
            content: content.into(),
            active_mode: ActiveMode::Release,
        }
    }

    pub fn event_on_press(mut self) -> Self {
        self.active_mode = ActiveMode::Press;

        self
    }

    pub fn on_hover(mut self, on_hover: Msg) -> Self {
        self.on_hover = Some(on_hover);

        self
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.x + self.width;
        let contains_y = y >= self.y && y < self.y + self.height;

        contains_x && contains_y
    }
}

impl<'a, Msg: Copy + Debug> Widget for Button<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut DispatchEvent<'_, Msg>,
    ) {
        use crate::MouseEvent::*;
        use Event::*;

        // TODO: Dispatch events for content?
        match event {
            Mouse(Down(MouseButton::Left)) => {
                self.state.mouse_pressed = true;

                if self.contains(cursor_position.0, cursor_position.1) {
                    if self.active_mode == ActiveMode::Press && !self.state.pressed {
                        if let Some(on_press) = self.on_press {
                            dispatch_event.call(on_press);
                        }
                    }

                    self.state.pressed = true;
                }
            }
            Mouse(Up(MouseButton::Left)) => {
                self.state.mouse_pressed = false;

                if self.contains(cursor_position.0, cursor_position.1)
                    && self.state.pressed
                    && self.active_mode == ActiveMode::Release
                {
                    if let Some(on_press) = self.on_press {
                        dispatch_event.call(on_press);
                    }
                }

                self.state.pressed = false;
            }
            Mouse(Move { .. }) => {
                let currently_contained = self.contains(cursor_position.0, cursor_position.1);

                if currently_contained && !self.state.mouse_contained {
                    if let Some(on_hover) = self.on_hover {
                        dispatch_event.call(on_hover);
                    }
                }

                self.state.mouse_contained = currently_contained;

                match self.active_mode {
                    ActiveMode::Press => {
                        if self.state.mouse_contained {
                            if self.state.mouse_pressed && !self.state.pressed {
                                self.state.pressed = true;
                                if let Some(on_press) = self.on_press {
                                    dispatch_event.call(on_press);
                                }
                            }
                        } else {
                            self.state.pressed = false;
                        }
                    }
                    ActiveMode::Release => {}
                };
            }
            _ => {}
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        let color = if self.state.pressed { 5 } else { 9 };

        // TODO: Handle properly
        draw.rect(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
            color,
        );

        draw.append_camera(-self.x, -self.y);
        self.content.as_widget().draw(draw);
        draw.append_camera(self.x, self.y);
    }
}