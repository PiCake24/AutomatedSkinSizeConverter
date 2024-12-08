package core;

import java.io.IOException;
import java.util.Map;
import java.util.Set;

public class CDTBExecution {
	/**
	 * Downloads the newest hashes
	 * 
	 * @return
	 * @throws IOException
	 * @throws InterruptedException
	 */
	static boolean downloadHashes() throws IOException, InterruptedException {
		String pythonScript = UnpackExe.getUnpackedCDTBTranslator().toString();

		ProcessBuilder pb = new ProcessBuilder(pythonScript, "download_hashes");
		Process process = pb.start();
		return printProcessOutput(process);
	}

	/**
	 * Extracts all league files into the 0WADS folder
	 * 
	 * @param map
	 * @param leaguePath
	 * @param rootPath
	 * @return
	 * @throws IOException
	 * @throws InterruptedException
	 */
	static boolean extractAllFiles(Map<String, Integer> map, String leaguePath, String rootPath)
			throws IOException, InterruptedException {
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			Gui.updateLog("Downloading " + champion);
			if (!extractFile(champion, leaguePath, rootPath)) {
				return false;
			}
		}
		return true;
	}

	/**
	 * Extracts a single champion into the 0WADS folder
	 * 
	 * @param champion
	 * @param leaguePath
	 * @param rootPath
	 * @return
	 * @throws IOException
	 * @throws InterruptedException
	 */
	private static boolean extractFile(String champion, String leaguePath, String rootPath)
			throws IOException, InterruptedException {
		String pythonScript = UnpackExe.getUnpackedCDTBTranslator().toString();
		String inputPath = leaguePath + "\\" + champion + ".wad.client";
		String outputPath = rootPath + "\\0WADS\\";
		String pattern = "*data*skins/*";
		ProcessBuilder pb2 = new ProcessBuilder(pythonScript, "unpack_file", inputPath, outputPath, pattern);
		Process process2 = pb2.start();
		return printProcessOutput(process2);
	}

	/**
	 * Prints the process output (exitcode) and checks if the program ran
	 * successfully
	 * 
	 * @param process
	 * @return
	 * @throws IOException
	 * @throws InterruptedException
	 */
	private static boolean printProcessOutput(Process process) throws InterruptedException {
		int exitCode = process.waitFor();
		return exitCode == 0;
	}

	private CDTBExecution() {
	}
}
