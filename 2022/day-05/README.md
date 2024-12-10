# Day 5
The main problem with this puzzle was parsing the input so I decided to do that part in Python because it's easier to deal with text in it than in Rust and Python comes with a regex module. `parse.py` "flips" the cargo stacks so that each one is on it's own line and removes the text from the move instructions.
