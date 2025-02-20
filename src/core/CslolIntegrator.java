package core;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class CslolIntegrator {
	// create a cslol mod

	// weiteres TODO: in options file neue generation anpassen
	// TODO options file check und gegebenfalls anpassen
	// TODO mit knopf aktivieren
	// TODO unpack der wad-make.exr
	// TODO logging and messages

	// cslol-manager -> installed
	// look at the cslol folders, look for "giant champion"
	// meta folder _> info.json
	// WAD folder -> wad.client file
	// maybe use wad make.exe
	// run wad-make-multi.bat
	// oder direkt wad-make.bat
	// wad-make.exe "E:\Teststuff\briar"
	public static void createMods(Map<String, Integer> map, String csLolPath) throws IOException {
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			createFolders(champion, csLolPath);
			createWad(champion);
		}
	}

	private static void createFolders(String champion, String cSlolPath) {
		File file = new File(cSlolPath + "\\Giant " + champion + "\\META");
		file.mkdirs();
		file = new File(cSlolPath + "\\Giant " + champion + "\\WAD");
		file.mkdir();
	}

	private static void createWad(String champion) throws IOException {
		// run wad-make mit richtigen argumenten
		String wadMake = UnpackExe.getUnpackedWadMake().toString();
		List<String> l = new ArrayList<>();
		l.add(wadMake);
		l.add("E:\\Teststuff\\" + champion);
		new ProcessBuilder(l).start();
	}

	private static void movewad(String rootPath, String csLolPath) {
		// bewege wad files in ordner
	}

	private static void createMeta() {
		// kreiere json datei mit richtigem inhalt
	}

}
