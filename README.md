## 📖 CanvasCLI - Command Line Interface for Canvas LMS (canvascli)
CanvasCLI is a powerful tool for UPenn students to interact with the Canvas LMS from the command line.

[UPenn Canvas LMS](canvas.upenn.edu) api.
- Student managed access token, ensures full privacy.
- Automate self-hosting of .ics calendar feeds.

### Usage:

### 🚀 Installation
```sh
# Coming soon...
```

### 🔧 Setup & Sync
```sh
canvascli setup                     # Setup API connection by prompting for a user token
canvascli sync                       # Sync course data from Canvas API
```

### 📚 Course Management
```sh
canvascli course ls                  # List active courses
canvascli course ls --all             # List all courses (including old/inactive)
```

### 📅 Calendar Management
```sh
canvascli calendar ls                 # List all calendar items chronologically
canvascli calendar ls --course <id>    # List calendar items for a specific course
canvascli calendar ls --filtered       # Show filtered calendar items
```

### 🎯 Calendar Filters
```sh
canvascli calendar filter add "<regex>" --course <id>   # Add regex filter to a course's calendar
canvascli calendar filter ls                             # Show list of active filters
canvascli calendar filter rm "<regex>" --course <id>    # Remove a specific filter
```

### 📆 Publishing
```sh
canvascli calendar publish --filtered  # Generate & publish filtered .ics feed
```

### ℹ️ Help
```sh
canvascli --help         # Show general help
canvascli <command> --help  # Show help for a specific command
```

🔹 **Designed for efficiency** – Short and intuitive commands.
🔹 **Flexible & Powerful** – Easily manage courses and calendars from the CLI.
🔹 **Expanding Features** – More integrations coming soon!

📌 *Stay tuned for updates and contributions!*


### Potential future feature implementation..
- Add more [schools](https://community.canvaslms.com/t5/Canvas-Basics-Guide/Where-do-I-find-my-institution-s-URL-to-access-Canvas/ta-p/82)
- Integrated directly with google calendars to automatically setup feed to newly published .ics feed.
- pending further thoughts...
