#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

// This is the message displayed at the start
const START_CONST: &str = r#"Please enter commands ('h' for help, 'quit' to quit, etc.):
"#;

// This is the menu message displayed when help is needed
const HELP_CONST: &str = r#"-Help and status-
-Controls:
 - 'h' - open help text and current status, the current open task
 - 'n' or 's' - close current if one is open and create-new/start task
 - 'nc' - close current if one is open, new copy of a previous task
 - 'lunch' - close current if one is open and start lunch break
 - 'bio' - close current if one is open and start biobreak
 - 'd' - change current task description
 - 'dc' - copy the description from a previous task
 - 'a' - append description to current task description
 - 'show' - show all tasks
 - 'quit' - quit the application and save to CSV file
 "#;

// The main struct that will hold the tasks,
// the current max id, if there is an open task
// and the currently open task
pub struct Tasks {
    pub list: Vec<Task>,
    pub open_task: bool,
    pub current_max_id: usize,
    pub current_open_task: Option<Task>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks {
            list: Vec::<Task>::new(),
            open_task: false,
            current_max_id: 0usize,
            current_open_task: None,
        }
    }
}

impl Default for Tasks {
    fn default() -> Self {
        Self::new()
    }
}

// The task struct that will hold the
// id, start date time, ending date time,
// and the description string
#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub start: time::OffsetDateTime,
    pub end: time::OffsetDateTime,
    pub description: String,
}

impl Task {
    pub fn new_task_with_desc(
        id_to_assign: usize,
        description_to_assign: String,
        print_message: bool,
    ) -> Self {
        if print_message {
            println!(); // For spacing
            println!("Started a new task with description:");
            println!("{}", description_to_assign);
            println!(); // For spacing
        }
        Task {
            id: id_to_assign,
            start: time::OffsetDateTime::now_local().expect("local time"),
            end: time::OffsetDateTime::now_local().expect("local time"),
            description: description_to_assign,
        }
    }
    pub fn new_task_with_desc_and_time(
        id_to_assign: usize,
        description_to_assign: String,
        print_message: bool,
        date_time: time::OffsetDateTime,
    ) -> Self {
        let mut task_with_modified_date_time =
            Task::new_task_with_desc(id_to_assign, description_to_assign.clone(), false);
        task_with_modified_date_time.start = date_time;
        task_with_modified_date_time.end = date_time;
        if print_message {
            println!(); // For spacing
            println!("Started a new task with description:");
            println!("{}", description_to_assign);
            println!(); // For spacing
        }
        task_with_modified_date_time
    }
}

// Implementing fmt::Display to print tasks
// more easily
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date_time_format =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                .expect("Harcoded format string was converted to a 'time' crate format descriptor");
        write!(
            f,
            "{} {} {} {}",
            self.id,
            self.start
                .format(&date_time_format)
                .expect("Date time was able to be formatted to a string properly"),
            self.end
                .format(&date_time_format)
                .expect("Date time was able to be formatted to a string properly"),
            self.description
        )
    }
}

// Function used to clear the command line screen
fn clear_command_line<T: std::io::Write>(w: &mut T) -> crossterm::Result<()> {
    // The queue! macro queues one or more command(s)
    // for further execution. They must be flushed to
    // be executed. In the case of stdout each line is
    // flushed, but we will call w.flush() to control this.
    // 1) "Clear" clears the terminal screen buffer
    // 2) "MoveTo" moves the terminal cursor
    //    to the given position (column, row).
    //    (0,0) is the top left cell.
    crossterm::queue!(
        w,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All), // Clear all
        crossterm::cursor::MoveTo(0, 0)
    )?;
    w.flush()?;
    Ok(())
}

fn print_current_open_task(tasks: &Tasks) {
    if tasks.open_task {
        let current_task = &tasks.current_open_task;
        println!("The current open task is:");
        println!("{}", current_task.clone().unwrap());
    } else {
        println!("There is currently no open item");
    }
    println!(); // For spacing
}

fn print_all_tasks(tasks: &Tasks) {
    for task in &tasks.list {
        println!("{}", task);
    }
    println!(); // For spacing
}

fn close_if_there_is_one_open(tasks: &mut Tasks) {
    if tasks.open_task {
        let current_task = &tasks.current_open_task;
        println!("The current open item is");
        println!("{}", current_task.clone().unwrap());
        if let Some(ref mut open_task) = &mut tasks.current_open_task {
            open_task.end = time::OffsetDateTime::now_local().expect("local time");
            tasks.list.push(open_task.clone());
            tasks.open_task = false;
        }
        println!("The task was closed");
    } else {
        println!("There is currently no open item to close");
    }
    println!(); // For spacing
}

fn new_task_with_desc_and_add_to_list(tasks: &mut Tasks, description: String) {
    let task_to_add = Task::new_task_with_desc(
        tasks.current_max_id,
        description,
        false, // Don't print message
    );
    tasks.current_max_id += 1;
    tasks.list.push(task_to_add);
}

fn new_task_with_desc_as_current(tasks: &mut Tasks, description: String) {
    let task_to_add = Task::new_task_with_desc(
        tasks.current_max_id,
        description,
        true, // Print message to inform user
    );
    tasks.current_open_task = Some(task_to_add);
    tasks.current_max_id += 1;
    tasks.open_task = true;
}

fn new_task_with_desc_and_time_as_current(
    tasks: &mut Tasks,
    description: String,
    date_time: time::OffsetDateTime,
) {
    let task_to_add = Task::new_task_with_desc_and_time(
        tasks.current_max_id,
        description,
        true, // Print message to inform user
        date_time,
    );
    tasks.current_open_task = Some(task_to_add);
    tasks.current_max_id += 1;
    tasks.open_task = true;
}

// This function reads and validates
// and input value to make sure it is
// a usize and within the current list of tasks
fn read_and_validate_id(tasks: &Tasks) -> Option<usize> {
    loop {
        println!("Please write a valid id to copy the description from or 'q' exit: ");
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line input");

        let input = input.trim().to_string();
        match &input[..] {
            "q" => return None,
            input => {
                match input.trim().parse() {
                    Ok(num) => {
                        if num < tasks.list.len() {
                            return Some(num);
                        } else {
                            continue;
                        }
                    }
                    Err(_) => continue,
                };
            }
        }
    }
}

// This is the configuration
// structure read from the xml
#[derive(Debug, Deserialize)]
pub struct ConfigXml {
    pub output_file_path_and_file_name: String,
    pub date_time_format_to_append_in_output_file_name: String,
}

fn save_to_csv(file_path: &std::path::Path, tasks: &Tasks) -> crossterm::Result<()> {
    // Generating the output to the csv file:
    let mut wtr = csv::Writer::from_path(file_path)
        .expect("Error creating the output file or the writing stream");
    // When writing records without Serde, the header record is written just
    // like any other record.
    wtr.write_record(&["id", "start", "end", "description"])
        .expect("Error while trying to write headers");

    let date_time_format =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
            .expect("Harcoded format string was converted to a 'time' crate format descriptor");

    for each_task in &tasks.list {
        wtr.write_record(&[
            each_task.id.to_string(),
            each_task
                .start
                .format(&date_time_format)
                .expect("Date time was able to be formatted to a string properly"),
            each_task
                .end
                .format(&date_time_format)
                .expect("Date time was able to be formatted to a string properly"),
            each_task.description.clone(),
        ])
        .expect("Error while trying to write a record");
    }
    if tasks.open_task {
        if let Some(task) = &tasks.current_open_task {
            wtr.write_record(&[
                task.id.to_string(),
                task.start
                    .format(&date_time_format)
                    .expect("Date time was able to be formatted to a string properly"),
                task.end
                    .format(&date_time_format)
                    .expect("Date time was able to be formatted to a string properly"),
                task.description.clone(),
            ])
            .expect("Error while trying to write a record");
        }
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> crossterm::Result<()> {
    let mut w = std::io::stdout();
    let mut tasks = Tasks::new(); // Create 'tasks' struct

    let xml_path_string = r"./config.xml";
    let xml_path = std::path::Path::new(xml_path_string);
    let config_file = std::fs::File::open(xml_path)?;
    let configxml: ConfigXml = serde_xml_rs::from_reader(config_file).unwrap();

    let current_start_time = time::OffsetDateTime::now_local().expect("local time");

    let date_time_format_string_from_config_file_xml =
        configxml.date_time_format_to_append_in_output_file_name;
    let date_time_format_from_config_file = time::format_description::parse(
        &date_time_format_string_from_config_file_xml,
    )
    .expect("Format string in config file should have the format expected by the time crate (eg. '[year]-[month]-[day]_[hour]_[minute]_[second]'). This is to be converted to a 'time' crate format descriptor");

    let current_start_time = current_start_time
        .format(&date_time_format_from_config_file).expect("The date time is expected to be able to be formatted with the format described in the config file. It should be in the way the 'time' crate understands it, for example '[year]-[month]-[day]_[hour]_[minute]_[second]'");

    let mut config_file_path_string = configxml.output_file_path_and_file_name;
    config_file_path_string.push_str(&current_start_time);
    config_file_path_string.push_str(".csv");
    let file_path = std::path::Path::new(&config_file_path_string);

    // Creating initial start up task
    new_task_with_desc_and_add_to_list(&mut tasks, "Start up of time tracking cli".to_string());

    // The execute! macro that crossterm provides
    // allows to execute one or more commands.
    // Here we execute the EnterAlternateScreen
    // command that switches to alternate screen.
    // Later in the code we will use the
    // LeaveAlternateScreen command to leave
    // the entered alternate screen.
    crossterm::execute!(w, crossterm::terminal::EnterAlternateScreen)?;

    // Begin the main loop
    loop {
        save_to_csv(file_path, &tasks)?;

        println!("{}", START_CONST);

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line input");

        input = input.trim().to_string();

        clear_command_line(&mut w)?;

        match &input[..] {
            "h" => {
                // Print help
                println!("{}", HELP_CONST);
                // Print current file path
                println!(
                    "The current file path to save is {}",
                    config_file_path_string
                );
                println!(); // For spacing
                            // If there is an open item print it
                print_current_open_task(&tasks);
            }
            "n" | "s" => {
                // We get the date_time at the start
                // because it might take some time to write
                // the description
                let date_time = time::OffsetDateTime::now_local().expect("local time");
                // Close currently open task
                close_if_there_is_one_open(&mut tasks);
                println!("A new task will be created. Please write the description:");
                let mut description = String::new();
                std::io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read description input");
                new_task_with_desc_and_time_as_current(
                    &mut tasks,
                    description.trim().to_string(),
                    date_time,
                )
            }
            "nc" => {
                println!("This will create a new task with a previous description");
                println!(); // For spacing
                close_if_there_is_one_open(&mut tasks);
                print_all_tasks(&tasks);
                // Logic to select an existing task and
                // create a new task with that description
                if let Some(id) = read_and_validate_id(&tasks) {
                    let copy_of_desc = tasks.list[id].description.clone();
                    new_task_with_desc_as_current(&mut tasks, copy_of_desc);
                }
            }
            "lunch" => {
                close_if_there_is_one_open(&mut tasks);
                new_task_with_desc_as_current(&mut tasks, "Lunch".to_string())
            }
            "bio" => {
                close_if_there_is_one_open(&mut tasks);
                new_task_with_desc_as_current(&mut tasks, "Biobreak".to_string())
            }
            "d" => {
                // If there is an open item print it
                print_current_open_task(&tasks);
                // Logic to get new description
                if tasks.open_task {
                    println!("Write the new description: ");
                    let mut description = String::new();
                    std::io::stdin()
                        .read_line(&mut description)
                        .expect("Failed to read description input");
                    let description = description.trim().to_string();
                    let replace_task = match tasks.current_open_task {
                        Some(task) => {
                            let mut new_task = task.clone();
                            new_task.description = description;
                            Some(new_task)
                        }
                        None => None,
                    };
                    tasks.current_open_task = replace_task;
                }
            }
            "dc" => {
                println!("This will copy a previous description to a currently open task");
                print_all_tasks(&tasks);
                // If there is an open item print it
                print_current_open_task(&tasks);
                // Logic to select an existing task and
                // copy that description
                if tasks.open_task {
                    if let Some(id) = read_and_validate_id(&tasks) {
                        let copy_of_desc = tasks.list[id].description.clone();
                        let replace_task = match tasks.current_open_task {
                            Some(task) => {
                                let mut new_task = task.clone();
                                new_task.description = copy_of_desc;
                                Some(new_task)
                            }
                            None => None,
                        };
                        tasks.current_open_task = replace_task;
                    }
                }
            }
            "a" => {
                // If there is an open item print it
                print_current_open_task(&tasks);
                // If there is an open task append it
                if tasks.open_task {
                    println!("Write what you would like to append: ");
                    let mut append = String::new();
                    std::io::stdin()
                        .read_line(&mut append)
                        .expect("Failed to read appending input");
                    let append = append.trim_end().to_string();
                    let replace_task = match tasks.current_open_task {
                        Some(task) => {
                            let mut new_task = task.clone();
                            new_task.description.push_str(&append);
                            Some(new_task)
                        }
                        None => None,
                    };
                    tasks.current_open_task = replace_task;
                }
            }
            "show" => {
                print_all_tasks(&tasks);
                // If there is an open item print it
                print_current_open_task(&tasks);
            }
            "quit" => {
                close_if_there_is_one_open(&mut tasks);
                break;
            }
            _ => {} // Ingore all other inputs
        }
    } // End of Main Loop

    // After the main loop exits

    // The execute! macro allows to execute one or more
    // commands.
    // Here we use the execute! macro to
    // execute the 'LeaveAlternateScreen' command
    // to leave the alternative screen that was
    // created at the start.
    crossterm::execute!(w, crossterm::terminal::LeaveAlternateScreen)?;

    // Creating shut down task
    new_task_with_desc_and_add_to_list(&mut tasks, "Shut down of time tracking cli".to_string());

    save_to_csv(file_path, &tasks)?;
    Ok(())
}
