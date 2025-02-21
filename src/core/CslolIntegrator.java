package core;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class CslolIntegrator {
	// TODO options file check und gegebenfalls anpassen
	// TODO mit knopf aktivieren
	// TODO logging and messages
	// todo: doc

	/**
	 * Creates mods in cslol
	 * 
	 * @param map
	 * @param rootPath
	 * @param csLolPath
	 * @throws IOException
	 * @throws InterruptedException
	 */
	public static void createCslolMods(Map<String, Integer> map, String rootPath, String csLolPath)
			throws IOException, InterruptedException {
		if (!UnpackExe.unpackWadMake()) {
			return;
		}
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			createWad(champion, rootPath);
			createFolders(champion, csLolPath);
		}
		Thread.sleep(2000);
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			createMeta(champion, csLolPath);
			movewad(champion, rootPath, csLolPath);
		}
		Thread.sleep(2000);
		UnpackExe.removeWadMake();
	}

	/**
	 * creates the needed folders in the cslol directory
	 * 
	 * @param champion
	 * @param cSlolPath
	 */
	private static void createFolders(String champion, String csLolPath) {
		Gui.updateLog("Creating cslol folders for " + champion);
		File file = new File(csLolPath + "\\giant " + champion + "\\META");
		file.mkdirs();
		file = new File(csLolPath + "\\giant " + champion + "\\WAD");
		file.mkdir();
	}

	/**
	 * Uses make-wad to create wad files from the directories
	 * 
	 * @param champion
	 * @throws IOException
	 */
	private static void createWad(String champion, String rootPath) throws IOException {
		Gui.updateLog("creating " + champion + " wad");
		String wadMake = UnpackExe.getUnpackedWadMake().toString();
		List<String> l = new ArrayList<>();
		l.add(wadMake);
		l.add(rootPath + "\\" + champion + ".wad.client");
		new ProcessBuilder(l).start();
	}

	/**
	 * Moves wad files to the correct folder and renames them accordingly
	 * 
	 * @param champion
	 * @param rootPath
	 * @param csLolPath
	 * @throws IOException
	 */
	private static void movewad(String champion, String rootPath, String csLolPath) throws IOException {
		Gui.updateLog("Moving " + champion + " wad");
		Path sourcePath = Path.of(rootPath + "\\" + champion + ".wad.client.wad.client");
		Path targetPath = Path.of(csLolPath + "\\giant " + champion + "\\WAD\\" + champion + ".wad.client");
		Files.move(sourcePath, targetPath, StandardCopyOption.REPLACE_EXISTING);
	}

	/**
	 * Creates the META information of the mods
	 * 
	 * @param champion
	 * @param cslolPath
	 * @throws IOException
	 */
	private static void createMeta(String champion, String cslolPath) throws IOException {
		Gui.updateLog("Creating META files");
		File file = new File(cslolPath + "\\giant " + champion + "\\META\\info.json");
		if (file.createNewFile()) {
			String s = "{\r\n" + "    \"Author\": \"AutomatedSkinSizeConverter\",\r\n"
					+ "    \"Description\": \"\",\r\n" + "    \"Heart\": \"\",\r\n" + "    \"Home\": \"\",\r\n"
					+ "    \"Name\": \"giant " + champion + "\",\r\n" + "    \"Version\": \"1\"\r\n" + "}";
			try (BufferedWriter out = new BufferedWriter(new FileWriter(file))) {
				out.write(s);
			}
		}
	}

}
