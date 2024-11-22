package core;

import java.io.File;
import java.io.IOException;

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
		String[] paths = Control.getPaths();
		String rootPath = paths[0];

		File file = new File(rootPath + "\\0WADS");
		file.mkdir();
		file = new File(rootPath + "\\0PutOptionFilesHere");
		file.mkdir();
	}
}
