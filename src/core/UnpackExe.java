package core;

import java.io.File;
import java.io.FileInputStream;
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
		try (InputStream in = new FileInputStream("./resources/ritobin_cli.exe");
				OutputStream out = Files.newOutputStream(unpackedRitobin)) {
			byte[] buffer = new byte[1024];
			int bytesRead;
			while ((bytesRead = in.read(buffer)) != -1) {
				out.write(buffer, 0, bytesRead);
			}
		}
		return unpackedRitobin.toFile().setExecutable(true);

	}

	/**
	 * Deletes Ritobin again
	 * 
	 * @throws IOException
	 */
	static void removeRitobin() throws IOException {
		deleteHashes();
		if (unpackedRitobin != null) {
			Files.deleteIfExists(unpackedRitobin);
		}
	}

	/**
	 * Unpacks the hashes into the tmp folder
	 * 
	 * @throws IOException
	 */
	private static void unpackHashes() throws IOException {
		String tempDir = System.getProperty(TMPDIR);
		unpackedFolder = Paths.get(tempDir, "hashes");
		File resourceFolder = new File("./resources/hashes");

		if (!resourceFolder.isDirectory()) {
			throw new IOException("Resource folder not found: " + resourceFolder.getAbsolutePath());
		}

		// Create the target folder in the temp directory
		Files.createDirectories(unpackedFolder);

		// Copy all files from the resource folder to the temp directory
		File[] files = resourceFolder.listFiles();
		if (files != null) {
			for (File file : files) {
				Path targetFile = unpackedFolder.resolve(file.getName());
				Files.copy(file.toPath(), targetFile, StandardCopyOption.REPLACE_EXISTING);
			}
		}
	}

	/**
	 * Deletes all files in the folder and the folder itself
	 * 
	 * @param folder
	 * @throws IOException
	 */
	private static void deleteHashes() throws IOException {
		if (unpackedFolder != null) {
			File[] files = unpackedFolder.toFile().listFiles();
			if (files != null) {
				for (File file : files) {
					Path path = file.toPath();
					Files.delete(path);
				}
			}
			Files.delete(unpackedFolder);
		}
	}

	/**
	 * unpacks CDTBTranslator to the tmp folder
	 * 
	 * @return
	 * @throws IOException
	 */
	static boolean unpackCDTBTranslator() throws IOException {
		String tempDir = System.getProperty(TMPDIR);
		unpackedCDTBTranslator = Paths.get(tempDir, "extractedCDTBTranslator.exe");
		try (InputStream in = new FileInputStream("resources/CDTBTranslator.exe");
				OutputStream out = Files.newOutputStream(unpackedCDTBTranslator)) {
			byte[] buffer = new byte[1024];
			int bytesRead;
			while ((bytesRead = in.read(buffer)) != -1) {
				out.write(buffer, 0, bytesRead);
			}
		}
		return unpackedCDTBTranslator.toFile().setExecutable(true);
	}

	/**
	 * Deletes the CDTBTranslator again
	 * 
	 * @throws IOException
	 */
	static void removeCDTBTranslator() throws IOException {
		if (unpackedCDTBTranslator != null) {
			Files.deleteIfExists(unpackedCDTBTranslator);
		}
	}

	static Path getUnpackedRitobin() {
		return unpackedRitobin;
	}

	static Path getUnpackedCDTBTranslator() {
		return unpackedCDTBTranslator;
	}

	private UnpackExe() {
	}
}
