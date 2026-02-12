# 25 - DAV Client Setup

Step-by-step instructions for connecting native OS clients via WebDAV, CalDAV, and CardDAV. Covers Windows, macOS, Linux, iOS, and Android.

## Table of Contents

- [WebDAV Setup](#webdav-setup) (File Access)
- [CalDAV Setup](#caldav-setup) (Calendar Sync)
- [CardDAV Setup](#carddav-setup) (Contact Sync)
- [Troubleshooting](#troubleshooting)

---

## WebDAV Setup

### Connection Information

- **Server URL**: `https://[your-oxicloud-server]/webdav/`
- **Username**: Your OxiCloud username
- **Password**: Your OxiCloud password

### Windows

#### Windows 10/11 (File Explorer)

1. Open File Explorer
2. Right-click on "This PC" and select "Add a network location"
3. Click "Next"
4. Select "Choose a custom network location" and click "Next"
5. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
6. When prompted, enter your username and password
7. Give the connection a name (e.g., "OxiCloud") and click "Next"
8. Click "Finish"

Files now appear as a network drive in File Explorer.

#### Alternative: Map Network Drive

1. Open File Explorer
2. Right-click on "This PC" and select "Map network drive"
3. Choose a drive letter
4. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
5. Check "Connect using different credentials"
6. Click "Finish"
7. Enter your username and password

**Windows Troubleshooting:**

If connections fail on Windows:

1. Open Registry Editor (regedit.exe)
2. Navigate to `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WebClient\Parameters`
3. Modify **BasicAuthLevel** to value `2`
4. Restart the WebClient service or reboot

To increase the file size limit:
1. In the same registry location, modify **FileSizeLimitInBytes** to a higher value (e.g., `4294967295` for 4GB)
2. Restart the WebClient service

### macOS

#### Finder

1. Open Finder
2. From the menu bar, click "Go" > "Connect to Server" (or press Cmd+K)
3. Enter the WebDAV URL: `https://[your-oxicloud-server]/webdav/`
4. Click "Connect"
5. Enter your username and password
6. Click "Connect"

Files appear as a mounted drive in Finder.

### Linux

#### GNOME (Nautilus)

1. Open Files (Nautilus)
2. Click the "+" button in the sidebar or press Ctrl+L
3. Enter: `davs://[your-oxicloud-server]/webdav/`
4. Enter credentials when prompted
5. Click "Connect"

#### KDE (Dolphin)

1. Open Dolphin
2. In the address bar, enter: `webdavs://[your-oxicloud-server]/webdav/`
3. Enter credentials when prompted
4. Click "Connect"

#### Command Line (davfs2)

1. Install davfs2: `sudo apt-get install davfs2` (Debian/Ubuntu) or equivalent
2. Create a mount point: `sudo mkdir /mnt/oxicloud`
3. Edit `/etc/davfs2/secrets` and add: `/mnt/oxicloud [username] [password]`
4. Mount: `sudo mount -t davfs https://[your-oxicloud-server]/webdav/ /mnt/oxicloud`

To auto-mount at boot, add to `/etc/fstab`:
```
https://[your-oxicloud-server]/webdav/ /mnt/oxicloud davfs user,rw,auto 0 0
```

---

## CalDAV Setup

### Apple Calendar (macOS/iOS)

#### macOS:

1. Open the Calendar app
2. Go to **Calendar** > **Add Account** > **Other CalDAV Account**
3. Enter:
   - **Account Type**: Advanced
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Server Address**: `https://[your-oxicloud-server]/caldav`
4. Click **Sign In**
5. Select the calendars you want to sync and click **Done**

#### iOS:

1. Go to **Settings** > **Calendar** > **Accounts** > **Add Account** > **Other**
2. Tap **Add CalDAV Account**
3. Enter:
   - **Server**: `https://[your-oxicloud-server]/caldav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Description**: OxiCloud Calendar (or any name)
4. Tap **Next**
5. Turn on **Calendars** and tap **Save**

### Thunderbird with Lightning

1. Open Thunderbird and go to the **Calendar** tab
2. Right-click in the left pane and select **New Calendar**
3. Select **On the Network** and click **Next**
4. Choose **CalDAV** as the format
5. Enter the location: `https://[your-oxicloud-server]/caldav/calendars/your-calendar-id`
6. Click **Next**
7. Enter a name for the calendar and choose a color
8. Click **Next** and then **Finish**
9. When prompted, enter your username and password

### Android (DAVx5)

1. Install [DAVx5](https://play.google.com/store/apps/details?id=at.bitfire.davdroid) from Google Play Store
2. Open DAVx5 and tap the **+** button
3. Select **Login with URL and username**
4. Enter:
   - **Base URL**: `https://[your-oxicloud-server]/caldav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
5. Tap **Connect**
6. Select the calendars you want to sync
7. Tap the checkbox to enable syncing

### Windows (Outlook)

1. Download and install [CalDAV Synchronizer](https://caldavsynchronizer.org/)
2. Open Outlook and navigate to the **CalDAV Synchronizer** tab
3. Click **Synchronization Profiles**
4. Click **Add** to create a new profile
5. Enter:
   - **Profile Name**: OxiCloud Calendar (or any name)
   - **CalDAV URL**: `https://[your-oxicloud-server]/caldav/calendars/your-calendar-id`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
6. Click **Test or discover settings**
7. Select the Outlook calendar to sync with
8. Click **OK** to save the profile

---

## CardDAV Setup

### Apple Contacts (macOS/iOS)

#### macOS:

1. Open the Contacts app
2. Go to **Contacts** > **Add Account** > **Other contacts account**
3. Select **CardDAV account**
4. Enter:
   - **Server**: `https://[your-oxicloud-server]/carddav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Description**: OxiCloud Contacts (or any name)
5. Click **Sign In**

#### iOS:

1. Go to **Settings** > **Contacts** > **Accounts** > **Add Account** > **Other**
2. Tap **Add CardDAV Account**
3. Enter:
   - **Server**: `https://[your-oxicloud-server]/carddav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
   - **Description**: OxiCloud Contacts (or any name)
4. Tap **Next**
5. Turn on **Contacts** and tap **Save**

### Thunderbird

1. Open Thunderbird and go to the **Address Book**
2. Click on **Tools** > **Address Book**
3. Go to **File** > **New** > **Remote Address Book**
4. Enter:
   - **Name**: OxiCloud Contacts (or any name)
   - **URL**: `https://[your-oxicloud-server]/carddav/address-books/your-address-book-id`
5. Click **OK**
6. When prompted, enter your username and password

### Android (DAVx5)

1. Install [DAVx5](https://play.google.com/store/apps/details?id=at.bitfire.davdroid) from Google Play Store
2. Open DAVx5 and tap the **+** button
3. Select **Login with URL and username**
4. Enter:
   - **Base URL**: `https://[your-oxicloud-server]/carddav`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
5. Tap **Connect**
6. Select the address books you want to sync
7. Tap the checkbox to enable syncing

### Windows (Outlook)

1. Download and install [CardDAV Synchronizer](https://caldavsynchronizer.org/) (same tool as CalDAV)
2. Open Outlook and navigate to the **CardDAV Synchronizer** tab
3. Click **Synchronization Profiles**
4. Click **Add** to create a new profile
5. Select **CardDAV** as the synchronization resource
6. Enter:
   - **Profile Name**: OxiCloud Contacts (or any name)
   - **CardDAV URL**: `https://[your-oxicloud-server]/carddav/address-books/your-address-book-id`
   - **Username**: Your OxiCloud username
   - **Password**: Your OxiCloud password
7. Click **Test or discover settings**
8. Select the Outlook contacts folder to sync with
9. Click **OK** to save the profile

---

## Troubleshooting

### Common Issues

#### WebDAV Connection Issues

- Verify the server URL includes the `/webdav/` path
- Double-check username and password
- Check if your network blocks ports 80/443
- Confirm WebDAV is enabled on the server

#### Calendar/Contact Sync Issues

- Verify the server URL includes the full path (`/caldav` or `/carddav`)
- Confirm the server is reachable from your network
- Verify the calendar or address book ID is correct
- Check you have proper permissions for the resource

#### Calendar Not Showing

- Confirm the calendar is enabled in your client
- Check if the calendar is shared with your account
- Ensure your client supports the CalDAV protocol version in use

#### Contact Photos Not Syncing

- Some clients have photo sync limitations
- Verify the photo is in a supported format (usually JPEG)
- Check photo size limits on the client side

### Client-Specific Issues

#### Windows File Explorer

- Make sure the **WebClient** service is running
- Increase timeout values in the registry
- Try a third-party WebDAV client like Cyberduck

#### iOS Devices

- If connection fails, try **Settings** > **Accounts & Passwords** and add the account manually
- For persistent issues, remove the account and re-add it

#### Android

- DAVx5 requires battery optimization to be disabled for reliable background sync
- Go to **Settings** > **Apps** > **DAVx5** > **Battery** > **Unrestricted**

#### Outlook

- Make sure you have the latest version of CalDAV/CardDAV Synchronizer
- The plugin may need reactivation after Outlook updates

### Performance Tips

1. **Large Files** -- for files over 100MB, download locally before editing
2. **Slow Connections** -- enable offline caching in your client when available
3. **File Locking** -- some clients support WebDAV locking to prevent conflicts

### Getting Help

If issues persist:

1. Check the server logs for error messages
2. Capture screenshots of any errors
3. Contact support with details about your client application, version, steps to reproduce, and error messages
