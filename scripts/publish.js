const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

function getGitHubToken() {
  if (process.env.GITHUB_TOKEN) return process.env.GITHUB_TOKEN;
  if (process.env.GH_TOKEN) return process.env.GH_TOKEN;
  try {
    const stdout = execSync('git credential fill', {
      input: 'protocol=https\nhost=github.com\n\n',
      encoding: 'utf8'
    });
    const match = stdout.match(/password=([^\r\n]+)/);
    if (match) return match[1].trim();
  } catch {}
  return null;
}

const token = getGitHubToken();
if (!token) {
  console.error('Error: GitHub token not found in environment or git credential manager.');
  process.exit(1);
}

const owner = 'Kryklin';
const repo = 'SynthIDStripper';
const tagName = 'v1.0.0';

async function run() {
  const headers = {
    'Authorization': `Bearer ${token}`,
    'User-Agent': 'SynthIDStripper-Publisher',
    'Accept': 'application/vnd.github.v3+json'
  };

  console.log(`Checking release for ${tagName}...`);
  const getRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/releases/tags/${tagName}`, { headers });
  if (!getRes.ok) {
    throw new Error(`Failed to find release ${tagName}: ${getRes.statusText}`);
  }
  const relData = await getRes.json();
  const releaseId = relData.id;
  console.log(`Release ID: ${releaseId} (${relData.html_url})`);

  const zipPath = path.join(__dirname, '../out-releases/synthid-stripper-v1.0.0-windows-x64.zip');
  if (!fs.existsSync(zipPath)) {
    throw new Error(`Package not found at ${zipPath}`);
  }
  const zipBuffer = fs.readFileSync(zipPath);
  const fileName = path.basename(zipPath);

  // Drop existing asset if present
  if (relData.assets && relData.assets.length > 0) {
    for (const asset of relData.assets) {
      if (asset.name === fileName) {
        console.log(`Deleting existing asset ${asset.id}...`);
        await fetch(`https://api.github.com/repos/${owner}/${repo}/releases/assets/${asset.id}`, {
          method: 'DELETE',
          headers
        });
      }
    }
  }

  console.log(`Uploading ${fileName} (${zipBuffer.length} bytes)...`);
  const uploadRes = await fetch(`https://uploads.github.com/repos/${owner}/${repo}/releases/${releaseId}/assets?name=${fileName}`, {
    method: 'POST',
    headers: {
      ...headers,
      'Content-Type': 'application/zip',
      'Content-Length': zipBuffer.length.toString()
    },
    body: zipBuffer
  });

  if (!uploadRes.ok) {
    const text = await uploadRes.text();
    throw new Error(`Failed to upload asset: ${uploadRes.status} ${text}`);
  }

  const assetData = await uploadRes.json();
  console.log(`Asset successfully uploaded!`);
  console.log(`Download URL: ${assetData.browser_download_url}`);
}

run().catch(err => {
  console.error(err);
  process.exit(1);
});
