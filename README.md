# AutomatedSkinSizeConverter

This program allows the user to change the size of league of legends skins in great numbers.

## Dependencies

[You need to download java.](https://www.oracle.com/java/technologies/downloads)

While my program doesn't directly need it I still recommend using [cslol]() to 
actually use the mods.

## How to set up the program

Step 1: run the .exe

Step 2: Click on Create Options File

Step 3: Open the Options.txt File the program created in the same folder where the program is located.

Step 4: Add behind Root Path: the Path to the folder, where everything should be created in, e.g. which you created in preparation.

Step 6: Add behind League Path the Path to the League of Legends Games Folder, where the LeagueClient.exe is located.

Step 7:Save the Options.txt file

Step 8: Now press Generate Folders to create the needed Folders in the root folder.

## How to use the program

Step 1: Add the champions you want to convert into the Options.txt file in the lines below the #, e.g.

`ahri`

Additionally you can limit the maximum number of skins being converted by putting a number separated by a space behind the champion name. E.g.

`ahri 61`

This will convert all skins until skin 61.

Step 2: Save the Options.txt file

Step 3: Add the size Options files into the 0PutOptionFilesHere folder.

Step 4: Click on Convert chosen champion and wait for the program to finish

If you check the Enable self unpack folder for the program to automatically get the files from league. If it is unchecked you have to move the unpacked files to the 0WADS folder with Obsidian yourself.

Step 5: Now just import the wad folders into cslol

## Size Options

The program allows the user to set a default size and also different sizes for each skin.

To do that you have to create a file named after the champion. E.g.

`ahri.txt`

Then the first line should contain the champion, followed by a colon, followed by the default size.\\
Int he lines below that you should put: The skin number, then a colon, the the size for that skin. The skin numbers should be sorted.  An Example:

```
Ahri: 4
3:8
12:7
13:6
14:6
```

This would set the default size to 4 and the size of skin 3 to 8, skin 12 to 7 and skin 13 and 14 to 6.

Note that the default size without a size options file is set to 5. So if you want all skins of a champion to be size 5 you do not need to create such a file.

## Further notes

When adding names to the Options.txt and as size Options files championnames should not contain a space.

So instead of Miss Fortune write missfortune

For DJ Sona and Elementalist Lux you have to create luxlegendary.txt and sonalegendary.txt in the 0PutOptionFilesHere folder.

They will be automatically done, when their skinnumber gets converted.

Or just look at the example optins file.
