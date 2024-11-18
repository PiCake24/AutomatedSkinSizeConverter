package core;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class LegendarySkins {
	public static void checkForSonaLegendary(String champion, String rootPath)
			throws InterruptedException, IOException {
		Gui.updateLog("Checking if champion has a legendary skin");
		if (champion.equals("sona")) {
			// check for djsona1, 2, 3
			convertSona(rootPath);
		} else {
			Gui.updateLog("No legendary found");
		}
	}

	private static void convertSona(String rootPath) throws InterruptedException, IOException {
		Map<Integer, Double> map = new HashMap<>();
		File file = new File(rootPath + "\\0PutOptionFilesHere\\sonalegendary.txt");
		if (file.exists()) {
			Gui.updateLog("Size options for Sona legendary found");
			try (BufferedReader read = new BufferedReader(new FileReader(file))) {
				String elemSize = read.readLine();
				double defaultsize = Double.parseDouble(elemSize.split(":")[1].trim());
				while ((elemSize = read.readLine()) != null) {
					Integer genre = Integer.parseInt(elemSize.split(":")[0].trim());
					double size = Double.parseDouble(elemSize.split(":")[1].trim());
					map.put(genre, size);
				}
				if (!map.containsKey(1)) {
					map.put(1, defaultsize);
				}
				if (!map.containsKey(2)) {
					map.put(2, defaultsize);
				}
				if (!map.containsKey(3)) {
					map.put(3, defaultsize);
				}
			}
		} else {
			Gui.updateLog("No size options for Sona legendary found. setting to default (4)");
			map.put(1, 4.);
			map.put(2, 4.);
			map.put(3, 4.);
		}
		for (Map.Entry<Integer, Double> entry : map.entrySet()) {
			int djsonaNr = entry.getKey();
			ExecuteRitobin.startProgBinToPy("sonadjgenre0" + djsonaNr, 6, rootPath);

			Thread.sleep(2000);

			// write into created file
			WriteIntoPy.writeInto("sonadjgenre0" + djsonaNr, 6, entry.getValue(), rootPath);

			Thread.sleep(2000);

			startProgPytoBinSona(djsonaNr, rootPath);
		}
	}

	/**
	 * Calls ritobin and converts a py file into a bin file. This does not use the
	 * almost identical version in Execute Programs, because the output directory is
	 * different
	 * 
	 * @param element
	 * @param rootPath
	 * @param cliPath
	 * @throws IOException
	 */
	public static void startProgPytoBinSona(int element, String rootPath) throws IOException {
		String cliPath = UnpackExe.getUnpackedRitobin().toString();

		List<String> l = new ArrayList<>();
		l.add(cliPath);
		l.add(rootPath + "\\0WADS\\data\\characters\\sonadjgenre0" + element + "\\skins\\skin6.py");
		l.add(rootPath + "\\sona.wad.client\\data\\characters\\sonadjgenre0" + element + "\\skins\\skin6.bin");
		new ProcessBuilder(l).start();
		Gui.updateLog("sonadjgenre0" + element + ".bin created");

	}

	private LegendarySkins() {
	}
}
