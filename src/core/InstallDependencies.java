package core;

import java.io.BufferedInputStream;
import java.io.BufferedOutputStream;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

import javax.swing.SwingWorker;

public class InstallDependencies extends SwingWorker<String, String> {
	@Override
	protected String doInBackground() throws Exception {
		installDependencies();
		Gui.enableButtons();
		return null;
	}

	private static void installDependencies() {
		if (!cdtbIsInstalled()) {
			if (!pipIsInstalled()) {
				Gui.updateLog("Pip is not installed, installing pip");
				if (installPip()) {
					Gui.updateLog("pip has been installed");
				} else {
					Gui.updateLog("Could not install pip");
				}
			}
			if (installCdtb()) {
				Gui.updateLog("cdtb has been installed");
			} else {
				Gui.updateLog("Could not install cdtb");
			}
		}
	}

	private static boolean cdtbIsInstalled() {
		ProcessBuilder builder = new ProcessBuilder("python", "-c", "import " + "cdtb");
		try {
			Process process = builder.start();
			process.waitFor();

			// Check the exit code
			int exitCode = process.exitValue();
			if (exitCode == 0) {
				Gui.updateLog("cdtb is already installed");
				return true;
			} else {
				Gui.updateLog("cdtb is not installed, installing cdtb...");
				return false;
			}
		} catch (IOException | InterruptedException e) {
			e.printStackTrace();
		}
		return false;
	}

	private static boolean pipIsInstalled() {
		String pipPath = GetPythonPath.getPythonPathWindows() + "/pip.exe"; // e.g., "C:/Python39/Scripts/pip.exe"
		File file = new File(pipPath);
		return file.exists();
	}

	private static final String GET_PIP_URL = "https://bootstrap.pypa.io/get-pip.py";
	private static final String GET_PIP_FILE = "get-pip.py";

	private static boolean installPip() {
		try {

			// Step 1: Download get-pip.py
			if (!downloadFile(GET_PIP_URL, GET_PIP_FILE)) {
				Gui.updateLog("File could not be downloaded");
				return false;
			}
			Gui.updateLog("File successfully downloaded");

			// Step 2: Run the get-pip.py script using Python
			ProcessBuilder processBuilder = new ProcessBuilder("python", GET_PIP_FILE);
			Process process = processBuilder.start();

			// Wait for the process to complete
			int exitCode = process.waitFor();
			Gui.updateLog("ExitCode:" + exitCode);
			return exitCode == 0;
		} catch (IOException | InterruptedException e) {
			e.printStackTrace();
			return false;
		} finally {
			// Clean up the downloaded file
			// new File(GET_PIP_FILE).delete();
		}
	}

	private static boolean downloadFile(String fileURL, String saveFilePath) {
		try (InputStream in = new BufferedInputStream(new java.net.URL(fileURL).openStream());
				OutputStream out = new BufferedOutputStream(new FileOutputStream(saveFilePath))) {

			byte[] buffer = new byte[1024];
			int bytesRead;
			while ((bytesRead = in.read(buffer)) != -1) {
				out.write(buffer, 0, bytesRead);
			}
			return true;
		} catch (IOException e) {
			e.printStackTrace();
			return false;
		}
	}

	private static boolean installCdtb() {
		Gui.updateLog("Installing cdtb");
		ProcessBuilder builder = new ProcessBuilder("python", "-m", "pip", "install", "cdtb");
		try {
			Process process = builder.start();
			int exitCode = process.waitFor();

			// Check the exit code
			return exitCode == 0; // Exit code 0 means installation was successful
		} catch (IOException | InterruptedException e) {
			e.printStackTrace();
		}
		return false;
	}

}
