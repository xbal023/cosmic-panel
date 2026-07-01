// Element for rendering a panel background

use calloop::LoopHandle;
use cosmic::iced::{Color, Length, id};
use cosmic::iced_core::Shadow;
use cosmic::widget::space;
use cosmic::{Theme, theme};

use crate::iced::{Element, IcedElement, Program};
use crate::xdg_shell_wrapper::shared_state::GlobalState;

pub type BackgroundElement = IcedElement<Background>;

pub fn background_element(
    id: id::Id,
    logical_width: i32,
    logical_height: i32,
    radius: [f32; 4],
    loop_handle: LoopHandle<'static, GlobalState>,
    theme: Theme,
    panel_id: usize,
    logical_pos: [f32; 2],
    color: [f32; 4],
    scale: f64,
    inner_glow: Option<cosmic_panel_config::InnerGlowConfig>,
) -> BackgroundElement {
    IcedElement::new(
        Background {
            id,
            logical_width,
            logical_height,
            radius,
            logical_pos: (logical_pos[0].round() as i32, logical_pos[1].round() as i32),
            color,
            scale,
            inner_glow,
        },
        (logical_width, logical_height),
        loop_handle,
        theme,
        panel_id,
        false,
    )
}

pub struct Background {
    pub id: id::Id,
    pub logical_width: i32,
    pub logical_height: i32,
    pub radius: [f32; 4],
    pub logical_pos: (i32, i32),
    pub color: [f32; 4],
    pub scale: f64,
    pub inner_glow: Option<cosmic_panel_config::InnerGlowConfig>,
}

impl Program for Background {
    type Message = ();

    fn view(&self) -> Element<'_, ()> {
        let width = self.logical_width as f32;
        let height = self.logical_height as f32;
        let radius_arr: [f32; 4] = self.radius;

        let color = self.color;
        let glow_config = self.inner_glow.clone();
        
        Element::from(
            cosmic::widget::container(space::horizontal().width(Length::Fixed(width)))
                .width(Length::Fixed(width))
                .height(Length::Fixed(height))
                .class(theme::Container::custom(move |theme| {
                    let cosmic = theme.cosmic();

                    let (border_width, border_color) = if let Some(ref glow) = glow_config {
                        if glow.animation_enabled {
                            // Fallback rendering since GPU shaders aren't supported in tiny-skia.
                            // We use the level to determine thickness and brightness to determine alpha.
                            let thickness = glow.level * 8.0;
                            let c = Color::from_rgba(glow.color[0], glow.color[1], glow.color[2], glow.brightness);
                            (thickness, c)
                        } else {
                            (0.0, cosmic.background.divider.into())
                        }
                    } else {
                        (0.0, cosmic.background.divider.into())
                    };

                    cosmic::widget::container::Style {
                        text_color: Some(cosmic.background.on.into()),
                        background: Some(Color::from(color).into()),
                        border: cosmic::iced::Border {
                            radius: radius_arr.into(),
                            width: border_width,
                            color: border_color,
                        },
                        shadow: Shadow::default(),
                        snap: true,
                        icon_color: Some(cosmic.background.on.into()),
                    }
                })),
        )
    }
}
