#!/usr/bin/env node
/**
 * gen-icons.cjs — One Pharm app-icon generator
 *
 * Usage:
 *   node scripts/gen-icons.cjs          # Normal mode
 *   node scripts/gen-icons.cjs --silent # Quiet mode
 */

"use strict";

const { Resvg } = require("@resvg/resvg-js");
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

// Config & Paths
const ROOT = path.resolve(__dirname, "..");
const ICONS_DIR = path.join(ROOT, "src-tauri", "icons");
const SOURCE_PNG = path.join(ICONS_DIR, "icon.png");
const IS_SILENT = process.argv.includes("--silent");

const log = (msg) => !IS_SILENT && console.log(msg);
const error = (msg) => console.error(msg);

// ─── Icon SVG ───
// Adapts public/logo.svg at 1024×1024 on a canvas-dark (#010120) rounded-rect background.
// Uses local gradient IDs to avoid any ID collision when rendered standalone.
const ICON_SVG = `<svg width="1024" height="1024" viewBox="0 0 1024 1024" fill="none" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <radialGradient id="ic_bg_grad" cx="50%" cy="50%" r="50%" fx="30%" fy="30%">
      <stop offset="0%" stop-color="#ffffff"/>
      <stop offset="100%" stop-color="#cbd5e1"/>
    </radialGradient>

    <filter id="ic_shadow" x="-20%" y="-20%" width="150%" height="150%">
      <feGaussianBlur in="SourceAlpha" stdDeviation="14"/>
      <feOffset dx="0" dy="20" result="offsetblur"/>
      <feComponentTransfer>
        <feFuncA type="linear" slope="0.28"/>
      </feComponentTransfer>
      <feMerge>
        <feMergeNode/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>

    <linearGradient id="ic_pill_green" x1="375" y1="298" x2="649" y2="298" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#10b981"/>
      <stop offset="50%" stop-color="#34d399"/>
      <stop offset="100%" stop-color="#059669"/>
    </linearGradient>

    <linearGradient id="ic_pill_yellow" x1="375" y1="512" x2="649" y2="512" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#f59e0b"/>
      <stop offset="50%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#d97706"/>
    </linearGradient>
  </defs>

  <!-- Canvas-dark background with large rounded corners (macOS / Windows icon style) -->
  <rect width="1024" height="1024" rx="224" ry="224" fill="#010120"/>

  <!-- Circular logo background -->
  <circle cx="512" cy="512" r="380" fill="url(#ic_bg_grad)"/>
  <circle cx="512" cy="512" r="380" stroke="#94a3b8" stroke-width="6" opacity="0.45"/>

  <!-- Three arc strokes: red (top-left), amber (bottom-left), green (right 3/4) -->
  <g opacity="0.9">
    <path d="M512 132C284.9 132 99 317.9 99 512" stroke="#ef4444" stroke-width="96" stroke-linecap="round" fill="none"/>
    <path d="M99 512C99 706.1 284.9 892 512 892"  stroke="#f59e0b" stroke-width="96" stroke-linecap="round" fill="none"/>
    <path d="M512 892C739.1 892 925 706.1 925 512C925 317.9 739.1 132 512 132" stroke="#10b981" stroke-width="128" stroke-linecap="round" fill="none"/>
  </g>

  <!-- Pill capsule -->
  <g filter="url(#ic_shadow)">
    <rect x="375" y="298" width="274" height="428" rx="137" fill="url(#ic_pill_green)"/>
    <path d="M375 512H649V583C649 650.7 594.6 706 512 706C429.4 706 375 650.7 375 583V512Z" fill="url(#ic_pill_yellow)"/>

    <!-- Highlight gloss -->
    <rect x="418" y="318" width="52" height="128" rx="26" fill="white" opacity="0.38"/>
    <path d="M418 544Q418 640 512 640" stroke="white" stroke-width="20" stroke-linecap="round" opacity="0.28" fill="none"/>
  </g>
</svg>`;

// ─── Main ───
(async () => {
  try {
    log("🎨 Rendering SVG → icon.png (1024×1024)…");

    const resvg = new Resvg(ICON_SVG, {
      fitTo: { mode: "width", value: 1024 },
      imageRendering: 1,
      shapeRendering: 2,
      textRendering: 2,
    });

    const pngBuffer = resvg.render().asPng();

    fs.mkdirSync(ICONS_DIR, { recursive: true });
    fs.writeFileSync(SOURCE_PNG, pngBuffer);
    log(`   ✓ Saved ${path.basename(SOURCE_PNG)} (${Math.round(pngBuffer.length / 1024)} KB)`);

    log("\n📦 Running tauri icon generator…");
    execSync(`npm run tauri -- icon "${SOURCE_PNG}"`, {
      cwd: ROOT,
      stdio: IS_SILENT ? "pipe" : "inherit",
      timeout: 120_000,
    });

    log("\n✅ All icons generated successfully in src-tauri/icons/");
    log("   Rebuild the app to apply: npm run tauri -- build");

  } catch (err) {
    error("\n❌ Process failed:");
    error(err.message || err);
    process.exit(1);
  }
})();
