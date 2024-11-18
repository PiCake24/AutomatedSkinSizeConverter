package core;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Map;
import java.util.Set;

public class CDTBExecution {
	static boolean downloadHashes() throws IOException, InterruptedException {
		String pythonScript = UnpackExe.getUnpackedCDTBTranslator().toString();

		ProcessBuilder pb = new ProcessBuilder(pythonScript, "download_hashes");
		Process process = pb.start();
		return printProcessOutput(process);
	}

	static boolean extractFiles(Map<String, Integer> map, String leaguePath, String rootPath)
			throws IOException, InterruptedException {
		Set<String> set = map.keySet();
		for (int championNumber = 0; championNumber < map.size(); championNumber++) {
			String champion = (String) set.toArray()[championNumber];
			if (!extractFile(champion, leaguePath, rootPath)) {
				return false;
			}
		}
		return true;

	}

	private static boolean extractFile(String champion, String leaguePath, String rootPath)
			throws IOException, InterruptedException {
		String pythonScript = UnpackExe.getUnpackedCDTBTranslator().toString();
		String inputPath = leaguePath + "\\" + champion + ".wad.client";
		String outputPath = rootPath + "\\0WADS";
		ProcessBuilder pb2 = new ProcessBuilder(pythonScript, "unpack_file", inputPath, outputPath);
		Process process2 = pb2.start();
		return printProcessOutput(process2);
	}

	private static boolean printProcessOutput(Process process) throws IOException, InterruptedException {
		BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
		String line;
		while ((line = reader.readLine()) != null) {
			System.out.println(line);
		}
		int exitCode = process.waitFor();
		return exitCode == 0;
	}

	private CDTBExecution() {
	}
}
