# AoC_2024
Rust solutions to Advent of Code 2024

I have made som help functions to parse the input. I also made a program that creates a new main, a module for the solution model code and downloads my personalized input file. Feel free to use these if you like. Instructions below.

In the bin folder there is a file that can create some framework code for the current day. " create_today_bin.rs " It creates a file with a main i bin folder, Day1_solver.rs e.g, and a folder in src for that day, day1 e.g., which holds a module for that day, day1_model.rs e.g. , where code that you might want to reuse a later day can be written.

It will also download your personal input as input.txt and put it in the folder for the day. To do this you need to create a text file, named my_secret.txt, in the root folder of this project. The file shall contain only one Line with the session key.

To find your session cookie login to adventofcode.com. press F12 to the web debug tools in e.g. Chrome. Search for session and find a Cookie with an attribute like this session=53616c7465645f5f4da6ca1c837481b7810520f86bf56a825a7481b65c0521a4242fb9ba12987426db1a2f181c71dacd5af37648fbe5b2e7829f1dfcb735a6f2 (this session key is from last year so it is invalid now) Copy your key into your file in one Line.

the input.rs has two different parsers for your input one simple that can transform one Line at a time, And one that uses a trait that you implement in a struct. This implementation can have states for more complex paring.

Hope you find this useful.

/SweTailor94 aka Magnus
