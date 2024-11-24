# AutomatedSkinSizeConverter

This program allows the user to change the size of league of legends skins in great numbers.

## Dependencies

[You need to download java.](https://www.oracle.com/java/technologies/downloads)

While my program doesn't directly need it I still recommend using [cslol](https://github.com/LeagueToolkit/cslol-manager) to 
actually use the mods.

## How to set up the program

Step 1: run the .exe

Step 2: Click on Create Options File. This creates the File which the program needs to run. This file should always be in the same folder as the program itself.

Step 3: Open the Options.txt File the program created in the same folder where the program is located.

Step 4: Insert the path to the folder where the mods should be put under "root path" (you will have to create this folder yourself). E.g.:

`C:\Users\Yanni\Desktop\MassConverter`

Step 5: Insert the path to the folder where the League Of Legends.exe is located under "League path" (It should end something like Riot Games\League of Legends\Game). E.g.:

`C:\Riot Games\League of Legends\Game`

Step 6:Save the Options.txt file

(Optional) Step 7: Now press Generate Folders to create the needed Folders in the root folder.

You only need to do this, if you want to set the skins to a different size. The size without using these folders is set to 5.

## How to use the program

Step 1: Add the champions you want to convert into the Options.txt file in the lines below the #, e.g.

`ahri`

(Optional) Additionally you can limit the maximum number of skins being converted by putting a number separated by a space behind the champion name. E.g.

`ahri 61`

This will convert all skins up until and including skin 61.

Step 2: Save the Options.txt file

Step 3: Add the size Options files into the 0PutSizeOptionFilesHere folder (see below how to create them).

Step 4: Click on Convert Chosen Champion and wait for the program to finish.

Check the Enable self unpack folder for the program to automatically get the files from league. If it is unchecked you have to move the unpacked files to the 0WADS folder with Obsidian yourself.

Step 5: Now just import the wad folders into cslol

## Size Options

The program allows the user to set a default size (A size that gets applied to all skins) and also a different size for each skin.

If you want to set different sizes you have to create a file in the 0PutSizeOptionsHere folder named after the champion. E.g.

`ahri.txt`

Then the first line should contain the champion, followed by a colon, followed by the default size.

In the lines below that you should put: The skin number, then a colon, the the size for that skin. The skin numbers should be sorted. 

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

When adding names to the Options.txt and as size Options files championnames should not contain a space.

So instead of Miss Fortune write missfortune

For DJ Sona and Elementalist Lux you have to create luxlegendary.txt and sonalegendary.txt in the 0PutOptionFilesHere folder.

They will be automatically done, when their skinnumber gets converted.

Or just look at the example optins file.
