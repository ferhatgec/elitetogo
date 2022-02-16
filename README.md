# [elite](https://github.com/ferhatgec/elite)togo
## [elite](https://github.com/ferhatgec/elite) -> golang converter

### input:
```rs
required_version is 0.1

set ProjectName as "elitetogo"
set HOME        as env "HOME"


for argument "install" [
    use exec "cargo install --path ."

    for exists "{HOME}.cargo/bin/{ProjectName}" [
        println "{ProjectName} installed to {HOME}.cargo/bin/{ProjectName}."
    ]

    use signal "exit"
]
```

### output
```golang

package main

import (
"fmt"
"os"
"os/exec"
"runtime"
)

var cpuarch, opsys = runtime.GOARCH, runtime.GOOS

func out(command string) {
output, _ := exec.Command("/usr/bin/bash", "-c", command).Output()
fmt.Printf("%s", output)
}
func exists(name string) bool {
_, err := os.Stat(name)
return !os.IsNotExist(err)
}
func main() {
if("0.1" != "0.1"){
 fmt.Println("elite: Required higher version")
 os.Exit(1)
}
ProjectName := "elitetogo"
_ = ProjectName
HOME := "/home/gech"
_ = HOME
if(opsys == "linux"){
 fmt.Println("fuck")
}
if(len(os.Args) >= 2 && os.Args[1] == "install"){
 out("cargo install --path .")
 if(exists("/home/gech/.cargo/bin/elitetogo")){
  fmt.Println("elitetogo installed to /home/gech/.cargo/bin/elitetogo.")
}
 os.Exit(1)
}
}

```

### elitetogo licensed under the terms of MIT License
