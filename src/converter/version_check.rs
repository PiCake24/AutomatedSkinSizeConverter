use std::error::Error;
use std::io::Read;
use std::sync::mpsc::Sender;
use crate::converter::main_gui::{log, WorkerMessage};
use crate::VERSION;

pub fn version_check(
    sender: &Sender<WorkerMessage>,
) -> Result<(String), Box<dyn Error>> {
    let url = "https://api.github.com/repos/PiCake24/AutomatedSkinSizeConverter/releases/latest";

    let client = reqwest::blocking::Client::builder()
        .user_agent("AutomatedSkinSizeConverter")
        .build()?;

    let res = client.get(url).send()
        .inspect_err(|e| {
            log(sender, format!("Could not contact GitHub: {}", e));
        })?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().unwrap_or_default();

        log(
            sender,
            format!("GitHub API returned {}: {}", status, body),
        );

        return Err(format!("GitHub API returned {}", status).into());
    }

    // let body = res.text()?;
    // println!("Status: {}", res.status());
    // println!("Body: {}", res.text()?);
    let version_tag = "\"tag_name\":\"v";
    let binding = res.text()?; //todo rename
    let a = binding.split_once(version_tag).unwrap().1.split_once("\",\"target_commitish\"").unwrap().0;
    // String versionTag = "\"tag_name\":\"v"; // The start of the version tag
    // int versionStartIndex = responseBody.indexOf(versionTag) + versionTag.length();
    // int versionEndIndex = responseBody.indexOf("\"", versionStartIndex);
    println!("{}", &a);

    Ok(a.parse()?)
}

/**
 * Returns the latest Version from github as String. Reads only the tags
 *
 * @return
 * @throws IOException
 */
// static String getLatestVersionFromGitHub() throws IOException {
// URL url = new URL(GITHUB_API_URL);
// HttpURLConnection connection = (HttpURLConnection) url.openConnection();
// connection.setRequestMethod("GET");
// connection.setRequestProperty("Accept", "application/json");
//
// if (connection.getResponseCode() == 200) {
// BufferedReader reader = new BufferedReader(new InputStreamReader(connection.getInputStream()));
// StringBuilder response = new StringBuilder();
// String line;
// while ((line = reader.readLine()) != null) {
// response.append(line);
// }
// reader.close();
//
// // Manually extract the "tag_name" field from the JSON response
// String responseBody = response.toString();
// String versionTag = "\"tag_name\":\"v"; // The start of the version tag
// int versionStartIndex = responseBody.indexOf(versionTag) + versionTag.length();
// int versionEndIndex = responseBody.indexOf("\"", versionStartIndex);
//
// if (versionStartIndex != -1 && versionEndIndex != -1) {
// return responseBody.substring(versionStartIndex, versionEndIndex).replace("v", "");
// } else {
// throw new IOException("Version not found in response");
// }
// } else {
// throw new IOException("Failed to fetch the latest version. HTTP Code: " + connection.getResponseCode());
// }
// }

/**
 * returns true, if new version > current version
 *
 * @param currentVersion
 * @param latestVersion
 * @return
 */
fn no(){}
// public static boolean isNewVersionAvailable(String currentVersion, String latestVersion) {
// return currentVersion.compareTo(latestVersion) < 0;
// }
