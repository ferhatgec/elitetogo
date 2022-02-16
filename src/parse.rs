// MIT License
//
// Copyright (c) 2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_go {
    use elite::ast::EliteKeywords;

    fn replace(data: String) -> String {
        data.replace('\"', "\\'").replace("\0", "")
    }

    pub fn parse(data: elite::parser::EliteParser) -> String {
        let mut regenerated_code = String::from("\n\
package main\n\
\n\
import (\n\
    \"fmt\"\n\
    \"os\"\n\
    \"os/exec\"\n\
    \"runtime\"\n\
)\n\
\n\
var cpuarch, opsys = runtime.GOARCH, runtime.GOOS\n\
\n\
func out(command string) {\n\
    output, _ := exec.Command(\"/usr/bin/bash\", \"-c\", command).Output()\n\
    fmt.Printf(\"%s\", output)
}\n\
func exists(name string) bool {\n\
    _, err := os.Stat(name)\n\
    return !os.IsNotExist(err)\n\
}\n\
func main() {\n");
        let mut line = 0u32;
        let mut is_for = false;

        for x in data.ast_nodes.data {
            match x.__type {
                EliteKeywords::Set => {
                    regenerated_code.push_str(
                        format!("{newline}{name} := \"{data}\"\n{newline}_ = {name}\n",
                                newline = " ".repeat(line as usize),
                                name = x.__name, data = replace(x.__data)).as_str());
                }
                EliteKeywords::Print => {
                    regenerated_code.push_str(
                        format!("{}fmt.Print(\"{}\")\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Println => {
                    regenerated_code.push_str(
                        format!("{}fmt.Println(\"{}\")\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Use => {}
                EliteKeywords::RequiredVersion => {
                    regenerated_code.push_str(format!("if(\"{}\" != \"{}\"){{\n{}",
                                                            replace(x.__name),
                                                            replace(x.__data),
                                                            " fmt.Println(\"elite: Required higher version\")\
                                                            \n os.Exit(1)\n}\n").as_str());
                }
                EliteKeywords::Change => {}
                EliteKeywords::IfArg => {
                    regenerated_code.push_str(format!("{}if(\"{}\"", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::LeftParenthese => {}
                EliteKeywords::RightParenthese => {}
                EliteKeywords::LeftSqBracket => {
                    regenerated_code.push_str("{\n");
                    if is_for { is_for = false; continue; } line += 1;
                }
                EliteKeywords::RightSqBracket => {
                    regenerated_code.push_str("}\n");
                    if line < 1 { continue } line -= 1;
                }
                EliteKeywords::Eq => {
                    regenerated_code.push_str(format!(" == \"{}\")", replace(x.__data)).as_str());
                }
                EliteKeywords::UnEq => {
                    regenerated_code.push_str(format!(" != \"{}\")", replace(x.__data)).as_str());
                }
                EliteKeywords::Signal => {
                    if x.__name == "exit" {
                        regenerated_code.push_str(format!("{}os.Exit(1)\n", " ".repeat(line as usize)).as_str());
                    } else if x.__name == "start" {
                        is_for = true;
                    }
                }
                EliteKeywords::Exec => {
                    regenerated_code.push_str(format!("{}out(\"{}\")\n", " ".repeat(line as usize), replace(x.__name)).as_str());
                }
                EliteKeywords::AddSource => {}
                EliteKeywords::Append => {}
                EliteKeywords::Exit => {
                    regenerated_code.push_str(format!("{}os.Exit(1)\n", " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Specific => {
                    match x.__data.as_str() {
                        "x86" => regenerated_code.push_str(
                            format!("{}if(cpuarch == \"386\")", " ".repeat(line as usize)).as_str()),
                        "amd64" => regenerated_code.push_str(
                                format!("{}if(cpuarch == \"amd64\")", " ".repeat(line as usize)).as_str()),
                        "windows" => regenerated_code.push_str(
                            format!("{}if(opsys == \"windows\")", " ".repeat(line as usize)).as_str()),
                        "macos" => regenerated_code.push_str(
                            format!("{}if(opsys == \"darwin\")", " ".repeat(line as usize)).as_str()),
                        "linux" => regenerated_code.push_str(
                            format!("{}if(opsys == \"linux\")", " ".repeat(line as usize)).as_str()),
                        "freebsd" => regenerated_code.push_str(
                            format!("{}if(opsys == \"freebsd\")", " ".repeat(line as usize)).as_str()),
                        "netbsd" => regenerated_code.push_str(
                            format!("{}if(opsys == \"netbsd\")", " ".repeat(line as usize)).as_str()),
                        "android" => regenerated_code.push_str(
                            format!("{}if(opsys == \"android\")", " ".repeat(line as usize)).as_str()),
                        _ =>
                            // other platforms are not directly supported but this is may be TODO.
                            regenerated_code.push_str(
                            format!("{} // not supported\n", " ".repeat(line as usize)).as_str())

                    }

                }
                EliteKeywords::Argument => {
                    regenerated_code.push_str(
                        format!("{}if(len(os.Args) >= 2 && os.Args[1] == \"{}\")", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Exists => {
                    regenerated_code.push_str(
                        format!("{}if(exists(\"{}\"))", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Undefined => {},
                _ => {}
            }
        }

        regenerated_code.push_str("}\n");
        regenerated_code
    }
}