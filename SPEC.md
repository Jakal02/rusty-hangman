# Rusty Hangman

This is a hangman game written in rust.

### The rules of Hangman

The executioner thinks of a random word, and the man to be hung must guess that word. The man to be hung guesses individual letters. If he gets it right, that letter is revealed, or else it counts against him. He is allowed only a certain number of failures, after which he loses and is hung!

### List of Requirements for version 1.0

- The crate is called hangman.
- The program accepts one keyword argument, a number from 3-6 or is excluded to restrict the secret number to that many letters, or any number between 3 and 6 inclusive if not specified.
- The program begins with this welcome message
```
Welcome to Hangman!

The Executioner has selected a <#> letter long secret word.
Press any key to start guessing.
```
- The game screen the player sees will look like
```
You have <#> guesses left.

    H U _ _ R Y

    +---+
    |   |
    |   O
    |  /|\
    |  / 
    |
=========

Previous guesses: I, O, L, P, S

Enter your next guess here: 
```
- Only accept letters a-z, case insensitive, as guesses



### Technologies to use:
- Clap for Argument parser