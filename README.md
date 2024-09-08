# Texthops

This is just a school project I have seen by a friend, who implemented this in java. I wanted to implement it in rust to learn the language and see how much faster it would be.

Big Credits to: earomc, who helped me getting into lookup tables and parallelistation!
Check out his github, hes a nice rust wizard!

The game works like that:
You have a text. Player 1 would start at the first letter and checks the index in the alphabet. It then moves as many steps as the alphabetic index forward. Player 2 (and as many players as you want) do the same thing but start at the second letter (and so on). The player that needed the minimum moves to move to the end wins. 

Here a visual explaination (In german, just look at the colors):

<img width="203" alt="image" src="https://github.com/user-attachments/assets/72f68fa2-69cf-4359-b11c-736be3cd8a7e">

Here a example when running the code and setting the player count to 4500 (The text length must be 4501 then):

![image](https://github.com/user-attachments/assets/1ef3dfdf-8e83-46ea-9a45-566256f50038)


