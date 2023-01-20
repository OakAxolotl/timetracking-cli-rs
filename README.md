# timetracking-cli-rs
A time tracking CLI application using the Rust programming language

## Rust crates used
 * **crossterm** for manipulating the command line (entering an alternative screen, leaving an alternative screen, clearing lines and moving the cursor)
 * **time** for getting the current (now) local date time and formatting the date time
 * **serde**, **serde-xml-rs** and **serde_derive** for reading a xml config file
 * **csv** for saving the tasks to a csv file

## How to use
You will need to have the Rust compiler installed: https://www.rust-lang.org/tools/install 

Just clone the repository, navigate to the directory and use
```
cargo run
```
The command line should shift to an alternative screen and from there you can input the following commands:

| Command            | Description                                                                                                                                    |
|--------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| ```h```            | Open the **h**elp text and see the current status. You can see the current open task and the CSV file path that will be used to save the tasks |
| ```n``` or ```s``` | Close the current task if one is open, and create **n**ew task (ie. **s**tart a task)                                                          |
| ```nc```           | Close the current task if one is open, and create a **n**ew **c**opy of a previous task                                                        |
| ```lunch```        | Close the current task if one is open, and start **lunch** break                                                                               |
| ```bio```          | Close the current task if one is open, and start **bio**break                                                                                  |
| ```d```            | Change the **d**escription of the current open task                                                                                            |
| ```dc```           | **C**opy the **d**escription from a previous task into the current open task                                                                   |
| ```a```            | **A**ppend a string to the description of the current open task                                                                                |
| ```show```         | **Show** all tasks                                                                                                                             |
| ```quit```         | **Quit** the application and save all the tasks to a CSV file                                                                                  |

## License
Licensed under either
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
