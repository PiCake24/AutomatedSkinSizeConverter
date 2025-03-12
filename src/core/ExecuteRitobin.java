package core;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;

import oshi.SystemInfo;
import oshi.hardware.GlobalMemory;

public class ExecuteRitobin {

	private static final String CHARACTERPATH = "\\0WADS\\data\\characters\\";
	private static final String SKINPATH = "\\skins\\skin";

	/**
	 * Calls ritobin and converts a bin file into a py file
	 * 
	 * @param champion
	 * @param skinNumber
	 * @param rootPath
	 * @param cliPath
	 * @throws IOException
	 * @throws InterruptedException
	 */
	public static void startProgBinToPy(String champion, int skinNumber, String rootPath)
			throws IOException, InterruptedException {
		String cliPath = UnpackExe.getUnpackedRitobin().toString();
		String processName = UnpackExe.getUnpackedRitobin().getFileName().toString();
		while (!isSystemResourcesAvailable() && isProgramRunning(processName)) {
			System.out.println("Is already running");
			Thread.sleep(50);
		}

		List<String> l = new ArrayList<>();
		l.add(cliPath);
		l.add(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".bin");
		l.add(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".py");
		new ProcessBuilder(l).start();
		Gui.updateLog(champion + " " + skinNumber + " .py created");
	}

	/**
	 * TODO
	 * 
	 * @param processName
	 * @return
	 */
	private static boolean isProgramRunning(String processName) {
		try {
			ProcessBuilder builder = new ProcessBuilder("tasklist");
			Process process = builder.start();
			try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()))) {
				return reader.lines().anyMatch(line -> line.toLowerCase().contains(processName.toLowerCase()));
			}
		} catch (IOException e) {
			e.printStackTrace();
		}

		return false;
	}

	/**
	 * TODO
	 * 
	 * @return
	 */
	private static boolean isSystemResourcesAvailable() {
		SystemInfo si = new SystemInfo();

		GlobalMemory memory = si.getHardware().getMemory();
		double memoryLoad = (memory.getTotal() - memory.getAvailable()) * 100.0 / memory.getTotal();
		return (memoryLoad < 90 && ResourceMonitor.isSystemResourcesAvailable());
	}

	/**
	 * Calls ritobin and converts a py file into a bin file
	 * 
	 * @param champion
	 * @param skinNumber
	 * @param rootPath
	 * @param cliPath
	 * @throws IOException
	 */
	public static void startProgPytoBin(String champion, int skinNumber, String rootPath) throws IOException {
		String cliPath = UnpackExe.getUnpackedRitobin().toString();
		List<String> l = new ArrayList<>();
		l.add(cliPath);
		l.add(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".py");
		l.add(rootPath + "\\" + champion + ".wad.client\\data\\characters\\" + champion + SKINPATH + skinNumber
				+ ".bin");
		new ProcessBuilder(l).start();
		Gui.updateLog(champion + " " + skinNumber + " .bin created");
	}

	private ExecuteRitobin() {
	}
}
