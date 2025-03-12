package core;

import oshi.SystemInfo;
import oshi.hardware.CentralProcessor;

public class ResourceMonitor {

	private static volatile double latestCpuLoad = 0.0;
	private static volatile boolean running = false; // Control flag
	private static Thread monitorThread = null; // Reference to the monitoring thread

	public static void startCpuMonitor() {
		if (monitorThread != null && monitorThread.isAlive()) {
			return;
		}

		running = true;
		monitorThread = new Thread(() -> {
			SystemInfo systemInfo = new SystemInfo();
			CentralProcessor processor = systemInfo.getHardware().getProcessor();

			while (running) {
				double load = processor.getSystemCpuLoad(1000);
				latestCpuLoad = load * 100;
				try {
					Thread.sleep(10);
				} catch (InterruptedException e) {
					Thread.currentThread().interrupt();
					break;
				}
			}
		});

		monitorThread.setDaemon(true);
		monitorThread.start();
	}

	public static void stopCpuMonitor() {
		running = false;
		if (monitorThread != null) {
			monitorThread.interrupt();
		}
	}

	public static boolean isSystemResourcesAvailable() {
		return latestCpuLoad < 85.0;
	}
}
