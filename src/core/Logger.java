package core;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Date;

public class Logger {
	private static Logger instance;
	private boolean doLog;
	private BufferedWriter writer;

	public static Logger getInstance() {
		if (instance == null) {
			instance = new Logger();
		}
		return instance;
	}

	private Logger() {
	}

	public void startLog() {
		this.doLog = Gui.getLogCheckBox();
		if (doLog) {
			File logFile = new File("log" + new SimpleDateFormat("yyyyMMddHHmmssSSS").format(new Date()) + ".txt");
			try {
				if (logFile.createNewFile()) {
					writer = new BufferedWriter(new FileWriter(logFile, true));
				}
			} catch (IOException e) {
				// nothing to do if it fails
			}
		}
	}

	public void log(String message) {
		if (doLog) {
			StringBuilder builder = new StringBuilder();
			try {
				builder.append(new SimpleDateFormat("yyyy-MM-dd-HH-mm-ss-SSS: ").format(new Date()));
				builder.append(message);
				builder.append("\n");
				writer.write(builder.toString());
				writer.flush();
			} catch (IOException e) {
				// nothing to do if this fails
			}
		}

	}
}
