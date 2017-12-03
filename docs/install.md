```
                                                       .__       .___      /\         .__  .__ 
                      ____   _____   ________________  |  |    __| _/     / /    ____ |  | |__|
                    _/ __ \ /     \_/ __ \_  __ \__  \ |  |   / __ |     / /   _/ ___\|  | |  |
                    \  ___/|  Y Y  \  ___/|  | \// __ \|  |__/ /_/ |    / /    \  \___|  |_|  |
                     \___  >__|_|  /\___  >__|  (____  /____/\____ |   / /      \___  >____/__|
                        \/      \/     \/           \/           \/   \/           \/         
                                                        
```

# Installing emerald-cli

## REQUIREMENTS:

 Install Rust
  
  https://www.rust-lang.org/en-US/install.html
  
  Unix one-liner:
  
  `curl https://sh.rustup.rs -sSf | sh` 
  
  NOTE: On Windows, Rust additionally requires the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire the build tools is by installing Microsoft Visual C++ Build Tools 2017 which provides just the Visual C++ build tools.
  
## Installation

### Step 1

Clone the repository

`git clone https://github.com/ethereumproject/emerald-cli.git`

Move to the new directory

`cd emerald-cli`

### Step 2 

Compile with cargo

`cargo build --release`

### Step 3

 Move to install dir
 `cd target\debug`
 
 
