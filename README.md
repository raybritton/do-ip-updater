## DigitalOcean Firewall address updater

![Main](https://github.com/raybritton/do-ip-updater/workflows/Main/badge.svg?branch=master)
[![dependency status](https://deps.rs/repo/github/raybritton/do-ip-updater/status.svg)](https://deps.rs/repo/github/raybritton/do-ip-updater)

Designed to be run from a network that changes it's external (internet) IP address regularly, it will replace all addresses on for the specified port for the specified firewall to current external IP every time it checks.

### Usage

```
Regularly checks that the internet IP address of this program is on a DigitalOcean firewall for the SSH port, and sets
it if not

USAGE:
    do_ip_updater [FLAGS] [OPTIONS] --token <TOKEN> --id <ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Set verbosity of program (between 0 and 3)

OPTIONS:
    -t, --token <TOKEN>          DigitalOcean Bearer Token
    -i, --id <ID>                DigitalOcean Firewall ID
    -f, --frequency <MINUTES>    How often (in minutes) to check the IP address is set [default: 30]
    -o, --once                   Run once then exit
    -p, --port <PORT>            Port in the firewall to be updated [default: 22]
```

###  License

```
Copyright 2020 Ray Britton

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```