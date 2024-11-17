package core;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.Writer;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class WriteIntoPy {

	public static void writeInto(String champion, int SkinNumber, double scale, String rootPath) {
		File file = new File(
				rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + SkinNumber + ".py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			Gui.updateLog("Writing into " + champion + " " + SkinNumber + " with size " + scale);
			List<String> list = new ArrayList<String>();
			String b;
			while ((b = in.readLine()) != null) {
				list.add(b);
				if (b.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
					break;
				}
			}
			list.add(in.readLine());
			list.add(in.readLine());
			list.add(in.readLine());

			list.add("\t \t \tskinScale: f32 = " + scale);
			while ((b = in.readLine()) != null) {
				if (b.contains("skinScale")) {
					continue;
				}
				list.add(b);
			}
			Gui.updateLog(champion + " " + SkinNumber + " overwritten");

			String outputString = String.join("\n", list);
			Writer w = new BufferedWriter(new FileWriter(file));
			w.write(outputString);
			w.close();
		} catch (Exception e) {
			Gui.updateLog(e.getLocalizedMessage());
		}
	}

	public static void writeLuxLegendaryIntoPy(String rootPath) {
		Map<String, Double> map = new HashMap<>();
		File file = new File(rootPath + "\\0PutOptionFilesHere\\luxlegendary.txt");
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
				// add them with default size
				if (!map.containsKey("light")) {
					map.put("light", defaultsize);
				}
				if (!map.containsKey("fire")) {
					map.put("fire", defaultsize);
				}
				if (!map.containsKey("water")) {
					map.put("water", defaultsize);
				}
				if (!map.containsKey("air")) {
					map.put("air", defaultsize);
				}
				if (!map.containsKey("ice")) {
					map.put("ice", defaultsize);
				}
				if (!map.containsKey("dark")) {
					map.put("dark", defaultsize);
				}
				if (!map.containsKey("magma")) {
					map.put("magma", defaultsize);
				}
				if (!map.containsKey("nature")) {
					map.put("nature", defaultsize);
				}
				if (!map.containsKey("mystic")) {
					map.put("mystic", defaultsize);
				}
				if (!map.containsKey("storm")) {
					map.put("storm", defaultsize);
				}
			} catch (Exception e) {
				System.out.println(e);
				Gui.updateLog(e.getLocalizedMessage());
			}
		} else {
			Gui.updateLog("No size options for Lux legendary found, setting to default(5)");
			map.put("light", 5.);
			map.put("fire", 5.);
			map.put("air", 5.);
			map.put("ice", 5.);
			map.put("water", 5.);
			map.put("dark", 5.);
			map.put("magma", 5.);
			map.put("nature", 5.);
			map.put("mystic", 5.);
			map.put("storm", 5.);
		}
		writeLux(rootPath, map);
	}

	private static void writeLux(String rootPath, Map<String, Double> map) {
		List<String> order = new ArrayList<>();
		order.add("light");
		order.add("magma");
		order.add("light");
		order.add("dark");
		order.add("nature");
		order.add("fire");
		order.add("mystic");
		order.add("storm");
		order.add("water");
		order.add("ice");
		order.add("air");

		File file = new File(rootPath + "\\0WADS\\data\\characters\\lux\\skins\\skin7.py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			List<String> list = new ArrayList<String>();
			String b;
			for (int i = 0; i < 11; i++) {
				Gui.updateLog("Writing into Lux " + order.get(i) + " with size " + map.get(order.get(i)));
				while ((b = in.readLine()) != null) {

					list.add(b);
					if (b.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
						break;
					}
				}
				list.add(in.readLine());
				list.add(in.readLine());
				list.add(in.readLine());
				list.add("\t \t \tskinScale: f32 = " + map.get(order.get(i)));
				b = in.readLine();
				if (!b.contains("skinScale")) {
					list.add(b);
				}

				Gui.updateLog("Lux " + order.get(i) + " overwritten");
			}
			while ((b = in.readLine()) != null) {
				list.add(b);
			}
			String a = String.join("\n", list);
			Writer w = new BufferedWriter(new FileWriter(file));
			w.write(a);
			w.close();
		} catch (Exception e) {
			System.out.println(e);
			Gui.updateLog(e.getLocalizedMessage());
		}
	}
	// light, magme, light, dark, nature, fire, mystic, storm, water, frost, air
}
