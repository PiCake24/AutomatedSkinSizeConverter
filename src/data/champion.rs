#[derive(Debug)]
pub struct Champion {
    name: String,
    skins:Vec<SkinScale>
}
impl Champion {
    pub(crate) fn new(name: &str) -> Champion{
        Self{
            name: name.to_string(),
            skins: Vec::new(),
        }
    }
    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn set_skins(&mut self, skins: Vec<SkinScale>){
        self.skins = skins;
    }
    pub fn get_skins(&self)-> &Vec<SkinScale>{
        &self.skins
    }
    pub fn add_skins(&mut self, skin: u16){
        self.skins.push(SkinScale::new(skin));
    }
    pub fn get_skins_mut(&mut self) -> &mut Vec<SkinScale> {
        &mut self.skins
    }
}
#[derive(Debug)]
pub struct SkinScale {
    skin: u16,
    scale: f32
}
impl SkinScale{
    pub(crate) fn new(skin: u16) -> SkinScale {
        Self {
            skin,
            scale: 2.0,
        }
    }

    pub fn get_skin(&self) -> u16 {
        self.skin
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}