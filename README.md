# Prompt2Shell

Prompt2Shell is an intreractive Rust program that request the OpenAI API to generate Linux commands based on user-given instructions.

## Warning

This program can directly executes commands obtained from the OpenAI API on your machine. Be cautious when using this program, as some commands might potentially harm your system or compromise its security.

Use this program at your own risk. The developers are not responsible for any damage caused by using this program.

## Setup

1. Install Rust and Cargo (https://www.rust-lang.org/tools/install).
2. Clone this repository: `git clone https://github.com/prompt2shell/prompt2shell.git`.
3. Navigate to the project folder: `cd prompt2shell`.
4. Install dependencies: `cargo build`.
5. Export your OpenAI API key: `export OPENAI_API_KEY=<your_api_key>`.

## Usage

Run the program using the command `cargo run`. You will see a `[prompt2shell]>` prompt where you can type your instructions. The program will send these instructions to the OpenAI API and generate a Linux command that you will be able to modify and/or execute on your machine.

## User Interaction

When the AI-generated command is displayed, you have the following options:

Type 'y' or 'yes' to execute the command.
Type 'n' or 'no' to reject the command.
Type 'e' or 'edit' to modify the command before executing it.
To exit the program, simply type quit.

## Examples
```
[prompt2shell]> Find all files larger than 50MB and compress them into a tar.gz archive named "large_files.tar.gz".
$ find . -type f -size +50M | tar -czvf large_files.tar.gz -T -
```

```
[prompt2shell]> Monitor real-time network traffic and display a summary report every 5 seconds.
$ watch -n 5 iftop -t
```

```
[prompt2shell]> List all running processes and sort them by the percentage of CPU usage in descending order.
$ ps aux --sort=-%cpu
```

```
[prompt2shell]> Remove all files and directories in the current folder that are older than 30 days and log the deleted items to "deleted_files.log".
$ find . -mtime +30 -type f -exec rm -v {} \; > deleted_files.log 2>&1
```

```
[prompt2shell]> Search for occurrences of the word "error" in all log files within the "/var/log" directory and display the top 10 most common error messages.
$ grep -r -i "error" /var/log | awk -F: '{print $2}' | sort | uniq -c | sort -rn | head -10
```

```
[prompt2shell]> Compile and install the latest source kernel.
$ sudo apt install -y build-essential libncurses5-dev libssl-dev libelf-dev bison flex git
  git clone https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
  cd linux
  make -j$(nproc) menuconfig
  make -j$(nproc)
  make M="drivers/crypto"
  sudo make modules_install
  sudo make install
  sudo update-initramfs -c -k $(make kernelversion)
  sudo update-grub
```

```
[prompt2shell]> load vmlinuz-6.3.0-rc4+ kernel without rebooting
$ sudo kexec -l /boot/vmlinuz-6.3.0-rc4+ --initrd=/boot/initrd.img-6.3.0-rc4+ --reuse-cmdline && sudo kexec -e
```

```
[prompt2shell]> show kernel version
uname -r
```
