package core;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;

public class VersionChecker {
	private static final String GITHUB_API_URL = "https://api.github.com/repos/PiCake24/AutomatedSkinSizeConverter/releases/latest";

	/**
	 * Returns the latest Version from github as String. Reads only the tags
	 * 
	 * @return
	 * @throws IOException
	 */
	static String getLatestVersionFromGitHub() throws IOException {
		URL url = new URL(GITHUB_API_URL);
		HttpURLConnection connection = (HttpURLConnection) url.openConnection();
		connection.setRequestMethod("GET");
		connection.setRequestProperty("Accept", "application/json");

		if (connection.getResponseCode() == 200) {
			BufferedReader reader = new BufferedReader(new InputStreamReader(connection.getInputStream()));
			StringBuilder response = new StringBuilder();
			String line;
			while ((line = reader.readLine()) != null) {
				response.append(line);
			}
			reader.close();

			// Manually extract the "tag_name" field from the JSON response
			String responseBody = response.toString();
			String versionTag = "\"tag_name\":\"v"; // The start of the version tag
			int versionStartIndex = responseBody.indexOf(versionTag) + versionTag.length();
			int versionEndIndex = responseBody.indexOf("\"", versionStartIndex);

			if (versionStartIndex != -1 && versionEndIndex != -1) {
				return responseBody.substring(versionStartIndex, versionEndIndex).replace("v", "");
			} else {
				throw new IOException("Version not found in response");
			}
		} else {
			throw new IOException("Failed to fetch the latest version. HTTP Code: " + connection.getResponseCode());
		}
	}

	/**
	 * returns true, if new version > current version
	 * 
	 * @param currentVersion
	 * @param latestVersion
	 * @return
	 */
	public static boolean isNewVersionAvailable(String currentVersion, String latestVersion) {
		return currentVersion.compareTo(latestVersion) < 0;
	}

	private VersionChecker() {
	}
}
