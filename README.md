![work-break](screenshot.png)

## Introduction
This balancer can track your work time and suggest resting time. It uses this [graph](https://www.desmos.com/calculator/duqezlkow8) by default,
where the horizontal axis is for work time, and the vertical axis is for resting time. It passes through the [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) (25/5) and the [52/17 rule](https://en.wikipedia.org/wiki/52/17_rule). The formula can be adjusted (see the configuration section below).

The more you work, the more rest you need per work minute. Sometimes, you have to start working without waiting until you have rested suggested time. In this case, this balancer accumulates strain by subtracting the actual rest time from the needed rest time and converting the result into work time back. It adds up to the following work on the next break.

This balancer sends you notifications on the following events:
* Work lasted for 25 minutes (can be changed)
* It lasted for 52 minutes (can be changed)
* It lasted for the daily work time limit if it is configured
* Break ended

The notification contains the current phase, strain and today work time, needed break, its end if it starts.

## How to install

### Compatibility

Windows 8+ (requires [MS Visual C++ 2015](https://www.microsoft.com/en-us/download/details.aspx?id=52685), tested on Windows 10)

MacOS 10+ (tested on MacOS 11.7)

Linux/BSD: should work with many distributions (tested on Arch Linux)

### Download a binary

Download a suitable binary from [releases](https://github.com/ShadoySV/work-break/releases) page.

### Arch Linux based distributions

The app can be installed using the PKGBUILD [work-break](https://aur.archlinux.org/packages/work-break), available on the [AUR](https://wiki.archlinux.org/index.php/Arch_User_Repository). This can be built and installed using an AUR helper or [by hand in the usual way](https://wiki.archlinux.org/title/Arch_User_Repository#Installing_and_upgrading_packages).

### With Cargo

You will need the Rust programming language (v1.59.0+) and its cargo package manager installed on your system. See the official documentation [here](https://www.rust-lang.org/tools/install).

Run this command to install the app (make sure that the cargo bin directory is in $PATH):
```
cargo install work-break
```

Configure a shortcut key or create a desktop icon to switch between work and rest time:
```
work-break
```

Configure the following command to launch the balancer on startup to get notifications (optional, but recommended):
```
work-break autorun
```

Configure a shortcut key or create a desktop icon to notify about the current status (optional):
```
work-break notify
```

To print the current status in CLI, you can use this (MacOS / Linux):
```
work-break status
```

## Configuration
After the first app launch, you can change the app's configuration by editing the following configuration file:

##### Windows
```
%APPDATA%\work-break\config\default-config.toml
```

##### MacOS
```
~/Library/Application\ Support/rs.work-break/default-config.toml
```
##### Linux
```
~/.config/work-break/default-config.toml
```
or you can delete the file to get the defaults:

##### Defaults
```
coefficient_a = 0.00147884224225867
coefficient_b = 1.67098454496329
coefficient_c = 0
coefficient_d = 0

daily_work_time_limit = 0
work_days_start_at = 0

phase1_ends_at = 25
phase1_name = "Pomodoro"
phase2_ends_at = 52
phase2_name = "Efficiency"
phase3_name = "Injury"

```
The **coefficients** are used for the formula: **break = a * (work ^ (b + d * today_work)) + c**. Variables **work**, **today_work** and **break** represent in seconds. Consider setting **coefficient_d** to `0.00001528` to see how today work time can increase resting time.

**daily_work_time_limit** represents in minutes, sends you notification when today work time reaches the limit, zero turns the notification off.
**work_days_start_at** defines an hour when work days start at and count resets (from 0 to 23).

**phase1_ends_at** and **phase2_ends_at** define current strain's thresholds to send you notifications.

**phase1_name**, **phase2_name**, and **phase3_name** define phases' names to print in notifications.

Restart your system or ask the app to apply the changed configuration:

```
work-break reload
```

# Similar projects

[Bartib](https://github.com/nikolassv/bartib) is a simple time tracker for the command line. It saves a log of all tracked activities as a plaintext file and allows you to create flexible reports.
