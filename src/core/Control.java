package core;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class Control {

	private static final String OPTIONS = "Options.txt";
	private static final String CHARACTERPATH = "\\0WADS\\data\\characters\\";
	private static final String SKINPATH = "\\skins\\skin";
	private static final String QUEUINGUP = "Queuing up: ";

	/**
	 * The main control method
	 * 
	 * @throws IOException
	 * @throws InterruptedException
	 */
	static void control() throws IOException, InterruptedException {
		Map<String, Integer> map = new HashMap<>();
		String[] paths = getPaths();
		String rootPath = paths[0];
		String leaguePath = paths[1];

		addChampionsToMap(map);

		createFolders(map, rootPath);

		Gui.updateLog("Unpacking Ritobin");
		if (!UnpackExe.unpackRitobin()) {
			throw new IOException();
		}

		if (Gui.getSelfUnpackCheckBoxBool()) {
			Gui.updateLog("Unpacking CDTB");
			if (!UnpackExe.unpackCDTBTranslator()) {
				throw new IOException();
			}
			Gui.updateLog("Updating hashes");
			if (!CDTBExecution.downloadHashes()) {
				throw new IOException();
			}
			if (!CDTBExecution.extractAllFiles(map, leaguePath, rootPath)) {
				throw new IOException();
			}

			UnpackExe.removeCDTBTranslator();
		}

		getActualNumberOfSkins(map, rootPath);

		translateAndRewriteFiles(map, rootPath);

		UnpackExe.removeRitobin();

		Gui.updateLog("Done");
	}

	/**
	 * Reads the paths from the options file
	 * 
	 * @return
	 * @throws IOException
	 */
	static String[] getPaths() throws IOException {
		String[] paths = new String[2];
		try (BufferedReader read = new BufferedReader(new FileReader(new File(OPTIONS)))) {
			String s1 = read.readLine();
			String s2 = read.readLine();
			String[] split1 = s1.split(":");
			String[] split2 = s2.split(":");
			s1 = split1[1] + ":" + split1[2];
			s2 = split2[1] + ":" + split2[2];
			paths = new String[3];
			paths[0] = s1.trim();
			paths[1] = s2.trim();
			paths[1] += "\\DATA\\FINAL\\Champions";
		}
		Gui.updateLog("Root Path:" + paths[0]);
		Gui.updateLog("League Path: " + paths[1]);
		Logger.getInstance().log(paths[0]);
		Logger.getInstance().log(paths[1]);
		return paths;
	}

	/**
	 * Adds champions to the map
	 * 
	 * @param map
	 * @param rootPath
	 */
	private static void addChampionsToMap(Map<String, Integer> map) throws IOException {
		try (BufferedReader read = new BufferedReader(new FileReader(new File(OPTIONS)))) {
			Gui.updateLog("Reading options file");
			String line;
			for (int i = 0; i < 2; i++) {
				line = read.readLine();
				if (line == null) {
					throw new IOException();
				}
			}
			while ((line = read.readLine()) != null) {
				if (line.charAt(0) != '#' && !line.trim().equals("")) {
					String[] split = line.split(" ");
					if (split.length > 1) {
						map.put(split[0].toLowerCase(), Integer.parseInt(split[1]));
						Gui.updateLog(QUEUINGUP + split[0] + " with " + (Integer.parseInt(split[1]) + 1) + " skins");
					} else {
						int number = -1;
						map.put(split[0].toLowerCase(), number);
						Gui.updateLog(QUEUINGUP + split[0] + " with all skins");
					}
				}

			}
		}
	}

	/**
	 * returns the number of skins for all champions that have their skin number not
	 * defined in the Options.txt file
	 * 
	 * @param map
	 * @param rootPath
	 */
	private static void getActualNumberOfSkins(Map<String, Integer> map, String rootPath) {
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			if (map.get(champion) < 0) {
				int number = getNumberOfSkins(champion, rootPath);
				map.put(champion, number);
				Gui.updateLog(QUEUINGUP + champion + " with " + (number + 1) + " skins");
			}

		}
	}

	/**
	 * Returns the number of skins, by checking whether a file for it exists. If
	 * there hasnt beeen found a file for 50 skins it will retrn the correct number
	 * 
	 * @param champion
	 * @param rootPath
	 * @return
	 */
	private static int getNumberOfSkins(String champion, String rootPath) {
		// check if skin exists
		Gui.updateLog("getting number of skins");
		int numberOfCensecutiveTries = 0;
		int numberOfSkins = 0;
		while (numberOfCensecutiveTries < 50) {
			File f = new File(rootPath + CHARACTERPATH + champion + SKINPATH + numberOfSkins + ".bin");
			if (!f.exists()) {
				numberOfCensecutiveTries++;
				numberOfSkins++;
			} else {
				numberOfSkins++;
				numberOfCensecutiveTries = 0;
			}
		}
		// subtract the 51 skins again that dont exist
		numberOfSkins -= 51;

		return numberOfSkins;
	}

	/**
	 * Creates all needed folders
	 * 
	 * @param map
	 * @param rootPath
	 */
	private static void createFolders(Map<String, Integer> map, String rootPath) {
		File file = new File(rootPath + "\\0WADS");
		file.mkdir();
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			file = new File(rootPath + "\\" + champion + ".wad.client\\data\\characters\\" + champion + "\\skins");
			file.mkdirs();
		}
	}

	/**
	 * creates all files for all champions in the map
	 * 
	 * @param map
	 * @param rootPath
	 * @throws InterruptedException
	 * @throws IOException
	 */
	private static void translateAndRewriteFiles(Map<String, Integer> map, String rootPath)
			throws InterruptedException, IOException {
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];

			convertToPython(map, rootPath, champion);
		}
		Thread.sleep(5000);
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			rewriteFile(map, rootPath, champion);
		}
		Thread.sleep(5000);
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			convertToWad(map, rootPath, champion);

			LegendarySkins.checkForLegendary(map, champion, rootPath);
		}
	}

	/**
	 * Converts all files to .py
	 * 
	 * @param map
	 * @param rootPath
	 * @param champion
	 * @throws InterruptedException
	 * @throws IOException
	 */
	private static void convertToPython(Map<String, Integer> map, String rootPath, String champion)
			throws InterruptedException, IOException {
		for (int skinNumber = 0; skinNumber <= map.get(champion); skinNumber++) {
			if (new File(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".bin").exists()) {
				ExecuteRitobin.startProgBinToPy(champion, skinNumber, rootPath);
				Thread.sleep(50);
			} else {
				Gui.updateLog(champion + " " + skinNumber + " does not exist");
			}
		}
	}

	/**
	 * writes into the files and changes their size accordingly
	 * 
	 * @param map
	 * @param rootPath
	 * @param champion
	 * @throws IOException
	 */
	private static void rewriteFile(Map<String, Integer> map, String rootPath, String champion) throws IOException {
		ArrayList<Double> sizes = getSize(champion, map.get(champion), rootPath);
		for (int skinNumber = 0; skinNumber <= map.get(champion); skinNumber++) {
			if (new File(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".bin").exists()) {
				if (champion.equals("lux") && skinNumber == 7) {
					WriteIntoPy.prepWriteLuxLegendaryIntoPy(rootPath);
				} else {
					WriteIntoPy.writeInto(champion, skinNumber, sizes.get(skinNumber), rootPath);
				}
			}
		}
	}

	/**
	 * returns the size of the champion and skin number as arraylist
	 * 
	 * @param champion
	 * @param skinNumbers
	 * @param rootPath
	 * @return
	 * @throws IOException
	 */
	private static ArrayList<Double> getSize(String champion, int skinNumbers, String rootPath) throws IOException {
		ArrayList<Double> sizes = new ArrayList<>();
		File file = new File(rootPath + "\\0PutSizeOptionFilesHere\\" + champion + ".txt");
		if (file.exists()) {
			Gui.updateLog("Size Options found");
			try (BufferedReader read = new BufferedReader(new FileReader(file))) {
				// Add default size
				addSizesToArray(read, skinNumbers, sizes);
			}
		} else {
			Gui.updateLog("No size options found, setting size to default(5)");
			for (int i = 0; i <= skinNumbers; i++) {
				sizes.add(5.);
			}
		}
		return sizes;
	}

	/**
	 * adds the sizes to the sizes Array
	 * 
	 * @param read
	 * @param skinNumbers
	 * @param sizes
	 * @throws IOException
	 */
	private static void addSizesToArray(BufferedReader read, int skinNumbers, ArrayList<Double> sizes)
			throws IOException {
		String size = read.readLine();
		String[] split = size.split(":");
		double defaulSize = Double.parseDouble(split[1].trim());
		int prevMinSkin = -1;
		while ((size = read.readLine()) != null) {
			if (!size.trim().equals("")) {
				split = size.split(":");
				int minSkin = Integer.parseInt(split[0]);

				// fill up until current skinnumber
				for (int i = prevMinSkin + 1; i < minSkin; i++) {
					if (i <= skinNumbers) {
						sizes.add(defaulSize);
					}
				}
				if (minSkin <= skinNumbers) {
					sizes.add(Double.parseDouble(split[1].trim()));
				}
				prevMinSkin = minSkin;
			}
		}
		// from prevminSkin to skinNumbers fill with default size
		for (int i = prevMinSkin + 1; i <= skinNumbers; i++) {
			sizes.add(defaulSize);
		}
	}

	/**
	 * Converts the file back to .bin
	 * 
	 * @param map
	 * @param rootPath
	 * @param champion
	 * @throws InterruptedException
	 * @throws IOException
	 */
	private static void convertToWad(Map<String, Integer> map, String rootPath, String champion)
			throws InterruptedException, IOException {
		for (int skinNumber = 0; skinNumber <= map.get(champion); skinNumber++) {
			if (new File(rootPath + CHARACTERPATH + champion + SKINPATH + skinNumber + ".bin").exists()) {
				ExecuteRitobin.startProgPytoBin(champion, skinNumber, rootPath);
				Thread.sleep(50);
			}
		}
	}

	private Control() {

	}
}
