pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            h if h == 0 && self.level >= 10 => Some(Player {
                health: 100,
                mana: Some(100),
                ..*self
            }),
            h if h == 0 => Some(Player {
                health: 100,
                ..*self
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) if mana_cost <= m => {
                self.mana = Some(m - mana_cost);
                mana_cost * 2
            }
            Some(_) => 0,
            None => {
                self.health = if self.health >= mana_cost {
                    self.health - mana_cost
                } else {
                    0
                };
                0
            }
        }
    }
}
