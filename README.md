## 📖 CanvasCLI - Command Line Interface for Canvas LMS

CanvasCLI is a powerful tool for UPenn students to interact with the Canvas LMS from the command line.

[UPenn Canvas LMS](canvas.upenn.edu) API.
- Student-managed access token ensures full privacy.
- Automate self-hosting of .ics calendar feeds.

### 🚀 Installation
```sh
# Coming soon...
```

### 🔧 Setup & Sync
```sh
canvascli setup                     # Setup API connection by prompting for a user token
canvascli sync                      # Sync course data from Canvas API
```

### 📚 Course Management
```sh
canvascli course ls                  # List active courses
canvascli course ls --all            # List all courses (including old/inactive)
canvascli course ls --published      # Show only courses with published .ics feeds
```

### 📅 Calendar Management
```sh
canvascli calendar ls                                # List all calendar items chronologically
canvascli calendar ls --course <id|name>             # List calendar items for a specific course
canvascli calendar ls --filtered                     # Show filtered calendar items
```

### 🎯 Calendar Filters (TUI-Based)
```sh
canvascli calendar filter                            # Launch interactive terminal UI for managing filters
```

#### 🖥️ Using the Terminal UI
- Navigate using arrow keys (`↑↓`)
- Add a new filter: Press `a`, then choose `include` or `exclude`, and enter a regex pattern
- Remove a filter: Select a filter and press `d`
- Edit a filter: Select a filter and press `e`
- Apply changes and exit: Press `q`

### 📆 Publishing (Google Drive - MVP)
```sh
canvascli calendar publish setup                     # Setup Google Drive API authentication
canvascli calendar publish --course <id|name>        # Publish a single course's .ics feed
canvascli calendar publish --all                     # Publish all active course .ics feeds
canvascli calendar publish --filtered                # Publish all active filtered .ics feeds
```
- **First-time setup:** Users will authenticate with Google Drive via OAuth2.
- **Publishing:** Uploads `.ics` to Google Drive and provides a shareable link.

### 🔄 Auto-Update Published Feeds
To ensure `.ics` feeds stay updated, users can enable automatic updates using the CLI.

#### **Enable Auto-Update**
```sh
canvascli calendar autoupdate enable  # Automatically sync and publish every 4 hours
```
- Adds a cron job (Linux/macOS) or Task Scheduler task (Windows) to run every 4 hours.
- Runs `canvascli sync` followed by `canvascli calendar publish --all --filtered`.

#### **Disable Auto-Update**
```sh
canvascli calendar autoupdate disable  # Remove the scheduled auto-update task
```
- Removes the previously created cron job or scheduled task.

#### **Manual Cron Job Setup (Alternative)**
```sh
crontab -e  # Open the crontab editor
0 */4 * * * canvascli sync && canvascli calendar publish --all --filtered  # Run every 4 hours
```

📌 **Note:** Auto-updates only run when the user's computer is turned on.

### 📜 Viewing Published Feeds
```sh
canvascli calendar published              # List all courses with published .ics feeds and their URLs
canvascli calendar published --course <id|name>  # Show the .ics link for a specific course
```

### 🗑️ Unpublishing
```sh
canvascli calendar unpublish --course <id|name>  # Remove a published .ics feed
```

### ℹ️ Help
```sh
canvascli --help         # Show general help
canvascli <command> --help  # Show help for a specific command
```

## Key Features

**🔹 Designed for efficiency**  
Short and intuitive commands  

**🔹 Flexible & Powerful**  
Easily manage courses and calendars from the CLI  

**🔹 Automatic Feed Updates**  
Scheduled sync ensures `.ics` feeds stay current when the computer is on.  

**🔹 Expanding Features**  
More integrations coming soon!  

### 🚀 Future Enhancements
- **Additional Cloud Providers:** Support for OneDrive and Dropbox for `.ics` hosting.
- **Provider Switching:** Allow users to select and switch between providers.
- **Self-Hosting:** Option to upload `.ics` files to a personal server.
- **Advanced Filtering:** More complex event filtering logic.

📌 *Stay tuned for updates and contributions!*
