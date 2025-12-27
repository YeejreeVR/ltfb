use std::fs::DirEntry;
use std::io::Write;
use std::path::Path;
use std::sync::{LazyLock,Mutex};
static HIDDEN:LazyLock<Mutex<bool>> = LazyLock::new(|| {Mutex::from(false)});
static SELECTED_DIR:std::sync::LazyLock<std::sync::Mutex<String>> = std::sync::LazyLock::new(|| {std::sync::Mutex::from(String::from(""))});
fn main() {
    let args = std::env::args();
    let mut hidden = HIDDEN.lock().unwrap();
    *hidden = args.collect::<Vec<String>>().contains(&"-hidden".to_string());
    drop(hidden);
    let mut current_directory = std::env::home_dir().unwrap().to_str().unwrap().to_string();
    std::env::set_current_dir(&current_directory).unwrap();
    loop {
        clear();
        let hidden = HIDDEN.lock().unwrap();
        let con = get_files(&current_directory, *hidden);
        drop(hidden);
        println!("{}", current_directory);
        show_dir_contents(&con);
        println!();
        current_directory = read(con);
        std::env::set_current_dir(&current_directory).unwrap()
    }
}
fn get_files(directory:&String,hidden:bool) -> Vec<DirEntry>{
    let mut return_value: Vec<DirEntry> = Vec::new();
    let files_result = std::fs::read_dir(directory);
 
    for i in files_result.unwrap() {
    let i_dir = i.unwrap();
    if !hidden {
        if !(i_dir.file_name().into_string().unwrap()[0..1] == String::from(".")) {
            return_value.push(i_dir);
        }
    }
    else {
        return_value.push(i_dir);
    }
        
    }
    return_value
}
    
    
fn has_permission(directory:&String) ->bool{
    std::fs::read_dir(directory).is_ok()
}
fn clear() {
    std::process::Command::new("clear").status().unwrap();
}
fn read_to_string(msg:&str) -> String {
    let mut return_value:String = String::new();
    print!("{}",msg);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut return_value).unwrap();
    return_value.trim().to_string()
}
fn read(contents:Vec<DirEntry>) -> String {
    let mut return_value = std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let read_string = read_to_string("Command>>>   ");
    let command:Vec<&str> = read_string.trim().split(" ").collect();
    let current_dir = std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let file_path:std::path::PathBuf;
    if command.get(1).is_some() {
        if command[1].parse::<usize>().is_ok() {
            file_path = contents.get(command[1].parse::<usize>().unwrap()).unwrap().path();
        }
        else if Path::new(&(current_dir.to_string()+"/"+command[1])).exists() {
            file_path = Path::new(&(current_dir.to_string()+"/"+command[1])).to_path_buf();
        }
        else {
            file_path = Path::new(&current_dir).to_path_buf();
        }
    }
    else {
        file_path = Path::new(&current_dir).to_path_buf();
    }
    if command[0] == String::from("cd") {
        if command.get(1).is_some() {
            if !command[1].parse::<usize>().is_err() {
            let hello = &contents.get(command[1].parse::<usize>().unwrap());
            if hello.is_some(){
                let dir_string = hello.unwrap().path().as_os_str().to_str().unwrap().to_string();
                if has_permission(&dir_string) {
                    return_value = hello.unwrap().path().as_os_str().to_str().unwrap().to_string();
                }
                else {
                    println!("You Dont Have Permission To Enter This Directory");
                    read_to_string("Press Enter To Continue");
                }
            }
            else {
                println!("Not A Directory");
                read_to_string("Press Enter To Continue");
                }
            }
            else if command[1] == "-" {
                return_value = std::env::current_dir().unwrap().parent().unwrap().to_str().unwrap().to_string();
            }
            else if Path::new(&(format!("{}/{}",&current_dir.as_str(), command[1]))).exists() {
                return_value = Path::new(&(format!("{}/{}",&current_dir.as_str(), command[1]))).to_str().unwrap().to_string();
            }
            else if Path::new(command[1]).exists() {
                return_value = Path::new(command[1]).to_str().unwrap().to_string();
            }
        }
        else {
            return_value = std::env::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
        }
    }
    else if command[0] == String::from("exit") {
        std::process::exit(0);
    }
    else if command[0] == String::from("delete") {
        let check = !(read_to_string(format!("You are trying to delete {} (Y/n)>  ",file_path.as_os_str().display()).as_str()) == "n");
        if file_path.is_dir() && check{
            std::fs::remove_dir_all(file_path.as_os_str().to_str().unwrap()).unwrap();
        }
        if file_path.is_file() && check{
            std::fs::remove_file(file_path.as_os_str().to_str().unwrap()).unwrap();
        }

    }

    else if command[0] == String::from("command") {
        let mut term_command = command;
        term_command.remove(0);
        let mut formated_term_command:Vec<String> = Vec::new();
        for item in 0..term_command.len() {
            if term_command[item].contains("{") && term_command[item].contains("}") {
                let item_split1 = term_command[item].split("{").collect::<Vec<_>>();
                let item_split2 = item_split1.get(1).unwrap().split("}").collect::<Vec<_>>();
                let formated_string = format!("{}{}{}", item_split1[0],contents.get(item_split2.get(0).unwrap().parse::<usize>().unwrap()).unwrap().path().to_str().unwrap(),item_split2[1]);
                formated_term_command.push(formated_string);
            }
            else {
                formated_term_command.push(term_command[item].to_string());
            }
        }
        std::thread::spawn(|| {std::process::Command::new("kitty").arg("-e").args(formated_term_command).output().unwrap();});

    }

    else if command[0] == String::from("select") || command[0] == String::from("slc"){
        let mut select = SELECTED_DIR.lock().unwrap();
        *select = file_path.as_os_str().to_str().unwrap().to_string();
        }
    else if command[0] == String::from("paste") {
        let select = SELECTED_DIR.lock().unwrap();
        std::process::Command::new("cp").arg("-r").arg(select.clone()).arg(current_dir).output().unwrap();
    }
    else if command[0] == String::from("move") {
        let mut select = SELECTED_DIR.lock().unwrap();
        std::process::Command::new("mv").arg(select.clone()).arg(current_dir).output().unwrap();
        *select = String::from("")
    }
    else if command[0] == String::from("open") {
        std::thread::spawn(|| {std::process::Command::new("kitty").arg("--directory").arg(current_dir).output().unwrap()});
    }

    else if command[0] == String::from("execute") {
        std::process::Command::new(format!("./{}", file_path.as_os_str().to_str().unwrap())).status().unwrap();
    }
    else if command[0] == String::from("rename") {
        let new_file_name = read_to_string("New File Name>>>     ");
        if new_file_name.len() != 0 {
            std::fs::rename(file_path, new_file_name).unwrap();
        }  
    }
    else if command[0] == String::from("help") {
        println!("cd - Changes Directory\ndelete - Deletes a file\nselect - Selects file for other operations\npaste - Pastes Selected File to Open Path\nopen - Opens Location in new terminal\nzip/unzip - Zips or Unzips zip files");
        read_to_string("Press Enter To Continue:");
    }

    else if command[0] == String::from("zip") {
        std::process::Command::new("zip").arg("-r").arg(format!("{}.zip",file_path.as_os_str().to_str().unwrap())).arg(file_path.file_name().unwrap().to_str().unwrap()).current_dir(current_dir).status().unwrap();
    }
    else if command[0] == String::from("unzip") {
        let new_unzip_folder;
        if command.get(2).is_some() {
            if command[2] == String::from("-n") {
               new_unzip_folder = file_path.as_os_str().to_str().unwrap().trim_end_matches(".zip");
               std::fs::create_dir(new_unzip_folder).unwrap();
            }
            else {
                new_unzip_folder = &current_dir
            }
        }
        else {
            new_unzip_folder = &current_dir
        }
        zip::ZipArchive::new(std::fs::File::open(file_path.clone()).unwrap()).unwrap().extract(new_unzip_folder).unwrap();
    }
    else if command[0] == String::from("hidden") {
        let mut hid = HIDDEN.lock().unwrap();
        *hid = !*hid;
        
    }
    else {
        let mut formated_term_command:Vec<String> = Vec::new();
        for item in 0..command.len() {
            if command[item].contains("{") && command[item].contains("}") {
                let item_split1 = command[item].split("{").collect::<Vec<_>>();
                let item_split2 = item_split1.get(1).unwrap().split("}").collect::<Vec<_>>();
                let formated_string = format!("{}{}{}", item_split1[0],contents.get(item_split2.get(0).unwrap().parse::<usize>().unwrap()).unwrap().path().to_str().unwrap(),item_split2[1]);
                formated_term_command.push(formated_string);
            }
            else {
                formated_term_command.push(command[item].to_string());
            }
        }
        let program = formated_term_command.drain(0..1).collect::<Vec<_>>().first().unwrap().to_string();
        std::thread::spawn(|| {std::process::Command::new(program).args(formated_term_command).output().unwrap();});
    }
    return_value
}
fn show_dir_contents(contents:&Vec<DirEntry>) {
    for i in contents.iter().enumerate() {
        let p = i.1.path();
        if p.is_dir() {
            print!("🗀 {}. {}   ",p.file_name().unwrap().display(),&i.0)
        }
        else if p.is_file() {
            print!("|| {}. {}     ", p.file_name().unwrap().display(),&i.0)
        }
    }
    std::io::stdout().flush().unwrap();
}
