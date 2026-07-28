#[derive(Default, Clone)]
pub struct Options {
    project_path: String,
    league_path: String,
    cslol_path: String,
    ltk_path: String
}
impl Options{ //todo defaultvalues for me for testing
    pub(crate) fn new() -> Options{
        Self{
            project_path: r"D:\wad".to_string(),
            league_path: r"D:\Riot Games\League of Legends\Game".to_string(),
            cslol_path: r"D:\Programs verknuepfng\Programs\cslol-manager".to_string(),
            ltk_path: r"C:\mods".to_string(),
        }
    }
    pub fn get_project_path(&self) -> &str{ &self.project_path }
    pub fn get_league_path(&self) -> &str{ &self.league_path }
    pub fn get_cslol_path(&self) -> &str{ &self.cslol_path }
    pub fn get_ltk_path(&self) -> &str{ &self.ltk_path }

}
pub fn get_ritobin_path(options: &Options) -> String{
    format!(r"{}\0WADS\ritobin_cli.exe", options.get_project_path())
}