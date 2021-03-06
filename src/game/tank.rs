use crate::game::bullet::Bullet;
use crate::game::common::{position::Position, view::PlayerView};
use crate::game::scene::unit::{MapCell, Unit, View};
use crate::game::scene::Scene;
use ggez::graphics::{self, DrawParam};
use ggez::Context;
use rand::Rng;
use std::cell::{Cell, RefCell};
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
    destroyed: Cell<bool>,
    view: View,
    map_cell: MapCell,
    pub scene: Rc<Scene>,
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

    fn position(&self) -> (u32, u32) {
        let p = self.position.borrow();
        (p.x(), p.y())
    }

    fn view(&self) -> Option<&View> {
        Some(&self.view)
    }

    fn view_enter(&self, _viewer: &dyn Unit) {
        //        println!("{} came into {}'s view", self, viewer);
    }

    fn view_leave(&self, _viewer: &dyn Unit) {
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
    }

    fn can_be_destroyed(&self, bullet: &Rc<Bullet>) -> bool {
        self.id != bullet.tank.id()
    }

    fn destroy(&self) {
        self.destroyed.set(true);
        self.scene.destroy_tank(self.id);
    }

    fn is_destroyed(&self) -> bool {
        self.destroyed.get()
    }
}

impl Tank {
    pub fn new(id: u32, cfg: Rc<TankCfg>, x: u32, y: u32, scene: Rc<Scene>) -> Tank {
        let position = RefCell::new(Position::new(x, y, 0.0));

        Tank {
            id,
            cfg,
            position,
            destroyed: Cell::new(false),
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

    pub fn fire(self: &Rc<Tank>, now: Instant) {
        let id = self.scene.next_unit_id();
        let (x, y) = self.position();
        let angle = self.position.borrow().angle();
        let bullet = Bullet::new(
            id,
            x,
            y,
            angle as f64,
            now,
            self.cfg.bullet.clone(),
            self.clone(),
        );
        self.scene.add_bullet(bullet);
    }

    pub fn update(self: &Rc<Self>, now: Instant) {
        if self.destroyed.get() {
            return;
        }

        if self.position.borrow_mut().update(now) {
            self.scene.map().unit_moved(self.clone())
        } else {
            if self.id != 1 {
                let (width, height) = self.scene.size();
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0, width);
                let y = rng.gen_range(0, height);
                self.move_to(x, y, now);
            }
        }
    }
}
