package core;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

import javax.swing.SwingWorker;

public class CreateOptionsFile extends SwingWorker<String, String> {
	@Override
	protected String doInBackground() throws Exception {
		File file = createFile();
		writeFile(file);
		Gui.enableButtons();
		return null;
	}

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

	public static void writeFile(File file) {
		try (FileWriter f = new FileWriter(file)) {
			f.write("Root Path: \nRitobin Cli Path: \nRiot Games Path: \n#Everything that starts with a # will be ignored. Add The champion names and their numbers of skins in the lines below this line(separated by a space). One champion per line\n#Ahri 61");
			Gui.updateLog("File Overwritten");
		} catch (IOException e) {
			Gui.updateLog("Could not edit file");
		}
	}
}
