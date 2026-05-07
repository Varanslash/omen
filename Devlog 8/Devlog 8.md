hello everyone

sorry i didn't post in the last 5 days, i was doing some other stuff

anyway i started work on the spec, which is the main thing with this project :D

i'm on the syntax portion, designing block types and whatnot, as well as operators and stuff

once i finish with it, i'll move on to documenting the video system and the music system.

there are a few new additions i made with syntax while writing libomen, which mainly come from my troubles accessing specific memory. i added the % sign, which allows access to newly created registers i made.

these registers so far would be %stack (references the top of the stack), and %cursorX and %cursorY, which dictate where you would draw on the screen if you use PRINT. they go by pixel values, so %cursorX 1 is one pixel away from 0, not 1 tile away

i think this system works quite well, let me know whether there could be some improvements or whether new registers are needed :3

goodbye everyone
