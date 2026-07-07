#[derive(Default)]
pub struct Options {
    project_path: String,
    league_path: String,
    cslol_path: String,
    ltk_path: String
}
impl Options{
    pub(crate) fn new() -> Options{
        Self{
            project_path: "".to_string(),
            league_path: "".to_string(),
            cslol_path: "".to_string(),
            ltk_path: "".to_string(),
        }
    }
}