
<h1>Rust Azure Blob Storage API</h1>
    <p>This is an example of a Rust API using the Rocket framework to upload and download files to and from Azure Blob Storage.</p>
    <h2>Routes:</h2>
    <ul>
        <li><strong>POST /upload</strong>: Route to upload a file.</li>
        <li><strong>GET /download</strong>: Route to download a file.</li>
    </ul>
    <h2>Configuration:</h2>
    <p>Before using this API, you need to replace the storage account information and access keys with your own:</p>
    <ul>
        <li><strong>account</strong>: Your Azure storage account name.</li>
        <li><strong>master_key</strong>: Your Azure storage account access key.</li>
        <li><strong>container</strong>: The name of the storage container where files will be stored.</li>
        <li><strong>blob_name</strong>: The name of the file to be uploaded and downloaded.</li>
    </ul>
    <h2>How to Use:</h2>
    <ol>
        <li>Run the Rust application.</li>
        <li>Make an HTTP POST request to <code>/upload</code> to send a file to the server.</li>
        <li>To download the file, make an HTTP GET request to <code>/download</code>.</li>
    </ol>
    <p>Make sure you have the necessary permissions and have installed the dependencies through Cargo.</p>
    <p>Also, ensure that your Azure storage account is configured properly to allow access from your application.</p>
