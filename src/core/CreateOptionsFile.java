package core;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

import javax.swing.SwingWorker;

public class CreateOptionsFile extends SwingWorker<String, String> {
	/**
	 * Gets called when the Create Options File button gets pressed
	 */
	@Override
	protected String doInBackground() throws Exception {
		File file = createFile();
		writeFile(file);
		Gui.enableButtons();
		return null;
	}

	/**
	 * Creates a new Options.txt file
	 * 
	 * @return
	 */
	public static File createFile() {
		File file = new File("Options.txt");
		try {
			if (file.createNewFile()) {
				Gui.updateLog("File created");
			} else {
				Gui.updateLog("Could not create file");
			}
		} catch (IOException e) {
			Gui.updateLog("Could not create file");
		}
		return file;
	}

	/**
	 * Writes the default Options text into the file
	 * 
	 * @param file
	 */
	public static void writeFile(File file) {
		try (FileWriter f = new FileWriter(file)) {
			f.write("Root Path: \nRitobin Cli Path: \nRiot Games Path: \n#Everything that starts with a # will be ignored. Add The champion names and their numbers of skins in the lines below this line(separated by a space). One champion per line\n#Ahri 61");
			Gui.updateLog("File Overwritten");
		} catch (IOException e) {
			Gui.updateLog("Could not edit file");
		}
	}
}
