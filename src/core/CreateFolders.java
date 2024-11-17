package core;

import java.io.File;
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
	 */
	public void createFolders() {
		Map<String, Integer> map = new HashMap<>();
		String[] paths = Control.getPaths();
		String rootPath = paths[0];
		Control.getChampions(map, rootPath);

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
}
