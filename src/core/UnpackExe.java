package core;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

public class UnpackExe {
	private static final String TMPDIR = "java.io.tmpdir";
	private static Path unpackedRitobin;
	private static Path unpackedCDTBTranslator;
	private static Path unpackedFolder;
	private static Path unpackedWadMake;

	/**
	 * unpacks Ritobin to the tmp folder
	 * 
	 * @return
	 * @throws IOException
	 */
	static boolean unpackRitobin() throws IOException {
		unpackHashes();
		String tempDir = System.getProperty(TMPDIR);
		unpackedRitobin = Paths.get(tempDir, "extractedRitobin.exe");

		try (InputStream in = AutomationMain.class.getResourceAsStream("/ritobin_cli.exe");
				OutputStream out = Files.newOutputStream(unpackedRitobin)) {
			if (in == null) {
				Logger.getInstance().log("Resource not found: /ritobin_cli.exe");
				throw new IOException("Resource not found: /ritobin_cli.exe");
			}

			byte[] buffer = new byte[1024];
			int bytesRead;
			while ((bytesRead = in.read(buffer)) != -1) {
				out.write(buffer, 0, bytesRead);
			}
		}
		Logger.getInstance().log("Ritobin unpacked under:" + unpackedRitobin.toString());
		return unpackedRitobin.toFile().setExecutable(true);
	}

	/**
	 * Deletes Ritobin again
	 * 
	 * @throws IOException
	 */
//	static void removeRitobin() throws IOException {
//		deleteHashes();
//		if (unpackedRitobin != null) {
//			Files.deleteIfExists(unpackedRitobin);
//			Logger.getInstance().log("Ritobin deleted");
//		}
//	}

	/**
	 * Unpacks the hashes into the tmp folder
	 * 
	 * @throws IOException
	 */
	private static void unpackHashes() throws IOException {
		String tempDir = System.getProperty(TMPDIR);
		unpackedFolder = Paths.get(tempDir, "hashes");

		Files.createDirectories(unpackedFolder);

		String[] hashFiles = { "hashes.binentries.txt", "hashes.binfields.txt", "hashes.binhashes.txt",
				"hashes.bintypes.txt", "hashes.game.txt", "hashes.lcu.txt" };
		for (String fileName : hashFiles) {
			try (InputStream in = AutomationMain.class.getResourceAsStream("/hashes/" + fileName)) {
				if (in == null) {
					Logger.getInstance().log("Resource not found: /hashes/" + fileName);
					throw new IOException("Resource not found: /hashes/" + fileName);
				}
				Path targetFile = unpackedFolder.resolve(fileName);
				Files.copy(in, targetFile, StandardCopyOption.REPLACE_EXISTING);
				Logger.getInstance().log("Hash unpacked to: " + targetFile);
			}
		}
	}

	/**
	 * Deletes all files in the folder and the folder itself
	 * 
	 * @param folder
	 * @throws IOException
	 */
//	private static void deleteHashes() throws IOException {
//		if (unpackedFolder != null) {
//			File[] files = unpackedFolder.toFile().listFiles();
//			if (files != null) {
//				for (File file : files) {
//					Path path = file.toPath();
//					Files.delete(path);
//					Logger.getInstance().log("Deleted Hash:" + path);
//				}
//			}
//			Files.delete(unpackedFolder);
//			Logger.getInstance().log("Deleted Folder:" + unpackedFolder);
//		}
//	}

	/**
	 * unpacks CDTBTranslator to the tmp folder
	 * 
	 * @return
	 * @throws IOException
	 */
	static boolean unpackCDTBTranslator() throws IOException {
		String tempDir = System.getProperty(TMPDIR);
		unpackedCDTBTranslator = Paths.get(tempDir, "extractedCDTBTranslator.exe");
		try (InputStream in = UnpackExe.class.getResourceAsStream("/CDTBTranslator.exe");
				OutputStream out = Files.newOutputStream(unpackedCDTBTranslator)) {
			if (in == null) {
				Logger.getInstance().log("Resource not found: /CDTBTanslator");
				throw new IOException("Resource not found: /CDTBTanslator");
			}
			byte[] buffer = new byte[1024];
			int bytesRead;
			while ((bytesRead = in.read(buffer)) != -1) {
				out.write(buffer, 0, bytesRead);
			}
		}
		Logger.getInstance().log("CDTBTranslator unpacked under:" + unpackedCDTBTranslator.toString());
		return unpackedCDTBTranslator.toFile().setExecutable(true);
	}

	/**
	 * Deletes the CDTBTranslator again
	 * 
	 * @throws IOException
	 */
//	static void removeCDTBTranslator() throws IOException {
//		if (unpackedCDTBTranslator != null) {
//			Files.deleteIfExists(unpackedCDTBTranslator);
//			Logger.getInstance().log("CDTBTranslator deleted");
//		}
//	}

	/**
	 * unpacks WadMake to the tmp folder
	 * 
	 * @return
	 * @throws IOException
	 */
	static boolean unpackWadMake() throws IOException {
		String tempDir = System.getProperty(TMPDIR);
		unpackedWadMake = Paths.get(tempDir, "wad-make.exe");
		try (InputStream in = UnpackExe.class.getResourceAsStream("/wad-make.exe");
				OutputStream out = Files.newOutputStream(unpackedWadMake)) {
			if (in == null) {
				Logger.getInstance().log("Resource not found: /wad-make");
				throw new IOException("Resource not found: /wad-make");
			}
			byte[] buffer = new byte[1024];
			int bytesRead;
			while ((bytesRead = in.read(buffer)) != -1) {
				out.write(buffer, 0, bytesRead);
			}
		}
		Logger.getInstance().log("wad-make unpacked under:" + unpackedWadMake.toString());
		return unpackedWadMake.toFile().setExecutable(true);
	}

	/**
	 * Deletes WadMake again
	 * 
	 * @throws IOException
	 */
//	static void removeWadMake() throws IOException {
//		if (unpackedWadMake != null) {
//			Files.deleteIfExists(unpackedWadMake);
//			Logger.getInstance().log("wad-make deleted");
//		}
//	}

	static Path getUnpackedRitobin() {
		return unpackedRitobin;
	}

	static Path getUnpackedCDTBTranslator() {
		return unpackedCDTBTranslator;
	}

	static Path getUnpackedWadMake() {
		return unpackedWadMake;
	}

	private UnpackExe() {
	}
}
