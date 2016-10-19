use entity::Entity;
use piston_window::*;

pub struct RenderSystem {}

impl RenderSystem {
    pub fn render_entity(entity: &Entity, window: &mut PistonWindow, e: &Event) {
        match entity.clone().graphics_component.sprite_filename {
            Some(sprite_path) => {
                let sprite = Texture::from_path(
                        &mut window.factory.clone(),
                        &sprite_path,
                        Flip::None,
                        &TextureSettings::new())
                        .unwrap();
                window.draw_2d(e, |c, g| {
                    image(&sprite, c.transform.trans(entity.x_pos(), entity.y_pos()), g);
                });
            },
            None => {
                window.draw_2d(e, |c, g| {
                    rectangle(entity.color(), entity.geometry(), c.transform, g);
                });
            }
        }
    }
}
