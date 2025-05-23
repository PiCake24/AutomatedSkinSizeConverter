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
	private static final String DJSONA = "sonadjgenre0";

	/**
	 * Checks if the current champion is Sona
	 * 
	 * @param champion
	 * @param rootPath
	 * @throws InterruptedException
	 * @throws IOException
	 */

	public static void checkForLegendary(Map<String, Integer> map, String champion, String rootPath)
			throws InterruptedException, IOException {
		Gui.updateLog("Checking if champion has a legendary skin");
		int maxSkinNumber = map.get(champion);
		if (champion.equals("sona") && maxSkinNumber >= 6) {
			createFolders(rootPath);
			convertSona(rootPath);
		} else {
			Gui.updateLog("No legendary found");
		}
	}

	/**
	 * 
	 */
	static void createFolders(String rootPath) {
		File file = new File(rootPath + "\\sona.wad.client\\data\\characters\\sonadjgenre01\\skins");
		file.mkdirs();
		file = new File(rootPath + "\\sona.wad.client\\data\\characters\\sonadjgenre02\\skins");
		file.mkdir();
		file = new File(rootPath + "\\sona.wad.client\\data\\characters\\sonadjgenre03\\skins");
		file.mkdir();
	}

	/**
	 * Converts DJSona
	 * 
	 * @param rootPath
	 * @throws InterruptedException
	 * @throws IOException
	 */
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
			int djSonaNr = entry.getKey();
			ExecuteRitobin.startProgBinToPy(DJSONA + djSonaNr, 6, rootPath);

			Thread.sleep(2000);

			// write into created file
			WriteIntoPy.writeInto(DJSONA + djSonaNr, 6, entry.getValue(), rootPath);

			Thread.sleep(2000);

			startProgPytoBinSona(djSonaNr, rootPath);
		}
	}

	/**
	 * Calls Ritobin and converts a py file into a bin file. This does not use the
	 * almost identical version in Execute Programs, because the output directory is
	 * different
	 * 
	 * @param djSonaNr
	 * @param rootPath
	 * @param cliPath
	 * @throws IOException
	 */
	public static void startProgPytoBinSona(int djSonaNr, String rootPath) throws IOException {
		String cliPath = UnpackExe.getUnpackedRitobin().toString();

		List<String> l = new ArrayList<>();
		l.add(cliPath);
		l.add(rootPath + "\\0WADS\\data\\characters\\sonadjgenre0" + djSonaNr + "\\skins\\skin6.py");
		l.add(rootPath + "\\sona.wad.client\\data\\characters\\sonadjgenre0" + djSonaNr + "\\skins\\skin6.bin");
		new ProcessBuilder(l).start();
		Gui.updateLog(DJSONA + djSonaNr + ".bin created");

	}

	private LegendarySkins() {
	}
}
