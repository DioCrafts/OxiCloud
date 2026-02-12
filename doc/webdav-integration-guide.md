# 22 - WebDAV Integration Guide

The WebDAV interface exposes file operations over HTTP at a single base path. All standard WebDAV methods are supported: **PROPFIND**, **GET**, **PUT**, **MKCOL**, **MOVE**, **COPY**, **DELETE**. Authentication is HTTP Basic over TLS.

## Table of Contents

1. [Base URL and Endpoints](#base-url-and-endpoints)
2. [Authentication](#authentication)
3. [Common Operations](#common-operations)
   - [Listing Directories](#listing-directories)
   - [Downloading Files](#downloading-files)
   - [Uploading Files](#uploading-files)
   - [Creating Folders](#creating-folders)
   - [Moving and Copying](#moving-and-copying)
   - [Deleting Resources](#deleting-resources)
4. [XML Schemas](#xml-schemas)
5. [Code Examples](#code-examples)
6. [Extending WebDAV](#extending-webdav)
7. [Troubleshooting](#troubleshooting)

## Base URL and Endpoints

The WebDAV interface lives at:

```
https://[your-oxicloud-server]/webdav/
```

All file and folder operations hang off this base path. Append the resource path to the URL.

Examples:
- Root folder: `https://[your-oxicloud-server]/webdav/`
- File "document.pdf" in root: `https://[your-oxicloud-server]/webdav/document.pdf`
- Folder "projects": `https://[your-oxicloud-server]/webdav/projects/`
- File in subfolder: `https://[your-oxicloud-server]/webdav/projects/proposal.docx`

## Authentication

WebDAV uses HTTP Basic Authentication. Include the `Authorization` header with base64-encoded credentials:

```
Authorization: Basic base64(username:password)
```

Always use HTTPS.

## Common Operations

### Listing Directories

Use the **PROPFIND** method with a **Depth** header:

- `Depth: 0` -- info about the resource itself
- `Depth: 1` -- the resource and its immediate children (recommended)
- `Depth: infinity` -- the resource and all descendants (careful with large trees)

Request:
```http
PROPFIND /webdav/projects/ HTTP/1.1
Host: your-oxicloud-server
Depth: 1
Content-Type: application/xml
Authorization: Basic [credentials]

<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>
```

Response:
```http
HTTP/1.1 207 Multi-Status
Content-Type: application/xml; charset=utf-8

<?xml version="1.0" encoding="utf-8" ?>
<D:multistatus xmlns:D="DAV:">
  <D:response>
    <D:href>/webdav/projects/</D:href>
    <D:propstat>
      <D:prop>
        <D:resourcetype><D:collection/></D:resourcetype>
        <D:displayname>projects</D:displayname>
        <!-- Other properties... -->
      </D:prop>
      <D:status>HTTP/1.1 200 OK</D:status>
    </D:propstat>
  </D:response>
  <!-- Child resources... -->
</D:multistatus>
```

### Downloading Files

Standard HTTP **GET**:

```http
GET /webdav/projects/document.pdf HTTP/1.1
Host: your-oxicloud-server
Authorization: Basic [credentials]
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/pdf
Content-Length: 12345
Last-Modified: Wed, 15 Nov 2023 12:34:56 GMT
ETag: "abc123"

[File content]
```

### Uploading Files

Use HTTP **PUT** to upload or update a file:

```http
PUT /webdav/projects/document.pdf HTTP/1.1
Host: your-oxicloud-server
Content-Type: application/pdf
Content-Length: 12345
Authorization: Basic [credentials]

[File content]
```

New files return `201 Created`. Updates return `204 No Content`.

### Creating Folders

Use the **MKCOL** method:

```http
MKCOL /webdav/projects/new-folder HTTP/1.1
Host: your-oxicloud-server
Authorization: Basic [credentials]
```

Returns `201 Created` on success.

### Moving and Copying

**MOVE** a resource:

```http
MOVE /webdav/old-location.pdf HTTP/1.1
Host: your-oxicloud-server
Destination: https://your-oxicloud-server/webdav/new-location.pdf
Authorization: Basic [credentials]
```

**COPY** a resource:

```http
COPY /webdav/original.pdf HTTP/1.1
Host: your-oxicloud-server
Destination: https://your-oxicloud-server/webdav/copy.pdf
Authorization: Basic [credentials]
```

Both return `204 No Content` on success.

### Deleting Resources

Use HTTP **DELETE**:

```http
DELETE /webdav/projects/document.pdf HTTP/1.1
Host: your-oxicloud-server
Authorization: Basic [credentials]
```

Returns `204 No Content` on success.

## XML Schemas

### PROPFIND Request

Request all properties:

```xml
<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>
```

Request specific properties:

```xml
<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:prop>
    <D:displayname/>
    <D:getcontentlength/>
    <D:getlastmodified/>
  </D:prop>
</D:propfind>
```

### PROPPATCH Request

Set and remove properties:

```xml
<?xml version="1.0" encoding="utf-8" ?>
<D:propertyupdate xmlns:D="DAV:" xmlns:Z="http://example.org/custom/">
  <D:set>
    <D:prop>
      <Z:custom-property>Custom Value</Z:custom-property>
    </D:prop>
  </D:set>
  <D:remove>
    <D:prop>
      <Z:old-property/>
    </D:prop>
  </D:remove>
</D:propertyupdate>
```

### LOCK Request

```xml
<?xml version="1.0" encoding="utf-8" ?>
<D:lockinfo xmlns:D="DAV:">
  <D:lockscope><D:exclusive/></D:lockscope>
  <D:locktype><D:write/></D:locktype>
  <D:owner>
    <D:href>mailto:user@example.com</D:href>
  </D:owner>
</D:lockinfo>
```

## Code Examples

### Python Example

Using the `requests` library:

```python
import requests
from requests.auth import HTTPBasicAuth
import xml.etree.ElementTree as ET

# Set up authentication
auth = HTTPBasicAuth('username', 'password')
base_url = 'https://your-oxicloud-server/webdav'

# 1. List directory contents
headers = {'Depth': '1'}
body = '''<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>'''

response = requests.request(
    'PROPFIND',
    f'{base_url}/projects/',
    headers=headers,
    data=body,
    auth=auth
)

if response.status_code == 207:  # Multi-Status
    # Parse XML response
    root = ET.fromstring(response.content)
    for response_elem in root.findall('.//{DAV:}response'):
        href = response_elem.find('.//{DAV:}href').text
        print(f"Resource: {href}")

        # Get displayname if available
        displayname = response_elem.find('.//{DAV:}displayname')
        if displayname is not None and displayname.text:
            print(f"  Name: {displayname.text}")

        # Check if it's a collection (folder)
        resourcetype = response_elem.find('.//{DAV:}resourcetype')
        is_collection = resourcetype is not None and resourcetype.find('.//{DAV:}collection') is not None
        print(f"  Type: {'Folder' if is_collection else 'File'}")

        # Get size if it's a file
        if not is_collection:
            contentlength = response_elem.find('.//{DAV:}getcontentlength')
            if contentlength is not None and contentlength.text:
                print(f"  Size: {contentlength.text} bytes")

# 2. Upload a file
with open('local-file.pdf', 'rb') as f:
    file_content = f.read()

response = requests.put(
    f'{base_url}/projects/document.pdf',
    data=file_content,
    auth=auth
)

if response.status_code in (201, 204):
    print("File uploaded successfully")

# 3. Download a file
response = requests.get(
    f'{base_url}/projects/document.pdf',
    auth=auth
)

if response.status_code == 200:
    with open('downloaded-file.pdf', 'wb') as f:
        f.write(response.content)
    print("File downloaded successfully")

# 4. Create a folder
response = requests.request(
    'MKCOL',
    f'{base_url}/projects/new-folder',
    auth=auth
)

if response.status_code == 201:
    print("Folder created successfully")

# 5. Move a file
headers = {
    'Destination': f'{base_url}/projects/new-location.pdf'
}
response = requests.request(
    'MOVE',
    f'{base_url}/projects/old-location.pdf',
    headers=headers,
    auth=auth
)

if response.status_code == 204:
    print("File moved successfully")

# 6. Delete a file
response = requests.delete(
    f'{base_url}/projects/document.pdf',
    auth=auth
)

if response.status_code == 204:
    print("File deleted successfully")
```

### JavaScript Example

Using the browser `fetch` API:

```javascript
// Base configuration
const baseUrl = 'https://your-oxicloud-server/webdav';
const credentials = btoa('username:password');
const headers = {
  'Authorization': `Basic ${credentials}`
};

// 1. List directory contents
async function listDirectory(path) {
  const response = await fetch(`${baseUrl}${path}`, {
    method: 'PROPFIND',
    headers: {
      ...headers,
      'Depth': '1',
      'Content-Type': 'application/xml'
    },
    body: `<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
  <D:allprop/>
</D:propfind>`
  });

  if (response.status === 207) {
    const text = await response.text();
    const parser = new DOMParser();
    const xmlDoc = parser.parseFromString(text, 'text/xml');

    const responses = xmlDoc.getElementsByTagNameNS('DAV:', 'response');
    const resources = [];

    for (let i = 0; i < responses.length; i++) {
      const response = responses[i];
      const href = response.getElementsByTagNameNS('DAV:', 'href')[0].textContent;

      let displayName = '';
      const displayNameElems = response.getElementsByTagNameNS('DAV:', 'displayname');
      if (displayNameElems.length > 0) {
        displayName = displayNameElems[0].textContent;
      }

      // Check if resource is a collection (folder)
      const resourceTypeElem = response.getElementsByTagNameNS('DAV:', 'resourcetype')[0];
      const isCollection = resourceTypeElem.getElementsByTagNameNS('DAV:', 'collection').length > 0;

      // Get file size if it's a file
      let size = null;
      if (!isCollection) {
        const contentLengthElems = response.getElementsByTagNameNS('DAV:', 'getcontentlength');
        if (contentLengthElems.length > 0) {
          size = parseInt(contentLengthElems[0].textContent, 10);
        }
      }

      resources.push({
        href,
        displayName,
        isCollection,
        size
      });
    }

    return resources;
  } else {
    throw new Error(`Failed to list directory: ${response.status}`);
  }
}

// 2. Upload a file
async function uploadFile(path, fileContent) {
  const response = await fetch(`${baseUrl}${path}`, {
    method: 'PUT',
    headers: {
      ...headers,
      'Content-Type': 'application/octet-stream'
    },
    body: fileContent
  });

  return response.status === 201 || response.status === 204;
}

// Example usage with a File object from an input
const fileInput = document.getElementById('fileInput');
fileInput.addEventListener('change', async (event) => {
  const file = event.target.files[0];
  if (file) {
    const result = await uploadFile(`/projects/${file.name}`, file);
    console.log(`Upload ${result ? 'successful' : 'failed'}`);
  }
});

// 3. Download a file
async function downloadFile(path) {
  const response = await fetch(`${baseUrl}${path}`, {
    method: 'GET',
    headers
  });

  if (response.status === 200) {
    return await response.blob();
  } else {
    throw new Error(`Failed to download: ${response.status}`);
  }
}

// Example usage with download attribute
async function downloadAndSave(path, filename) {
  try {
    const blob = await downloadFile(path);
    const url = URL.createObjectURL(blob);

    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();

    // Clean up
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('Download failed:', error);
  }
}

// 4. Create a folder
async function createFolder(path) {
  const response = await fetch(`${baseUrl}${path}`, {
    method: 'MKCOL',
    headers
  });

  return response.status === 201;
}

// 5. Move a file
async function moveResource(fromPath, toPath) {
  const response = await fetch(`${baseUrl}${fromPath}`, {
    method: 'MOVE',
    headers: {
      ...headers,
      'Destination': `${baseUrl}${toPath}`
    }
  });

  return response.status === 204;
}

// 6. Delete a resource
async function deleteResource(path) {
  const response = await fetch(`${baseUrl}${path}`, {
    method: 'DELETE',
    headers
  });

  return response.status === 204;
}
```

### C# Example

```csharp
using System;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;

class WebDavClient
{
    private readonly HttpClient _httpClient;
    private readonly string _baseUrl;

    public WebDavClient(string baseUrl, string username, string password)
    {
        _baseUrl = baseUrl.TrimEnd('/') + "/webdav";
        _httpClient = new HttpClient();

        // Set Basic Authentication
        var credentials = Convert.ToBase64String(Encoding.UTF8.GetBytes($"{username}:{password}"));
        _httpClient.DefaultRequestHeaders.Authorization =
            new AuthenticationHeaderValue("Basic", credentials);
    }

    public async Task<XDocument> ListDirectoryAsync(string path)
    {
        var request = new HttpRequestMessage(new HttpMethod("PROPFIND"), $"{_baseUrl}/{path.TrimStart('/')}");
        request.Headers.Add("Depth", "1");
        request.Content = new StringContent(
            @"<?xml version=""1.0"" encoding=""utf-8"" ?>
            <D:propfind xmlns:D=""DAV:"">
              <D:allprop/>
            </D:propfind>",
            Encoding.UTF8,
            "application/xml"
        );

        var response = await _httpClient.SendAsync(request);

        if (response.StatusCode == System.Net.HttpStatusCode.MultiStatus)
        {
            var content = await response.Content.ReadAsStringAsync();
            return XDocument.Parse(content);
        }

        throw new Exception($"Failed to list directory: {response.StatusCode}");
    }

    public async Task<bool> UploadFileAsync(string path, byte[] content)
    {
        var request = new HttpRequestMessage(HttpMethod.Put, $"{_baseUrl}/{path.TrimStart('/')}");
        request.Content = new ByteArrayContent(content);

        var response = await _httpClient.SendAsync(request);

        return response.StatusCode == System.Net.HttpStatusCode.Created ||
               response.StatusCode == System.Net.HttpStatusCode.NoContent;
    }

    public async Task<byte[]> DownloadFileAsync(string path)
    {
        var response = await _httpClient.GetAsync($"{_baseUrl}/{path.TrimStart('/')}");

        if (response.IsSuccessStatusCode)
        {
            return await response.Content.ReadAsByteArrayAsync();
        }

        throw new Exception($"Failed to download file: {response.StatusCode}");
    }

    public async Task<bool> CreateFolderAsync(string path)
    {
        var request = new HttpRequestMessage(new HttpMethod("MKCOL"), $"{_baseUrl}/{path.TrimStart('/')}");
        var response = await _httpClient.SendAsync(request);

        return response.StatusCode == System.Net.HttpStatusCode.Created;
    }

    public async Task<bool> MoveResourceAsync(string fromPath, string toPath)
    {
        var request = new HttpRequestMessage(new HttpMethod("MOVE"), $"{_baseUrl}/{fromPath.TrimStart('/')}");
        request.Headers.Add("Destination", $"{_baseUrl}/{toPath.TrimStart('/')}");

        var response = await _httpClient.SendAsync(request);

        return response.StatusCode == System.Net.HttpStatusCode.NoContent;
    }

    public async Task<bool> DeleteResourceAsync(string path)
    {
        var response = await _httpClient.DeleteAsync($"{_baseUrl}/{path.TrimStart('/')}");

        return response.StatusCode == System.Net.HttpStatusCode.NoContent;
    }
}

// Example usage
async Task RunExampleAsync()
{
    var client = new WebDavClient("https://your-oxicloud-server", "username", "password");

    // List directory
    try
    {
        var directoryListing = await client.ListDirectoryAsync("/projects");
        // Process XML results...
        Console.WriteLine("Directory listing successful");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error listing directory: {ex.Message}");
    }

    // Upload a file
    try
    {
        var fileContent = await File.ReadAllBytesAsync("local-file.pdf");
        var result = await client.UploadFileAsync("/projects/document.pdf", fileContent);
        Console.WriteLine($"Upload {(result ? "successful" : "failed")}");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error uploading file: {ex.Message}");
    }

    // Download a file
    try
    {
        var fileContent = await client.DownloadFileAsync("/projects/document.pdf");
        await File.WriteAllBytesAsync("downloaded-file.pdf", fileContent);
        Console.WriteLine("Download successful");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error downloading file: {ex.Message}");
    }
}
```

## Extending WebDAV

### Adding Custom Properties

To support custom WebDAV properties:

1. Define your XML namespace for custom properties.
2. Implement storage for them (database table recommended).
3. Update the WebDAV adapter to handle these properties.

Example adapter code for custom properties:

```rust
// Add to WebDavAdapter implementation
fn handle_custom_property(name: &QualifiedName, value: Option<&str>) -> Result<bool> {
    if name.namespace == "http://example.org/custom/" {
        // Store the custom property in your database
        // ...
        return Ok(true);
    }

    // Property not handled
    Ok(false)
}
```

### Supporting CalDAV/CardDAV

To extend with CalDAV/CardDAV support:

1. Create additional adapters for calendar and contact data.
2. Implement the additional XML namespaces required.
3. Create handlers for the specialized methods.
4. Integrate with calendar and contacts storage.

See `caldav-technical-spec.md` and `carddav-technical-spec.md` for details.

## Troubleshooting

### Common Issues

1. **Authentication Failures**
   - Check credentials are correctly Base64-encoded
   - Ensure the Authorization header is formatted correctly
   - Verify the user has the necessary permissions
