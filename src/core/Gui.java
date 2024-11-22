package core;

import java.awt.Desktop;
import java.awt.Dimension;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;

import javax.swing.BoxLayout;
import javax.swing.JButton;
import javax.swing.JCheckBox;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JOptionPane;
import javax.swing.JPanel;
import javax.swing.JScrollPane;
import javax.swing.ScrollPaneConstants;
import javax.swing.WindowConstants;

public class Gui {
	static JPanel logPanel;
	static JScrollPane log;
	static JButton createOptions;
	static JButton convertChosenSkins;
	static JButton createFolders;
	static JLabel intLabel;
	static JCheckBox selfUnpackCheckBox;

	static CreateOptionsFile createO = new CreateOptionsFile();
	static BackgroundConverting bgConv = new BackgroundConverting();
	static CreateFolders createF = new CreateFolders();

	/**
	 * Generates the Gui
	 */
	public static void generateGui(final String CURRENT_VERSION) {
		JFrame mainFrame = new JFrame();
		JPanel mainPanel = new JPanel();
		mainPanel.setLayout(new BoxLayout(mainPanel, BoxLayout.Y_AXIS));
		JPanel p0 = new JPanel();
		JPanel p1 = new JPanel();
		p1.setLayout(new BoxLayout(p1, BoxLayout.Y_AXIS));
		JPanel p2 = new JPanel();
		JPanel p3 = new JPanel();
		JPanel p4 = new JPanel();

		intLabel = new JLabel();

		createOptions = new JButton("Create Options File");
		createOptions.addActionListener(new ActionListener() {

			@Override
			public void actionPerformed(ActionEvent e) {
				createO = new CreateOptionsFile();
				createOptions.setEnabled(false);
				convertChosenSkins.setEnabled(false);
				createFolders.setEnabled(false);
				createO.execute();

			}
		});

		createFolders = new JButton("Create Folders");
		createFolders.addActionListener(new ActionListener() {

			@Override
			public void actionPerformed(ActionEvent e) {
				createF = new CreateFolders();
				createOptions.setEnabled(false);
				convertChosenSkins.setEnabled(false);
				createFolders.setEnabled(false);
				createF.execute();
			}
		});
		convertChosenSkins = new JButton("Convert chosen champions");
		convertChosenSkins.addActionListener(new ActionListener() {

			@Override
			public void actionPerformed(ActionEvent e) {
				bgConv = new BackgroundConverting();
				createOptions.setEnabled(false);
				convertChosenSkins.setEnabled(false);
				createFolders.setEnabled(false);
				bgConv.execute();
			}
		});

		logPanel = new JPanel();
		logPanel.setLayout(new BoxLayout(logPanel, BoxLayout.Y_AXIS));

		log = new JScrollPane(logPanel);
		log.setVerticalScrollBarPolicy(ScrollPaneConstants.VERTICAL_SCROLLBAR_ALWAYS);
		log.setPreferredSize(new Dimension(400, 200));

		selfUnpackCheckBox = new JCheckBox("Enable self unpack");

		p2.add(createOptions);
		p2.add(createFolders);
		p2.add(convertChosenSkins);
		p3.add(log);
		p4.add(intLabel);
		p4.add(selfUnpackCheckBox);
		mainPanel.add(p0);
		mainPanel.add(p1);
		mainPanel.add(p2);
		mainPanel.add(p3);
		mainPanel.add(p4);
		mainFrame.add(mainPanel);
		mainFrame.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE);
		mainFrame.setSize(1000, 500);
		mainFrame.setResizable(false);
		mainFrame.setVisible(true);

		String latestVersion;
		try {
			latestVersion = VersionChecker.getLatestVersionFromGitHub();
			if (VersionChecker.isNewVersionAvailable(CURRENT_VERSION, latestVersion)) {
				createPopup(latestVersion);
			}
		} catch (IOException e1) {
			e1.printStackTrace();
		}

	}

	/**
	 * Creates a popup that redericts the user to github
	 * 
	 * @param latestVersion
	 */
	public static void createPopup(String latestVersion) {
		String message = "A new version (" + latestVersion + ") is available! Please update.";
		String title = "Update Available";
		int option = JOptionPane.showConfirmDialog(null, message, title, JOptionPane.DEFAULT_OPTION,
				JOptionPane.INFORMATION_MESSAGE);

		if (option == JOptionPane.OK_OPTION && Desktop.isDesktopSupported()
				&& Desktop.getDesktop().isSupported(Desktop.Action.BROWSE)) {
			try {
				Desktop.getDesktop().browse(new URI("https://github.com/PiCake24/AutomatedSkinSizeConverter/releases"));
			} catch (IOException | URISyntaxException e) {
				e.printStackTrace();
			}
		}
	}

	/**
	 * Gives the end user information, by showing the text in the logPanel
	 * 
	 * @param text
	 */
	public static void updateLog(String text) {
		if (logPanel != null) {
			logPanel.add(new JLabel(text), 0);
			logPanel.revalidate();
			logPanel.repaint();
		}
	}

	/**
	 * Reenables all buttons
	 */
	public static void enableButtons() {
		createFolders.setEnabled(true);
		convertChosenSkins.setEnabled(true);
		createOptions.setEnabled(true);
	}

	/**
	 * Returns if the selfUnpackCheckBox is checked
	 * 
	 * @return
	 */
	public static boolean getCheckBoxBool() {
		return selfUnpackCheckBox.getSelectedObjects() != null;
	}

	private Gui() {

	}
}
