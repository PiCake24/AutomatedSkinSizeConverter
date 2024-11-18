package core;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

import javax.swing.SwingWorker;

public class CreateFolders extends SwingWorker<String, String> {
	/**
	 * Gets called when the Create Folders button get pressed
	 */
	@Override
	protected String doInBackground() throws Exception {
		createFolders();
		Gui.enableButtons();
		return null;
	}

	/**
	 * Creates all needed folders
	 * 
	 * @throws IOException
	 */
	public void createFolders() throws IOException {
		Map<String, Integer> map = new HashMap<>();
		String[] paths = Control.getPaths();
		String rootPath = paths[0];
		getChampions(map, rootPath);

		File file = new File(rootPath + "\\0WADS");
		file.mkdir();
		file = new File(rootPath + "\\0PutOptionFilesHere");
		file.mkdir();
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			file = new File(rootPath + "\\" + champion + ".wad.client\\data\\characters\\" + champion + "\\skins");
			file.mkdirs();
		}

	}

	private static void getChampions(Map<String, Integer> map, String rootPath) throws IOException { // TODO rework into
																										// better shape
		try (BufferedReader read = new BufferedReader(new FileReader(new File("Options.txt")))) {
			read.readLine();
			read.readLine();
			read.readLine();
			String line;
			while ((line = read.readLine()) != null) {
				if (line.charAt(0) != '#' && !line.trim().equals("")) {
					String[] split = line.split(" ");
					map.put(split[0].toLowerCase(), 1);
				}

			}
		} catch (IOException e) {
			System.out.println(e);
			throw new IOException();
		}
	}
}
