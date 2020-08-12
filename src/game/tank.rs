use crate::game::common::{path::Path, position::Position, view::PlayerView};
use crate::game::scene::{
    unit::{MapCell, Unit, View},
    Scene,
};
use config::Config;
use ggez::graphics::{self, DrawParam, Image};
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};
use std::cell::{Cell, Ref, RefCell};
use std::convert::{TryFrom, TryInto};
use std::f64;
use std::fmt;
use std::rc::Rc;
use std::time::Instant;

mod cfg;
pub use cfg::*;

pub struct Tank {
    id: u32,
    cfg: Rc<TankCfg>,
    position: RefCell<Position>,
    destroyed: bool,
    view: View,
    map_cell: MapCell,
    scene: Rc<Scene>,
}

impl fmt::Display for Tank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tank({})", self.id)
    }
}

impl fmt::Debug for Tank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tank({})", self.id)
    }
}

impl Unit for Tank {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        "crazy tank"
    }

    fn position(&self) -> Ref<'_, Position> {
        self.position.borrow()
    }

    fn view(&self) -> Option<&View> {
        Some(&self.view)
    }

    fn view_enter(&self, viewer: &dyn Unit) {
        //        println!("{} came into {}'s view", self, viewer);
    }

    fn view_leave(&self, viewer: &dyn Unit) {
        //        println!("{} disapear from {}'s view", self, viewer);
    }

    fn map_cell(&self) -> &MapCell {
        &self.map_cell
    }

    fn draw(&self, ctx: &mut Context, view: &PlayerView) {
        let position = self.position.borrow();

        let dx = position.x() as f64 - view.x as f64;
        let dy = position.y() as f64 - view.y as f64;
        let angle = position.angle() + std::f32::consts::FRAC_PI_2;

        // tank
        graphics::draw(
            ctx,
            &self.cfg.image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5])
                .rotation(angle),
        )
        .unwrap();

        // barrel
        graphics::draw(
            ctx,
            &self.cfg.barrel_image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.9])
                .rotation(angle),
        )
        .unwrap();

        // let dx = self.x as f64 - x1 as f64;
        // let dy = self.y as f64 - y1 as f64;

        // self.batch.add(
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.5]),
        // );

        // // tank
        // graphics::draw(
        //     ctx,
        //     &self.cfg.image,
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.5])
        //         .rotation(self.angle),
        // )?;

        // // barrel
        // graphics::draw(
        //     ctx,
        //     &self.cfg.image_barrel,
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.1])
        //         .rotation(self.angle),
        // )?;

        // bullet
        // if let Some(ref mut b) = self.bullet {
        //     b.draw(ctx, x1, y1, &self.cfg.bullet)?;
        // }

        //        crate::util::debug::draw_circle(ctx, dx as f32, dy as f32, 1.0)?;

        // // flag
        // if let Some(p) = &self.path {
        //     let fx = p.x2 as f64 - x1 as f64;
        //     let fy = p.y2 as f64 - y1 as f64;

        //     graphics::draw(
        //         ctx,
        //         flag,
        //         DrawParam::new()
        //             .dest([fx as f32, fy as f32])
        //             .offset([0.5, 0.5]),
        //     )?;
        // }
    }
}

impl Tank {
    pub fn new(id: u32, cfg: Rc<TankCfg>, x: u32, y: u32, scene: Rc<Scene>) -> Tank {
        let position = RefCell::new(Position::new(x, y, 0.0));

        Tank {
            id,
            cfg,
            position,
            destroyed: false,
            view: View::new(100),
            map_cell: Default::default(),
            scene,
        }
    }

    pub fn move_to(&self, x: u32, y: u32, now: Instant) {
        self.position
            .borrow_mut()
            .move_to(x, y, self.cfg.speed, now);
    }

    pub fn fire(&mut self, now: Instant) {
        //        self.bullet = Some(Bullet::new(self.x, self.y, self.barrel_angle as f64, now));
    }

    pub fn update(self: &Rc<Self>, now: Instant) {
        if self.position.borrow_mut().update(now) {
            self.scene.map().unit_moved(self.clone())
        } else {
            if self.id != 1 {
                // player-controlled tank
                let (width, height) = self.scene.size();

                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0, width);
                let y = rng.gen_range(0, height);

                self.move_to(x, y, now)
            }
        }
    }
}
