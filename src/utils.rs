use colored::Colorize;
use log::info;

pub fn get_os_version() -> String {
    "Debian SID".to_string()
}

pub fn warning(unsafe_mode: bool) {
    let warning_message = if unsafe_mode {
        "
WARNING: UNSAFE mode is enabled. This mode will automatically execute
AI-generated Linux commands based on your input. Please exercise extreme
caution when using this tool, as incorrect or harmful commands may be
generated.

By using this application, you acknowledge the risks involved and assume full
responsibility for any consequences resulting from the execution of
AI-generated commands.
"
    } else {
        "
WARNING: This application uses AI to generate and execute Linux commands based
on your input. Please be cautious when using this tool, as incorrect or harmful
commands may be generated. Always review the generated command before executing
it.

By using this application, you acknowledge the risks involved and assume full
responsibility for any consequences resulting from the execution of
AI-generated commands.
"
    };

    println!("{}", warning_message.red());
}

pub fn log(log_file: &str) {
    info!("{}", log_file);
}
