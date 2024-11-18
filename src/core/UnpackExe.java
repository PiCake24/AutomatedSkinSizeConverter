package core;

import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class UnpackExe {
	private static Path unpackedRitobin;
	private static Path unpackedCDTBTranslator;

	/**
	 * unpacks Ritobin to the tmp folder
	 * 
	 * @return
	 * @throws IOException
	 */
	static boolean unpackRitobin() throws IOException {
		String tempDir = System.getProperty("java.io.tmpdir");
		unpackedRitobin = Paths.get(tempDir, "extractedRitobin.exe");
		try (InputStream in = new FileInputStream("/resources/ritobin_cli.exe");
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
		if (unpackedRitobin != null) {
			Files.deleteIfExists(unpackedRitobin);
		}
	}

	/**
	 * unpacks CDTBTranslator to the tmp folder
	 * 
	 * @return
	 * @throws IOException
	 */
	static boolean unpackCDTBTranslator() throws IOException {
		String tempDir = System.getProperty("java.io.tmpdir");
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
