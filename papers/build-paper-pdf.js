/**
 * build-paper-pdf.js — Academic Paper PDF Generator
 *
 * Converts the genesis-protocol.html paper into a
 * journal-quality PDF using Puppeteer (headless Chromium).
 *
 * Features:
 *   - Single-column academic layout with professional typography
 *   - Page numbers (bottom center)
 *   - DOI visible on first page
 *   - Proper reference formatting
 *   - Hash tables with monospace rendering
 *
 * Usage: node papers/build-paper-pdf.js
 *
 * Prerequisites: npm install (puppeteer)
 */

const fs = require("fs");
const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const PAPER_HTML = path.join(__dirname, "genesis-protocol.html");
const DIST_DIR = path.join(ROOT, "dist");
const OUTPUT_PDF = path.join(DIST_DIR, "genesis-protocol.pdf");

// ——— Generate PDF ———————————————————————————
async function generatePDF() {
  console.log("[PAPER-PDF] Building academic PDF...\n");

  if (!fs.existsSync(DIST_DIR)) fs.mkdirSync(DIST_DIR, { recursive: true });

  // Step 1: Load HTML
  console.log("  [1/2] Loading genesis-protocol.html...");
  if (!fs.existsSync(PAPER_HTML)) {
    console.error(`  ERROR: ${PAPER_HTML} not found.`);
    console.error("  Run from project root: node papers/build-paper-pdf.js");
    process.exit(1);
  }
  const html = fs.readFileSync(PAPER_HTML, "utf-8");
  console.log(`  OK  HTML loaded (${(Buffer.byteLength(html) / 1024).toFixed(0)} KB)`);

  // Step 2: Puppeteer → PDF
  console.log("  [2/2] Puppeteer: HTML → PDF...");

  const puppeteer = require("puppeteer");
  const browser = await puppeteer.launch({
    headless: true,
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  try {
    const page = await browser.newPage();

    // Load HTML content directly
    await page.setContent(html, {
      waitUntil: "networkidle0",
      timeout: 30000,
    });

    // Wait for fonts
    await page.evaluateHandle("document.fonts.ready");

    // Generate PDF
    await page.pdf({
      path: OUTPUT_PDF,
      format: "Letter",
      margin: {
        top: "0.85in",
        bottom: "0.75in",
        left: "0.75in",
        right: "0.75in",
      },
      displayHeaderFooter: true,
      headerTemplate: `
        <div style="font-family: Georgia, serif; font-size: 7.5pt;
                    color: #999; width: 100%; padding: 0 0.75in;
                    box-sizing: border-box; text-align: center;">
          Burns (2026) &mdash; Genesis Protocol: Deterministic Artificial Life
        </div>
      `,
      footerTemplate: `
        <div style="font-family: Georgia, serif; font-size: 8pt;
                    color: #666; width: 100%; text-align: center;
                    padding: 0 0.75in; box-sizing: border-box;">
          <span class="pageNumber"></span>
        </div>
      `,
      printBackground: true,
      preferCSSPageSize: false,
    });

    const stats = fs.statSync(OUTPUT_PDF);
    const sizeKB = (stats.size / 1024).toFixed(0);
    const sizeMB = (stats.size / 1024 / 1024).toFixed(2);

    console.log(`\n[PAPER-PDF] Academic PDF generated successfully!`);
    console.log(`[PAPER-PDF]    Output: ${path.relative(ROOT, OUTPUT_PDF)}`);
    console.log(`[PAPER-PDF]    Size: ${sizeKB} KB (${sizeMB} MB)`);
    console.log(`[PAPER-PDF]    Format: US Letter (8.5" x 11")`);
    console.log(`[PAPER-PDF]    Layout: Single-column academic`);
    console.log(`[PAPER-PDF]    DOI: https://doi.org/10.5281/zenodo.18646886`);
    console.log(`\n[PAPER-PDF] Ready for SSRN, ResearchGate, and Zenodo upload.\n`);
  } catch (err) {
    console.error(`\n[PAPER-PDF] PDF generation failed:`);
    console.error(err.message);
    process.exit(1);
  } finally {
    await browser.close();
  }
}

generatePDF();
