package core;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class GetPythonPath {

	static String getPythonPathWindows() {
		ProcessBuilder builder = new ProcessBuilder("cmd.exe", "/c", "where", "python");
		try {
			Process process = builder.start();
			BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
			String line = reader.readLine();
			if (line != null) {
				// Remove python.exe from the end of the path
				line = line.trim();
				if (line.toLowerCase().endsWith("\\python.exe")) {
					line = line.substring(0, line.length() - "\\python.exe".length());
				}

				// Append the Scripts folder path
				String scriptsPath = line + "\\Scripts";
				if (!scriptsPath.endsWith("\\")) {
					scriptsPath += "\\";
				}
				Gui.updateLog("PythonPath: " + scriptsPath);
				return scriptsPath;
			}
		} catch (IOException e) {
			e.printStackTrace();
		}
		return null;
	}

	private GetPythonPath() {
	}
}