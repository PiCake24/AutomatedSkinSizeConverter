package core;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class Control {

	public static void fillMap(Map<String, Integer> map, String rootPath) {
		try (BufferedReader read = new BufferedReader(new FileReader(new File("Options.txt")))) {
			Gui.updateLog("Reading options file");
			read.readLine();
			read.readLine();
			read.readLine();
			String line;
			while ((line = read.readLine()) != null) {
				if (line.charAt(0) != '#') {
					if (line.trim() != "") {
						String[] split = line.split(" ");
						if (split.length > 1) {
							map.put(split[0].toLowerCase(), Integer.parseInt(split[1]));
							Gui.updateLog(
									"Queuing up: " + split[0] + " with " + (Integer.parseInt(split[1]) + 1) + " skins");
						} else {
							System.out.println("n");
							int number = getNumberOfSkins(split[0].toLowerCase(), rootPath);
							map.put(split[0].toLowerCase(), number);
							Gui.updateLog("Queuing up: " + split[0] + " with " + (number + 1) + " skins");
						}
					}

				}

			}
		} catch (IOException e) {
			System.out.println(e);
		}
	}

	public static void getChampions(Map<String, Integer> map, String rootPath) {
		try (BufferedReader read = new BufferedReader(new FileReader(new File("Options.txt")))) {
			read.readLine();
			read.readLine();
			read.readLine();
			String line;
			while ((line = read.readLine()) != null) {
				if (line.charAt(0) != '#') {
					System.out.println(2);
					if (line.trim() != "") {
						System.out.println(3);
						String[] split = line.split(" ");
						map.put(split[0].toLowerCase(), 1);
					}
				}
			}
		} catch (IOException e) {
			System.out.println(e);
		}
	}

	public static void control() {
		Map<String, Integer> map = new HashMap<>();
		String[] paths = getPaths();
		String rootPath = paths[0];
		String cliPath = paths[1];
		String leaguePath = paths[2];
		if (Gui.getCheckBoxBool()) {
			String pythonPath = GetPythonPath.getPythonPathWindows();
			getHashes(pythonPath);

			getChampions(map, rootPath);
			UnpackWAD.unpackWAD(map, pythonPath, leaguePath, rootPath);
		}
		fillMap(map, rootPath);
		copyMapFiles(map, rootPath, cliPath);
		System.out.println("Done");
		Gui.updateLog("Done");
	}

	private static void getHashes(String pythonPath) {
		Gui.updateLog("Getting Hashes");

		ProcessBuilder builder = new ProcessBuilder();
		builder.directory(new File(pythonPath));
		Process process;
		try {
			process = builder.command("cmd.exe", "/c", "cdtb fetch-hashes").start();
			try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
					BufferedReader errorReader = new BufferedReader(new InputStreamReader(process.getErrorStream()))) {

				String line;
				while ((line = reader.readLine()) != null) {
					System.out.println(line); // Print the output to the console
				}
				while ((line = errorReader.readLine()) != null) {
					System.err.println(line); // Print the errors to the error stream
				}
			}
			int exitCode = process.waitFor();
		} catch (IOException | InterruptedException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
		Gui.updateLog("Completed Downloading Hashes");

	}

	public static void copyMapFiles(Map<String, Integer> map, String rootPath, String cliPath) {
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];

			convertToPython(map, rootPath, cliPath, champion);
			try {
				Thread.sleep(3000);
			} catch (InterruptedException e) {
				System.out.println(e);
			}
			rewriteFile(map, rootPath, cliPath, champion);
			try {
				Thread.sleep(1000);
			} catch (InterruptedException e) {
				System.out.println(e);
			}
			convertToWad(map, rootPath, cliPath, champion);

			LegendarySkins.checkForLegendary(champion, rootPath, cliPath);
		}
	}

	/**
	 * Reads the paths from the options file
	 * 
	 * @return
	 */
	public static String[] getPaths() {
		String[] paths = new String[2];
		try (BufferedReader read = new BufferedReader(new FileReader(new File("Options.txt")))) {
			String s1 = read.readLine();
			String s2 = read.readLine();
			String s3 = read.readLine();
			String[] split1 = s1.split(":");
			String[] split2 = s2.split(":");
			String[] split3 = s3.split(":");
			s1 = split1[1] + ":" + split1[2];
			s2 = split2[1] + ":" + split2[2];
			s3 = split3[1] + ":" + split3[2];
			paths = new String[3];
			paths[0] = s1.trim();
			paths[1] = s2.trim();
			paths[2] = s3.trim();
			paths[2] += "\\League of Legends\\Game\\DATA\\FINAL\\Champions";
		} catch (IOException e) {
			Gui.updateLog(e.getLocalizedMessage());
		}
		Gui.updateLog("Root Path:" + paths[0]);
		Gui.updateLog("Ritobin Cli Path:" + paths[1]);
		Gui.updateLog("League Path: " + paths[2]);
		return paths;
	}

	public static ArrayList<Double> getSize(String champion, int skinNumbers, String rootPath) {
		ArrayList<Double> sizes = new ArrayList<Double>();
		File file = new File(rootPath + "\\0PutOptionFilesHere\\" + champion + ".txt");
		if (file.exists()) {
			Gui.updateLog("Size Options found");
			try (BufferedReader read = new BufferedReader(new FileReader(file))) {
				// Add default size
				String size = read.readLine();
				String[] split = size.split(":");
				double defaulSize = Double.parseDouble(split[1].trim());
				int prevMinSkin = -1;

				while ((size = read.readLine()) != null) {
					if (size.trim() != "") {
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
			} catch (IOException e) {
				System.out.println(e);
				Gui.updateLog(e.getLocalizedMessage());
			}

		} else {
			Gui.updateLog("No size options found, setting size to default(5)");
			for (int i = 0; i <= skinNumbers; i++) {
				sizes.add(5.);
			}
		}
		return sizes;
	}

	public static int getNumberOfSkins(String champion, String rootPath) {
		// check if skin exists
		Gui.updateLog("getting number of skins");
		int numberOfCensecutiveTries = 0;
		int numberOfSkins = 0;
		while (numberOfCensecutiveTries < 50) {
			File f = new File(
					rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + numberOfSkins + ".bin");
			System.out.println(numberOfSkins);
			if (!f.exists()) {
				numberOfCensecutiveTries++;
				numberOfSkins++;
			} else {
				System.out.println("Exists");
				numberOfSkins++;
				numberOfCensecutiveTries = 0;
			}
		}
		System.out.println(numberOfSkins);
		numberOfSkins -= 51;

		return numberOfSkins;
	}

	private static void convertToPython(Map<String, Integer> map, String rootPath, String cliPath, String champion) {

		for (int skinNumber = 0; skinNumber <= map.get(champion); skinNumber++) {
			if (new File(rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".bin")
					.exists()) {
				// System.out.println("a");
				ExecuteProgramm.startProgBinToPy(champion, skinNumber, rootPath, cliPath);
				try {
					Thread.sleep(50);
				} catch (InterruptedException e) {
					System.out.println(e);
				}
//					
			} else {
				Gui.updateLog(champion + " " + skinNumber + " does not exist");
			}
		}
	}

	private static void rewriteFile(Map<String, Integer> map, String rootPath, String cliPath, String champion) {
		ArrayList<Double> sizes = getSize(champion, map.get(champion), rootPath);
		for (int skinNumber = 0; skinNumber <= map.get(champion); skinNumber++) {
			if (new File(rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".bin")
					.exists()) {
				if (champion.equals("lux") && skinNumber == 7) {
					WriteIntoPy.writeLuxLegendaryIntoPy(rootPath);
				} else {
					WriteIntoPy.writeInto(champion, skinNumber, sizes.get(skinNumber), rootPath);
				}
			}
		}
	}

	private static void convertToWad(Map<String, Integer> map, String rootPath, String cliPath, String champion) {
		for (int skinNumber = 0; skinNumber <= map.get(champion); skinNumber++) {
			if (new File(rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".bin")
					.exists()) {
				ExecuteProgramm.startProgPytoBin(champion, skinNumber, rootPath, cliPath);
				try {
					Thread.sleep(50);
				} catch (InterruptedException e) {
					System.out.println(e);
				}
			}
		}
	}
}
