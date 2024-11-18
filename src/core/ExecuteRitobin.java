package core;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

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
	 */
	public static void startProgBinToPy(String champion, int skinNumber, String rootPath) throws IOException {
		String cliPath = UnpackExe.getUnpackedRitobin().toString();
		List<String> l = new ArrayList<>();
		l.add(cliPath);
		l.add(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".bin");
		l.add(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".py");
		new ProcessBuilder(l).start();
		Gui.updateLog(champion + " " + skinNumber + " .py created");
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
