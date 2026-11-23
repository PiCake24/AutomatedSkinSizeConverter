# AutomatedSkinSizeConverter

This program allows the user to change the size of league of legends skins in great numbers.

## Dependencies

While my program doesn't directly need it I still recommend using [cslol](https://github.com/LeagueToolkit/cslol-manager) or [ltk manager](https://github.com/LeagueToolkit/ltk-manager) to 
actually use the mods.

You can download the [latest release from the "Releases" section](https://github.com/PiCake24/AutomatedSkinSizeConverter/releases/latest).

## How to set up the program

Step 1: run the .exe

Step 2: If you do not have already an Options file in the same folder as  the program, it will create one and prompt you the needed values.

Step 3: Insert the path to the folder where the mods should be put under "Root Path" (you will have to create this folder yourself). E.g.:

`C:\Users\<user>\Desktop\MassConverter`

(Optional) Step 4: Insert the path to the folder where the League Of Legends.exe is located under "League Path" (It should end something like Riot Games\League of Legends\Game). E.g.:

`C:\Riot Games\League of Legends\Game`

(This is only needed if you want the program to unpack the files itself)

(Optional) Step 5:  Insert the path to the folder where the CsLolManager is located under "CsLol path" (This is only needed if you want the program to import the mods directly into cslol). E.g.:

` D:\Programs verknuepfng\Programs\cslol-manager`

(Optional) Step 6:  Insert the path to the folder where the Ltk Manager puts its mods under "Ltk Path" (This is only needed if you want the program to import the mods directly into ltk manager). E.g.:

`C:\mods`

Step 7: Click Confirm

## How to use the program

Step 1: Add the champions you want to convert into the 0Options.txt file, which you can find under <project>/0PutSizeOptionFilesHere/default, 

`ahri`

(Optional) Additionally you can limit the exactly which skins you want to convert being converted by putting a number separated by a space behind the champion name. E.g.

`ahri 7-61` This will convert all skins from 7 up until and including skin 61.

`ahri 1|23|90` This will convert skins 1, 23 and 90

`ahri 1-34|87` This will convert skins 1 to 34 and 87

(If you have used an older version of the program where you could only select the max skin the program will automatically convert these to the new format the first time you start it)

Step 2: Save the 0Options.txt file

Step 3: Add the size Options files into the 0PutSizeOptionFilesHere/default folder (see below how to create them).

(Optional) Step 4: In the program check the "Download Files" checkbox for the program to automatically get the files from league. If it is unchecked you have to move the unpacked files to the 0WADS folder with Obsidian yourself.

(Optional) Step 5: In the program check the "Export to cslol"/"Export to ltk" checkboxes for the program to add the mods as "giant <championname>" into cslol/ltk manager. If you want to use this option I recommend to close cslol/ltk manager while my program is running.

Step 6: Click "Start Conversion" and wait for the program to finish.

Step 7: Now just import the wad folders into cslol, if you have not checked the "Export to cslol"/"Export to ltk" checkbox yourself

## Size Options

The program allows the user to set a default size per set (currently only the default set exists) and a default size per champion and also a different size for each skin.

For a default size per set for all champions you add `Scale: <size>` into the 0Options.txt in the default set.

If you want to set skins to your own preferred sizes you have to create a file in the 0PutSizeOptionFilesHere/default folder named after the champion. E.g.

`ahri.txt`

Then the first line should contain the champion, followed by a colon, followed by the default size.

If you additionally want to tweak specific skin sizes, in the lines below that you should put: The skin number, then a colon, then the size for that skin.

(Repeat for any skin u want changed)

An Example:

```
Ahri: 4
3:8
12:7
13:6
14:6
```

This would set the size of skin 3 to 8, skin 12 to 7 and skin 13 and 14 to 6 and all other skins to size 4.

Note that the default size without a size options file and no scale in 0PutSizeOptionFilesHere/default/0Options.txt is set to 2. So if you want all skins of a champion to be size 2 you do not need to create such a file.

## Further notes

When adding names to the Options.txt or to the size options files the champion names should not contain a space.

So instead of `Miss Fortune` write `missfortune`

DJ Sona and Elementalist Lux will be done automatically, when their specific number is set to convert.

To set a different size for DJ Sona and Elementalist Lux you have to create luxlegendary.txt and sonalegendary.txt in the 0PutSIzeOptionFilesHere/default folder.

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
