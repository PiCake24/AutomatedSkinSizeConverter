#[derive(Debug)]
pub struct Champion {
    name: String,
    skins:Vec<u16>,
    size:Vec<f32>
}
impl Champion {
    pub(crate) fn new(name: &str) -> Champion{
        Self{
            name: name.to_string(),
            skins: Vec::new(),
            size:Vec::new()
        }
    }
    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn set_skins(&mut self, skins: Vec<u16>){
        self.skins = skins;
    }
    pub fn get_skins(&self)-> &Vec<u16>{
        &self.skins
    }
    pub fn set_size(&self){
        todo!()
    }
}