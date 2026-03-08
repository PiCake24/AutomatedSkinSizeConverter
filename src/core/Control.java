package core;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
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
	 * @throws URISyntaxException
	 */
	static void control() throws IOException, InterruptedException {
		Map<String, Integer> map = new HashMap<>();
		String[] paths = getPaths();
		if (paths.length == 0) {
			return;
		}
		String rootPath = paths[0];
		String leaguePath = paths[1];
		String csLolPath = paths[2];

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

//			UnpackExe.removeCDTBTranslator();
		}

		getActualNumberOfSkins(map, rootPath);

		translateAndRewriteFiles(map, rootPath);

//		UnpackExe.removeRitobin();

		if (Gui.getImportIntoCsLolCheckBox()) {

			CslolIntegrator.createCslolMods(map, rootPath, csLolPath);
		}

		Gui.updateLog("Done");
	}

	/**
	 * Reads the paths from the options file and checks if they exist
	 * 
	 * @return
	 * @throws IOException
	 */
	static String[] getPaths() throws IOException {
		String[] paths = new String[2];
		try (BufferedReader read = new BufferedReader(new FileReader(new File(OPTIONS)))) {
			String s1 = read.readLine();
			String s2 = read.readLine();
			String s3 = read.readLine();

			String[] split1 = s1.split(":");
			if (!split1[0].equals("Root Path")) {
				Gui.updateLog("No Root Path option in options file. Trying to add it to the file");
				addRootPath();
				return new String[0];
			}
			if (split1[1].trim().equals("")) {
				Gui.updateLog("No Root Path entered in options file. Please add a Root Path to the options file");
				return new String[0];
			}
			if (split1.length < 3) {
				Gui.updateLog("The Root Path is not set correctly");
				return new String[0];
			}
			s1 = split1[1] + ":" + split1[2];

			String[] split2 = s2.split(":");
			if (!split2[0].equals("League Path")) {
				Gui.updateLog("No League Path option in options file. Trying to add it to the file");
				addLeaguePath();
				return new String[0];
			}
			if (split2[1].trim().equals("")) {
				Gui.updateLog("No League Path entered in options file. Please add a Root Path to the options file");
				return new String[0];
			}
			if (split2.length < 3) {
				Gui.updateLog("The League Path is not set correctly");
				return new String[0];
			}
			s2 = split2[1] + ":" + split2[2];

			String[] split3 = s3.split(":");
			if (!split3[0].equals("CsLol Path")) {
				Gui.updateLog("No CsLol Path option in options file. Trying to add it to the file");
				addCsLolPath();
				if (Gui.getImportIntoCsLolCheckBox()) {
					Gui.updateLog(
							"Added CsLol Path line to Options file. Please open the file and add a value behind it");
					return new String[0];
				}
				Gui.updateLog("Added CsLol Path line to Options file.");
			}
			if (Gui.getImportIntoCsLolCheckBox()) {
				if (split3[1].trim().equals("")) {
					Gui.updateLog("No CsLol Path entered in options file. Please add a Root Path to the options file");
					return new String[0];
				}
				if (split3.length < 3) {
					Gui.updateLog("The CsLol Path is not set correctly");
					return new String[0];
				}
				s3 = split3[1] + ":" + split3[2];
			}
			paths = new String[3];
			paths[0] = s1.trim();
			paths[1] = s2.trim();
			paths[1] += "\\DATA\\FINAL\\Champions";
			paths[2] = s3.trim();
			paths[2] += "\\installed";
		}
		Gui.updateLog("Root Path:" + paths[0]);
		Gui.updateLog("League Path: " + paths[1]);
		if (Gui.getImportIntoCsLolCheckBox()) {
			Gui.updateLog("CsLol Path:" + paths[2]);
		}
		return paths;

	}

	/**
	 * Adds a root path placeholder to the option file if it did not exist already
	 * 
	 * @throws IOException
	 */
	private static void addRootPath() throws IOException {
		List<String> list = new ArrayList<>();
		try (BufferedReader in = new BufferedReader(new FileReader(OPTIONS))) {
			String line;
			list.add("Root Path: ");
			while ((line = in.readLine()) != null) {
				list.add(line);
			}
		}
		String outputString = String.join("\n", list);
		try (Writer w = new BufferedWriter(new FileWriter(OPTIONS))) {
			w.write(outputString);
		}
		Gui.updateLog("Added Root Path line to Options file. Please open the file and add a value behind it");
	}

	/**
	 * Adds a league path placeholder to the option file if it did not exist already
	 * 
	 * @throws IOException
	 */
	private static void addLeaguePath() throws IOException {
		List<String> list = new ArrayList<>();
		try (BufferedReader in = new BufferedReader(new FileReader(OPTIONS))) {
			String line;
			list.add(in.readLine());
			list.add("League Path: ");
			while ((line = in.readLine()) != null) {
				list.add(line);
			}
		}
		String outputString = String.join("\n", list);
		try (Writer w = new BufferedWriter(new FileWriter(OPTIONS))) {
			w.write(outputString);
		}
		Gui.updateLog("Added League Path line to Options file. Please open the file and add a value behind it");
	}

	/**
	 * Adds a CsLol path placeholder to the option file if it did not exist already
	 * 
	 * @throws IOException
	 */
	private static void addCsLolPath() throws IOException {
		List<String> list = new ArrayList<>();
		try (BufferedReader in = new BufferedReader(new FileReader(OPTIONS))) {
			String line;
			list.add(in.readLine());
			list.add(in.readLine());
			list.add("CsLol Path: ");
			while ((line = in.readLine()) != null) {
				list.add(line);
			}
		}
		String outputString = String.join("\n", list);
		try (Writer w = new BufferedWriter(new FileWriter(OPTIONS))) {
			w.write(outputString);
		}
	}

	/**
	 * Adds champions to the map
	 * 
	 * @param map
	 * @param rootPath
	 */
	private static void addChampionsToMap(Map<String, Integer> map) throws IOException {
		try (BufferedReader read = new BufferedReader(new FileReader(new File(OPTIONS)))) {
			Gui.updateLog("Reading champions file");
			String line;
			for (int i = 0; i < 3; i++) {
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
		ResourceMonitor.startCpuMonitor();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			convertToPython(map, rootPath, champion);
		}
		ResourceMonitor.stopCpuMonitor();
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
				Thread.sleep(10);
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
				Thread.sleep(10);
			}
		}
	}

	private Control() {

	}
}
