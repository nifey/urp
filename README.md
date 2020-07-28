# Boolean calculator engine using Unate Recursive Paradigm

This project is done as an assignment for the *VLSI CAD Part 1: Logic* course on Coursera,
offered by University of Illinois at Urbana-Champaign.

## Brief intro

Unate Recursive Paradigm is a way of manipulating boolean functions.
The boolean functions are represented in the Positional Cube Notation PCN format.

The functions in Sum of Product form can be represented as a Cubelist data structure.
Each cube in the cubelist corresponds to one Product term in the Sum of product form.
This data structure allows us to do operations on boolean functions by using
Shannon Cofactors, Recursion and properties of Unate functions.

## How to execute

To run the program, install Rust and then run
```bash
cargo run <command_file_name>
```
This will read the commands from the command file and execute them

If instead you want to run interactively then just run
```bash
cargo run
```

Some sample command files and .pcn files are present in sample_files folder. These
files were provided as a part of the Coursera Course mentioned above.

## Format of the input files
All the functions are named using numbers and the input functions should be
present as a file in PCN (Positional Cube Notation) format. The input file should be
named with a .pcn extension. Eg: function 3 should be present in PCN format in a file named 3.pcn

PCN format files are written as follows:
* The first line contains one number N, indicating the number of variables
* The second line contains one number M, indicating the number of cubes in the function
* The M following lines have the number of Non Dontcare terms in the cube followed by 
the numbers indicating the variables present in the cube. If it is a positive number,
then the variable indicated by the number is present as a positive literal in the cube.
If it is a negative number, then the variable indicated by the number is present as
a negative literal in the cube.

Eg: the function abc' + d + b'd' can be written as
```
4		// 4 variables are involved (a,b,c,d)
3		// 3 cubes or product terms
3 1 2 -3	// 3 variable, a=1 b=2 c'=-3
1 4		// 1 variable, d=4
2 -2 -4		// 2 variable, b'=-2 d'=-4
```

## Available commands
* r 3

reads the function 3 from 3.pcn
* p 3

writes the function 3 (in memory) to 3.pcn
* \+ 6 3 2

Performs Logical OR, 6 = 3 OR 2
* & 6 3 2

Performs Logical AND, 6 = 3 AND 2
* ! 6 3

Performs Logical Not, 6 = NOT 3
* t 6

Prints whether function 6 is a tautology
* q

quits the program
