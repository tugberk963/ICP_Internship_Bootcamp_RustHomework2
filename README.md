# ICP_Internship_Bootcamp_RustHomework2
ICP Internship Bootcamp Rust Homework2

Task Details

In this homework, you will create a word counter program in Rust.

Steps:

Create a struct named WordCounter.

In the WordCounter struct, have the following field:

text (text to count words from)

Implement the following functions for WordCounter:

new(text: &str) -> WordCounter: This function should take a text and create an instance of the WordCounter struct.

count_words() -> usize: This function should count the number of words in the text and return the result as an integer. A word is defined as a string separated by whitespace characters.

In the main function, prompt the user to enter a text.

In the count_words function, check if the text is empty. If it is empty, return an error message. 

 

Create an instance of WordCounter using the text entered by the user.

Call the count_words function on the created WordCounter instance.

Print the obtained word count to the screen.

Compile and run the program to test its functionality.

Checklist:

Define a struct named WordCounter.

In the WordCounter struct, define a field named text.

Implement the new function.

Implement the count_words function.

Prompt the user for text input.

Create a WordCounter instance using the input text.

Call the count_words function and print the word count to the screen.

Compile and run the program to test its functionality.
