package core;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class WriteIntoPy {

	private static final String LIGHT = "light";
	private static final String FIRE = "fire";
	private static final String WATER = "water";
	private static final String AIR = "air";
	private static final String ICE = "ice";
	private static final String DARK = "dark";
	private static final String MAGMA = "magma";
	private static final String NATURE = "nature";
	private static final String MYSTIC = "mystic";
	private static final String STORM = "storm";

	/**
	 * Reads the champion file and writes the new size into it
	 * 
	 * @param champion
	 * @param skinNumber
	 * @param scale
	 * @param rootPath
	 * @throws IOException
	 */
	public static void writeInto(String champion, int skinNumber, double scale, String rootPath) throws IOException {
		File file = new File(
				rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + skinNumber + ".py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			Gui.updateLog("Writing into " + champion + " " + skinNumber + " with size " + scale);
			List<String> list = new ArrayList<>();
			String line;
			while ((line = in.readLine()) != null) {
				if (line.trim().contains("skinScale")) {
					list.add("\t \t \tskinScale: f32 = " + scale);
				} else {
					list.add(line);
				}
//				list.add(line);
//				if (line.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
//					break;
//				}
			}
//			for (int i = 0; i < 6; i++) {
//				line = in.readLine();
//
//			}
//			while ((line = in.readLine()) != null) {
//				list.add(line);
//			}
			Gui.updateLog(champion + " " + skinNumber + " overwritten");

			String outputString = String.join("\n", list);
			try (Writer w = new BufferedWriter(new FileWriter(file))) {
				w.write(outputString);
			}
		}
	}

	/**
	 * Prepares writing Elementalist Lux
	 * 
	 * @param rootPath
	 * @throws IOException
	 */
	public static void prepWriteLuxLegendaryIntoPy(String rootPath) throws IOException {
		Map<String, Double> map = new HashMap<>();
		File file = new File(rootPath + "\\0PutSizeOptionFilesHere\\luxlegendary.txt");
		if (file.exists()) {
			Gui.updateLog("Size options for Lux legendary found");
			try (BufferedReader read = new BufferedReader(new FileReader(file))) {
				String elemSize = read.readLine();
				double defaultsize = Double.parseDouble(elemSize.split(":")[1].trim());
				while ((elemSize = read.readLine()) != null) {
					String element = elemSize.split(":")[0].trim();
					double size = Double.parseDouble(elemSize.split(":")[1].trim());
					map.put(element, size);
				}
				// check if elements arent in map
				addMapDefaultSize(map, defaultsize);
			}
		} else {
			Gui.updateLog("No size options for Lux legendary found, setting to default(5)");
			map.put(LIGHT, 5.);
			map.put(FIRE, 5.);
			map.put(WATER, 5.);
			map.put(AIR, 5.);
			map.put(ICE, 5.);
			map.put(DARK, 5.);
			map.put(MAGMA, 5.);
			map.put(NATURE, 5.);
			map.put(MYSTIC, 5.);
			map.put(STORM, 5.);
		}
		writeLux(rootPath, map);
	}

	/**
	 * Adds sizes to default, when they are not set in the optionsfile
	 * 
	 * @param map
	 * @param defaultsize
	 */
	static void addMapDefaultSize(Map<String, Double> map, double defaultsize) {
		if (!map.containsKey(LIGHT)) {
			map.put(LIGHT, defaultsize);
		}
		if (!map.containsKey(FIRE)) {
			map.put(FIRE, defaultsize);
		}
		if (!map.containsKey(WATER)) {
			map.put(WATER, defaultsize);
		}
		if (!map.containsKey(AIR)) {
			map.put(AIR, defaultsize);
		}
		if (!map.containsKey(ICE)) {
			map.put(ICE, defaultsize);
		}
		if (!map.containsKey(DARK)) {
			map.put(DARK, defaultsize);
		}
		if (!map.containsKey(MAGMA)) {
			map.put(MAGMA, defaultsize);
		}
		if (!map.containsKey(NATURE)) {
			map.put(NATURE, defaultsize);
		}
		if (!map.containsKey(MYSTIC)) {
			map.put(MYSTIC, defaultsize);
		}
		if (!map.containsKey(STORM)) {
			map.put(STORM, defaultsize);
		}
	}

	/**
	 * Writes Elementalist Lux
	 * 
	 * @param rootPath
	 * @param map
	 * @throws IOException
	 */
	private static void writeLux(String rootPath, Map<String, Double> map) throws IOException {
		List<String> order = new ArrayList<>();
		order.add(LIGHT);
		order.add(MAGMA);
		order.add(LIGHT);
		order.add(DARK);
		order.add(NATURE);
		order.add(FIRE);
		order.add(MYSTIC);
		order.add(STORM);
		order.add(WATER);
		order.add(ICE);
		order.add(AIR);

		File file = new File(rootPath + "\\0WADS\\data\\characters\\lux\\skins\\skin7.py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			List<String> list = new ArrayList<>();
			String line;
			for (int i = 0; i < 11; i++) {
				Gui.updateLog("Writing into Lux " + order.get(i) + " with size " + map.get(order.get(i)));
				while ((line = in.readLine()) != null) {

					list.add(line);
					if (line.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
						break;
					}
				}
				list.add(in.readLine());
				list.add(in.readLine());
				list.add(in.readLine());
				list.add("\t \t \tskinScale: f32 = " + map.get(order.get(i)));
				line = in.readLine();
				if (!line.contains("skinScale")) {
					list.add(line);
				}

				Gui.updateLog("Lux " + order.get(i) + " overwritten");
			}
			while ((line = in.readLine()) != null) {
				list.add(line);
			}
			String a = String.join("\n", list);
			try (Writer w = new BufferedWriter(new FileWriter(file))) {
				w.write(a);
			}
		}
	}

	private WriteIntoPy() {
	}
}
