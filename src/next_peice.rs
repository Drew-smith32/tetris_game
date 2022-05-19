use bevy::prelude::*;
use rand::Rng;

use crate::kind::{Kind, KINDS};


pub struct NextPeieces {
    pub peices: Vec<Kind>,
    pub entity: Option<Entity>,
}

impl NextPeieces {
    pub fn new() -> Self {
        let mut next = NextPeieces {
            peices: Vec::<Kind>::with_capacity(4),
            entity: None,
        };

        let mut rng = rand::thread_rng();
        for i in 0..4 {
            let p = KINDS[rng.gen_range(0..KINDS.len())];
            next.peices.insert(0, p);
        }

        next
    }

    pub fn pop(&mut self) -> Kind {
        let mut rng = rand::thread_rng();
        let result = self.peices.pop();
        info!("pop {:?}", result);
        self.peices.push(KINDS[rng.gen_range(0..KINDS.len())]);
        if let Some(r) = result {
            r
        } else {
            // shouldnt ever reach
            Kind::J
        }
    }
}