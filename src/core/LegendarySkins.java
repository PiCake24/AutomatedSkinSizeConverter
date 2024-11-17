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
	public static void checkForLegendary(String champion, String rootPath, String cliPath) {
		// sona
		Gui.updateLog("Checking if champion has a legendary skin");
		if (champion.equals("sona")) {
			// check for djsona1, 2, 3
			convertSona(rootPath, cliPath);
		} else {
			Gui.updateLog("No legendary found");
		}
	}

	private static void convertSona(String rootPath, String cliPath) {
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
				if (!map.containsKey(1)) {
					map.put(2, defaultsize);
				}
				if (!map.containsKey(1)) {
					map.put(3, defaultsize);
				}
			} catch (Exception e) {
				System.out.println(e);
				Gui.updateLog(e.getLocalizedMessage());
			}

		} else {
			Gui.updateLog("No size options for Sona legendary found. setting to default (4)");
			map.put(1, 4.);
			map.put(2, 4.);
			map.put(3, 4.);
		}
		for (Map.Entry<Integer, Double> entry : map.entrySet()) {
			int element = entry.getKey();
			ExecuteProgramm.startProgBinToPy("sonadjgenre0" + element, 6, rootPath, cliPath); // TODO

			// TODO write, export
			try {
				Thread.sleep(2000);
			} catch (InterruptedException e) {
				Gui.updateLog(e.getLocalizedMessage());
			}
			// write into created file
			WriteIntoPy.writeInto("sonadjgenre0" + element, 6, entry.getValue(), rootPath);
			startProgPytoBinSona(element, rootPath, cliPath);
		}
		// sonadjgenere01
		// senadjgenre02
		// senadjgenre03
	}

	public static void startProgPytoBinSona(int element, String rootPath, String cliPath) {
		try {
			List<String> l = new ArrayList<>();
			l.add(cliPath);
			l.add(rootPath + "\\0WADS\\data\\characters\\sonadjgenre0" + element + "\\skins\\skin6.py");
			l.add(rootPath + "\\sona.wad.client\\data\\characters\\sonadjgenre0" + element + "\\skins\\skin6.bin");
			new ProcessBuilder(l).start();
			Gui.updateLog("sonadjgenre0" + element + ".bin created");
		} catch (IOException e) {
			Gui.updateLog(e.getLocalizedMessage());
		}
	}

}

//	private static void convertLux(String rootPath, String cliPath) {
//		Map<String, Double> map = new HashMap<>();
//		File file = new File(rootPath + "\\0PutOptionFilesHere\\luxlegendary.txt");
//		if (file.exists()) {
//			Gui.updateLog("Size options for Lux legendary found");
//			try (BufferedReader read = new BufferedReader(new FileReader(file))) {
//
//				String elemSize = read.readLine();
//				double defaultsize = Double.parseDouble(elemSize.split(":")[1].trim());
//				while ((elemSize = read.readLine()) != null) {
//					String element = elemSize.split(":")[0].trim();
//					double size = Double.parseDouble(elemSize.split(":")[1].trim());
//					map.put(element, size);
//				}
//				// check if elements arent in map
//				// add them with default size
//				if (!map.containsKey("fire")) {
//					map.put("fire", defaultsize);
//				}
//				if (!map.containsKey("water")) {
//					map.put("water", defaultsize);
//				}
//				if (!map.containsKey("air")) {
//					map.put("air", defaultsize);
//				}
//				if (!map.containsKey("ice")) {
//					map.put("ice", defaultsize);
//				}
//				if (!map.containsKey("dark")) {
//					map.put("dark", defaultsize);
//				}
//				if (!map.containsKey("magma")) {
//					map.put("magma", defaultsize);
//				}
//				if (!map.containsKey("nature")) {
//					map.put("nature", defaultsize);
//				}
//				if (!map.containsKey("mystic")) {
//					map.put("mystic", defaultsize);
//				}
//				if (!map.containsKey("storm")) {
//					map.put("storm", defaultsize);
//				}
//			} catch (Exception e) {
//				System.out.println(e);
//				Gui.updateLog(e.getLocalizedMessage());
//			}
//		} else {
//			map.put("fire", 5.);
//			map.put("air", 5.);
//			map.put("ice", 5.);
//			map.put("water", 5.);
//			map.put("dark", 5.);
//			map.put("magma", 5.);
//			map.put("nature", 5.);
//			map.put("mystic", 5.);
//			map.put("storm", 5.);
//		}
//		for (Map.Entry<String, Double> entry : map.entrySet()) {
//			String element = entry.getKey();
//			ExecuteProgramm.startProgBinToPy("lux" + element, 7, rootPath, cliPath);
//			try {
//				Thread.sleep(2000);
//			} catch (InterruptedException e) {
//				Gui.updateLog(e.getLocalizedMessage());
//			}
//			// write into created file
//			WriteIntoPy.writeInto("lux" + element, 7, entry.getValue(), rootPath);
//			startProgPytoBinLux("lux", "lux" + element, 7, rootPath, cliPath);
//			// create file in lux folder
//
//		}
//		// skins being air, dark,fire, ice, magma, mystic, nature, storm, water
//
//	}
//
//	public static void startProgPytoBinLux(String champion, String element, int skinNumber, String rootPath,
//			String cliPath) {
//		try {
//			List<String> l = new ArrayList<>();
//			l.add(cliPath);
//			l.add(rootPath + "\\0WADS\\data\\characters\\" + element + "\\skins\\skin" + skinNumber + ".py");
//			l.add(rootPath + "\\" + champion + ".wad.client\\data\\characters\\" + element + "\\skins\\skin"
//					+ skinNumber + ".bin");
//			new ProcessBuilder(l).start();
//			Gui.updateLog(element + " " + skinNumber + " .bin created");
//		} catch (IOException e) {
//			Gui.updateLog(e.getLocalizedMessage());
//		}
//	}
//}
