package core;

import javax.swing.SwingWorker;

/**
 * Gets called when the Convert Chosen Champions button gets pressed. Calls
 * Control
 */
public class BackgroundConverting extends SwingWorker<String, String> {

	@Override
	protected String doInBackground() throws Exception {
		Control.control();
		Gui.enableButtons();
		return null;
	}
}
