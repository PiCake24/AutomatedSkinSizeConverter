package core;
import oshi.SystemInfo;
import oshi.hardware.CentralProcessor;
import oshi.hardware.GlobalMemory;

public class test {
	public static void main(String[] args) throws InterruptedException {
		// Create a SystemInfo object to access system hardware info
		SystemInfo si = new SystemInfo();
		while (true) {
			// Get the processor information
			CentralProcessor processor = si.getHardware().getProcessor();

			// Get the memory information
			GlobalMemory memory = si.getHardware().getMemory();

			// CPU Load (total CPU load across all cores)
			double cpuLoad = processor.getSystemCpuLoad(1000) * 100; // Get system-wide CPU load (percentage)
			System.out.println("CPU Load: " + cpuLoad + "%");

			// Memory Usage
			System.out.println("Total memory: " + memory.getTotal() / (1024 * 1024) + " MB");
			System.out.println("Available memory: " + memory.getAvailable() / (1024 * 1024) + " MB");
			System.out.println("Used memory: " + (memory.getTotal() - memory.getAvailable()) / (1024 * 1024) + " MB");
//			Thread.sleep(1000);
		}
	}
}
