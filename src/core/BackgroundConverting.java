package core;

import java.io.IOException;

import javax.swing.SwingWorker;

/**
 * Gets called when the Convert Chosen Champions button gets pressed. Calls the
 * main control routine
 */
public class BackgroundConverting extends SwingWorker<String, String> {

	@Override
	protected String doInBackground() throws Exception {
		try {
			Logger.getInstance().startLog();
			Control.control();
		} catch (IOException | InterruptedException e) {
			Gui.updateLog(e.getMessage());
			Logger.getInstance().log(e.getMessage());
			e.printStackTrace();
			UnpackExe.removeCDTBTranslator();
			UnpackExe.removeRitobin();
			Thread.currentThread().interrupt();
		} finally {
			Logger.getInstance().endLog();
			Gui.enableButtons();
		}
		return null;
	}
}
