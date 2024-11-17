package test;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.Writer;
import java.util.ArrayList;
import java.util.List;

public class TestingWithThreads {
	static String rootPath = "D:\\wad";
	static String champion = "ahri";
	static int SkinNumber = 85;
	static int scale = 5;

	public static void main(String[] args) {
//		arraytest2();
		normaltest();
		arraytest();

	}

	static void normaltest() {
		long start = System.currentTimeMillis();
		System.out.println("Start without multithreading");
		File file = new File(
				rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + SkinNumber + ".py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			// Gui.updateLog("Writing into " + champion + " " + SkinNumber + " with size " +
			// scale);

			String a = "";
			String b;
			while ((b = in.readLine()) != null) {
				a += b;
				a += "\n";
				if (b.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
					break;
				}
			}

			long finish = System.currentTimeMillis();
			long timeElapsed = finish - start;
			System.out.println("Found the size: " + timeElapsed);

			a += in.readLine();
			a += "\n";
			a += in.readLine();
			a += "\n";
			a += in.readLine();
			a += "\n";

			a += "\t \t \tskinScale: f32 = " + scale + "\n";
			while ((b = in.readLine()) != null) {
				if (b.contains("skinScale")) {
					continue;
				}
				a += b;
				a += "\n";
			}
//			Gui.updateLog(champion + " " + SkinNumber + " overwritten");
			finish = System.currentTimeMillis();
			timeElapsed = finish - start;
			System.out.println("Before writing: " + timeElapsed);
//			Writer w = new BufferedWriter(new FileWriter(file));
//			w.write(a);
//			w.close();
			System.out.println(a);
		} catch (Exception e) {
//			Gui.updateLog(e.getLocalizedMessage());
		}
		long finish = System.currentTimeMillis();
		long timeElapsed = finish - start;
		System.out.println("Done " + timeElapsed);
	}

	static void arraytest() {
		long start = System.currentTimeMillis();
		System.out.println("Start without multithreading, with putting everything into an array first");
		File file = new File(
				rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + SkinNumber + ".py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			// Gui.updateLog("Writing into " + champion + " " + SkinNumber + " with size " +
			// scale);

			String a = "";
			List<String> list = new ArrayList<String>();
			String b;
			while ((b = in.readLine()) != null) {
				list.add(b);
				list.add("\n");
				if (b.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
					break;
				}
			}

			long finish = System.currentTimeMillis();
			long timeElapsed = finish - start;
			System.out.println("Found the size: " + timeElapsed);

			list.add(b);
			list.add("\n");
			list.add(b);
			list.add("\n");
			list.add(b);
			list.add("\n");

			list.add("\t \t \tskinScale: f32 = " + scale + "\n");
			while ((b = in.readLine()) != null) {
				if (b.contains("skinScale")) {
					continue;
				}
				list.add(b);
				list.add("\n");
			}
			finish = System.currentTimeMillis();
			timeElapsed = finish - start;
			System.out.println("Before creating String: " + timeElapsed);
			a = String.join("", list);
//			Gui.updateLog(champion + " " + SkinNumber + " overwritten");
			finish = System.currentTimeMillis();
			timeElapsed = finish - start;
			System.out.println("Before writing: " + timeElapsed);
			Writer w = new BufferedWriter(new FileWriter(file));
			w.write(a);
			w.close();
		} catch (Exception e) {
//			Gui.updateLog(e.getLocalizedMessage());
		}
		long finish = System.currentTimeMillis();
		long timeElapsed = finish - start;
		System.out.println("Done " + timeElapsed);
	}

	static void arraytest2() {
		long start = System.currentTimeMillis();
		System.out.println("Start without multithreading, with putting everything into an array first 2");
		File file = new File(
				rootPath + "\\0WADS\\data\\characters\\" + champion + "\\skins\\skin" + SkinNumber + ".py");
		try (BufferedReader in = new BufferedReader(new FileReader(file))) {
			// Gui.updateLog("Writing into " + champion + " " + SkinNumber + " with size " +
			// scale);

			String a = "";
			List<String> list = new ArrayList<String>();
			String b;
			while ((b = in.readLine()) != null) {
				list.add(b);
				if (b.trim().equals("skinMeshProperties: embed = SkinMeshDataProperties {")) {
					break;
				}
			}

			long finish = System.currentTimeMillis();
			long timeElapsed = finish - start;
			System.out.println("Found the size: " + timeElapsed);

			list.add(b);
			list.add(b);
			list.add(b);

			list.add("\t \t \tskinScale: f32 = " + scale);
			while ((b = in.readLine()) != null) {
				if (b.contains("skinScale")) {
					continue;
				}
				list.add(b);
			}
			finish = System.currentTimeMillis();
			timeElapsed = finish - start;
			System.out.println("Before creating String: " + timeElapsed);
			a = String.join("\n", list);
//			Gui.updateLog(champion + " " + SkinNumber + " overwritten");
			finish = System.currentTimeMillis();
			timeElapsed = finish - start;
			System.out.println("Before writing: " + timeElapsed);
			Writer w = new BufferedWriter(new FileWriter(file));
			w.write(a);
			w.close();
		} catch (Exception e) {
//			Gui.updateLog(e.getLocalizedMessage());
		}
		long finish = System.currentTimeMillis();
		long timeElapsed = finish - start;
		System.out.println("Done " + timeElapsed);
	}
}
