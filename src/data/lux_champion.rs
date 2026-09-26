use std::sync::mpsc::Sender;
use crate::converter::main_gui::{log, WorkerMessage};

pub struct LuxChampion {
    base: Option<f32>,
    fire: Option<f32>,
    water: Option<f32>,
    air: Option<f32>,
    nature: Option<f32>,
    magma: Option<f32>,
    dark: Option<f32>,
    mystic: Option<f32>,
    ice: Option<f32>,
    storm: Option<f32>,
    light: Option<f32>,
}

impl LuxChampion {
    /// All fields start unset.
    pub fn new() -> Self {
        LuxChampion {
            base: None,
            fire: None,
            water: None,
            air: None,
            nature: None,
            magma: None,
            dark: None,
            mystic: None,
            ice: None,
            storm: None,
            light: None,
        }
    }

    /// All fields start at 2.0 (original behavior).
    pub fn new_default() -> Self {
        LuxChampion {
            base: Some(2.0),
            fire: Some(2.0),
            water: Some(2.0),
            air: Some(2.0),
            nature: Some(2.0),
            magma: Some(2.0),
            dark: Some(2.0),
            mystic: Some(2.0),
            ice: Some(2.0),
            storm: Some(2.0),
            light: Some(2.0),
        }
    }

    // Getters (default to 0.0 if unset)
    pub fn get_base(&self) -> f32 {
        self.base.unwrap_or(0.0)
    }

    pub fn get_fire(&self) -> f32 {
        self.fire.unwrap_or(0.0)
    }

    pub fn get_water(&self) -> f32 {
        self.water.unwrap_or(0.0)
    }

    pub fn get_air(&self) -> f32 {
        self.air.unwrap_or(0.0)
    }

    pub fn get_nature(&self) -> f32 {
        self.nature.unwrap_or(0.0)
    }

    pub fn get_magma(&self) -> f32 {
        self.magma.unwrap_or(0.0)
    }

    pub fn get_dark(&self) -> f32 {
        self.dark.unwrap_or(0.0)
    }

    pub fn get_mystic(&self) -> f32 {
        self.mystic.unwrap_or(0.0)
    }

    pub fn get_ice(&self) -> f32 {
        self.ice.unwrap_or(0.0)
    }

    pub fn get_storm(&self) -> f32 {
        self.storm.unwrap_or(0.0)
    }

    pub fn get_light(&self) -> f32 {
        self.light.unwrap_or(0.0)
    }

    // Setters — log and skip if already set
    pub fn set_base(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.base.is_some() {
            log(sender, "Value for base is already set");
            return;
        }
        self.base = Some(value);
    }

    pub fn set_fire(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.fire.is_some() {
            log(sender, "Value for fire is already set");
            return;
        }
        self.fire = Some(value);
    }

    pub fn set_water(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.water.is_some() {
            log(sender, "Value for water is already set");
            return;
        }
        self.water = Some(value);
    }

    pub fn set_air(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.air.is_some() {
            log(sender, "Value for air is already set");
            return;
        }
        self.air = Some(value);
    }

    pub fn set_nature(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.nature.is_some() {
            log(sender, "Value for nature is already set");
            return;
        }
        self.nature = Some(value);
    }

    pub fn set_magma(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.magma.is_some() {
            log(sender, "Value for magma is already set");
            return;
        }
        self.magma = Some(value);
    }

    pub fn set_dark(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.dark.is_some() {
            log(sender, "Value for dark is already set");
            return;
        }
        self.dark = Some(value);
    }

    pub fn set_mystic(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.mystic.is_some() {
            log(sender, "Value for mystic is already set");
            return;
        }
        self.mystic = Some(value);
    }

    pub fn set_ice(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.ice.is_some() {
            log(sender, "Value for ice is already set");
            return;
        }
        self.ice = Some(value);
    }

    pub fn set_storm(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.storm.is_some() {
            log(sender, "Value for storm is already set");
            return;
        }
        self.storm = Some(value);
    }

    pub fn set_light(&mut self, sender: &Sender<WorkerMessage>, value: f32) {
        if self.light.is_some() {
            log(sender, "Value for light is already set");
            return;
        }
        self.light = Some(value);
    }

    /// Sets every field that hasn't been explicitly set yet to `value`.
    /// Fields already holding a value are left untouched — no log here,
    /// since "filling gaps" isn't a violation of the write-once rule.
    pub fn set_rest(&mut self, value: f32) {
        if self.base.is_none() {
            self.base = Some(value);
        }
        if self.fire.is_none() {
            self.fire = Some(value);
        }
        if self.water.is_none() {
            self.water = Some(value);
        }
        if self.air.is_none() {
            self.air = Some(value);
        }
        if self.nature.is_none() {
            self.nature = Some(value);
        }
        if self.magma.is_none() {
            self.magma = Some(value);
        }
        if self.dark.is_none() {
            self.dark = Some(value);
        }
        if self.mystic.is_none() {
            self.mystic = Some(value);
        }
        if self.ice.is_none() {
            self.ice = Some(value);
        }
        if self.storm.is_none() {
            self.storm = Some(value);
        }
        if self.light.is_none() {
            self.light = Some(value);
        }
    }
}