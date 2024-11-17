package core;

import javax.swing.SwingWorker;

public class BackgroundConverting extends SwingWorker<String, String> {

	@Override
	protected String doInBackground() throws Exception {
		Control.control();
		Gui.enableButtons();
		return null;
	}
}
