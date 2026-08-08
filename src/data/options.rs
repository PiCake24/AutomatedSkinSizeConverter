#[derive(Default, Clone, Debug)]
pub struct Options {
    project_path: String,
    league_path: String,
    cslol_path: String,
    ltk_path: String
}
impl Options{ //todo defaultvalues for me for testing
    pub(crate) fn new_default() -> Options{
        Self{
            project_path: r"D:\wad".to_string(),
            league_path: r"D:\Riot Games\League of Legends\Game".to_string(),
            cslol_path: r"D:\Programs verknuepfng\Programs\cslol-manager".to_string(),
            ltk_path: r"C:\mods".to_string(),
        }
    }
    pub(crate) fn new(project_path: &str, league_path: &str,cslol_path: &str, ltk_path: &str) -> Options{
        Self{
           project_path: project_path.parse().unwrap(), //todo
            league_path: league_path.parse().unwrap(), //todo
            cslol_path: cslol_path.parse().unwrap(), //todo
            ltk_path: ltk_path.to_string(),
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