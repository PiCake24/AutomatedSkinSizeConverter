package core;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class ExecuteProgramm {

	public static void startProgBinToPy(String champion, int skinNumber, String rootPath, String cliPath) {
		try {
			List<String> l = new ArrayList<>();
			l.add(cliPath);
			l.add(rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".bin");
			l.add(rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".py");
			new ProcessBuilder(l).start();
			Gui.updateLog(champion + " " + skinNumber + " .py created");
		} catch (IOException e) {
			e.printStackTrace();
		}
	}

	public static void startProgPytoBin(String champion, int skinNumber, String rootPath, String cliPath) {
		try {
			List<String> l = new ArrayList<>();
			l.add(cliPath);
			l.add(rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".py");
			l.add(rootPath + "\\" + champion + ".wad.client\\data\\characters\\" + champion + "\\skins\\skin"
					+ skinNumber + ".bin");
			new ProcessBuilder(l).start();
			Gui.updateLog(champion + " " + skinNumber + " .bin created");
		} catch (IOException e) {
			Gui.updateLog(e.getLocalizedMessage());
		}
	}

	private ExecuteProgramm() {
	}
}
