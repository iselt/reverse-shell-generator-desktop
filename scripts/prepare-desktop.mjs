import { cpSync, mkdirSync, rmSync, existsSync, writeFileSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const out = join(root, 'dist-desktop');

if (existsSync(out)) {
  rmSync(out, { recursive: true, force: true });
}
mkdirSync(out, { recursive: true });

const copy = (rel) => {
  const src = join(root, rel);
  if (!existsSync(src)) {
    console.warn(`[prepare-desktop] skip missing: ${rel}`);
    return;
  }
  cpSync(src, join(out, rel), { recursive: true });
};

for (const rel of ['index.html', 'favicon.ico', 'js', 'css', 'assets']) {
  copy(rel);
}

// Desktop defaults: offline raw/download (no Netlify).
const indexPath = join(out, 'index.html');
let html = readFileSync(indexPath, 'utf8');
if (!html.includes('RSG_USE_SERVER_RAW')) {
  html = html.replace(
    '<head>',
    `<head>\n    <script>window.RSG_USE_SERVER_RAW = false;</script>`
  );
  writeFileSync(indexPath, html);
}

console.log(`[prepare-desktop] wrote ${out}`);
