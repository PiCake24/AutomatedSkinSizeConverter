package core;

import java.io.File;
import java.io.IOException;
import java.nio.file.FileVisitResult;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.SimpleFileVisitor;
import java.nio.file.StandardCopyOption;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.Callable;
import java.util.concurrent.CompletionService;
import java.util.concurrent.ExecutorCompletionService;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

public class UnpackWAD {

	// use console and python to unpack the files and move them
	static void unpackWAD(Map<String, Integer> map, String pythonPath, String leaguePath, String rootPath)
			throws InterruptedException {
		unpackAllWAD(map, pythonPath, leaguePath);

		moveFolders(map, leaguePath, rootPath);
		Thread.sleep(5000);
		Gui.updateLog("All champions moved");
	}

	private static void unpackAllWAD(Map<String, Integer> map, final String pythonPath, final String leaguePath)
			throws InterruptedException {
		Set<String> set = map.keySet();
		ExecutorService executor = Executors.newFixedThreadPool(4);
		CompletionService<Integer> completionService = new ExecutorCompletionService<>(executor);

		for (final String champion : set) {
			Gui.updateLog("Submitting unpack task for " + champion);
			completionService.submit(new Callable<Integer>() {
				@Override
				public Integer call() throws Exception {
					ProcessBuilder builder = new ProcessBuilder();
					builder.directory(new File(pythonPath));
					Process process = builder.command("cmd.exe", "/c",
							"cdtb wad-extract \"" + leaguePath + "\\" + champion + ".wad.client\"").start();
					int exitCode = process.waitFor();
					Gui.updateLog("Unpacking task completed for " + champion);
					return exitCode;
				}
			});
		}
		executor.shutdown();
		executor.awaitTermination(Long.MAX_VALUE, TimeUnit.NANOSECONDS);

		Gui.updateLog("All unpack tasks completed.");
	}

	private static void moveFolders(Map<String, Integer> map, String leaguePath, String rootPath) {
		Set<String> set = map.keySet();
		for (String champion : set) {
			Gui.updateLog("Moving " + champion);

			final Path deletePath = Paths.get(leaguePath, champion + ".wad");
			final Path sourcePath = Paths.get(leaguePath, champion + ".wad", "data", "characters", champion, "skins");
			final Path targetPath = Paths.get(rootPath, "0WADS", "data", "characters", champion, "skins");
			System.out.println(rootPath + "0WADS\\data\\characters\\" + champion + "\\skins");

			try {
				if (Files.notExists(sourcePath)) {
					Gui.updateLog("Source path does not exist: " + sourcePath);
					continue; // Skip this champion if the source path doesn't exist
				}

				Files.createDirectories(targetPath);
				Files.walkFileTree(sourcePath, new SimpleFileVisitor<Path>() {
					@Override
					public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) throws IOException {
						Path targetFile = targetPath.resolve(sourcePath.relativize(file));
						Files.copy(file, targetFile, StandardCopyOption.REPLACE_EXISTING);
						Files.delete(file);
						return FileVisitResult.CONTINUE;
					}

					@Override
					public FileVisitResult postVisitDirectory(Path dir, IOException exc) throws IOException {
						Files.delete(dir);
						return FileVisitResult.CONTINUE;
					}
				});

				Gui.updateLog("Removing not needed folders");
				if (Files.notExists(deletePath)) {
					Gui.updateLog("Delete path does not exist: " + deletePath);
					continue; // Skip if the delete path doesn't exist
				}

				Files.walkFileTree(deletePath, new SimpleFileVisitor<Path>() {
					@Override
					public FileVisitResult visitFile(Path file, BasicFileAttributes attrs) throws IOException {
						Files.delete(file);
						return FileVisitResult.CONTINUE;
					}

					@Override
					public FileVisitResult postVisitDirectory(Path dir, IOException exc) throws IOException {
						if (exc == null) {
							Files.delete(dir);
							return FileVisitResult.CONTINUE;
						} else {
							throw exc;
						}
					}
				});
				Gui.updateLog(champion + " move completed");
			} catch (IOException e) {
				e.printStackTrace();
			}
		}
	}
}
