# AutomatedSkinSizeConverter

This program allows the user to change the size of league of legends skins in great numbers.

## Dependencies

[You need to download java.](https://www.oracle.com/java/technologies/downloads)

While my program doesn't directly need it I still recommend using [cslol](https://github.com/LeagueToolkit/cslol-manager) to 
actually use the mods.

You can download the [latest release from the "Releases" section](https://github.com/PiCake24/AutomatedSkinSizeConverter/releases/tag/v1.2.0).

## How to set up the program

Step 1: run the .exe

Step 2: Click on Create Options File. This creates the File which the program needs to run. This file should always be in the same folder as the program itself.

Step 3: Open the Options.txt File the program created in the same folder where the program is located.

Step 4: Insert the path to the folder where the mods should be put under "root path" (you will have to create this folder yourself). E.g.:

`C:\Users\Yanni\Desktop\MassConverter`

Step 5: Insert the path to the folder where the League Of Legends.exe is located under "League path" (It should end something like Riot Games\League of Legends\Game). E.g.:

`C:\Riot Games\League of Legends\Game`

(Optional) Step 6:  Insert the path to the folder where the League Of Legends.exe is located under "CsLol path" (This is only needed if you want the program to import the mods directly into cslol). E.g.:

` D:\Programs verknuepfng\Programs\cslol-manager`

Step 7:Save the Options.txt file

(Optional) Step 8: Now press Generate Folders to create the needed Folders in the root folder.

You only need to do this, if you want to set the skins to a different size. The size without using these folders is set to 5.

## How to use the program

Step 1: Add the champions you want to convert into the Options.txt file in the lines below path lines, e.g.

`ahri`

(Optional) Additionally you can limit the maximum number of skins being converted by putting a number separated by a space behind the champion name. E.g.

`ahri 61`

This will convert all skins up until and including skin 61.

Step 2: Save the Options.txt file

Step 3: Add the size Options files into the 0PutSizeOptionFilesHere folder (see below how to create them).

(Optional) Step 4: In the Pogram check the "Enable self unpack" checkbox for the program to automatically get the files from league. If it is unchecked you have to move the unpacked files to the 0WADS folder with Obsidian yourself.

(Optional) Step 5: In the program check the "Import mods into cslol" checkbox for the program to add the mods as "giant <championname>" into cslol. To see the mods in cslol you have to restart cslol.

Step 6: Click on Convert Chosen Champion and wait for the program to finish.

Step 7: Now just import the wad folders into cslol, if you have not checked the "Import mods into cslol" checkbox yourself

## Size Options

The program allows the user to set a default size (A size that gets applied to all skins) and also a different size for each skin.

If you want to set skins to your own preferred sizes you have to create a file in the 0PutSizeOptionsHere folder named after the champion. E.g.

`ahri.txt`

Then the first line should contain the champion, followed by a colon, followed by the default size.

(Stop here if you want every skin to have the same size)

If you additionally want to tweak specific skin sizes, in the lines below that you should put: The skin number, then a colon, then the size for that skin.

(Repeat for any skin u want changed)

The skin numbers should be sorted.

An Example:

```
Ahri: 4
3:8
12:7
13:6
14:6
```

This would set the size of skin 3 to 8, skin 12 to 7 and skin 13 and 14 to 6 and all other skins to size 4.

Note that the default size without a size options file is set to 5. So if you want all skins of a champion to be size 5 you do not need to create such a file.

## Further notes

When adding names to the Options.txt or to the size options files the champion names should not contain a space.

So instead of `Miss Fortune` write `missfortune`

DJ Sona and Elementalist Lux will be done automatically, when their specific number is set to convert.

To set a different size for DJ Sona and Elementalist Lux you have to create luxlegendary.txt and sonalegendary.txt in the 0PutSIzeOptionFilesHere folder.

sonalegendary should look like this:
```
Sona: 5
1: 4.25
2: 4.5
3: 4.75
```

luxlegendary should look like this
```
lux: 5
magma: 7
dark: 7
mystic: 7
ice: 7
storm: 7
light: 3
```
