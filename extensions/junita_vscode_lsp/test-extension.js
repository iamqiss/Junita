#!/usr/bin/env node

/**
 * Test script to verify the Junita DSL extension loads correctly
 */

const fs = require('fs');
const path = require('path');

console.log('🧪 Testing Junita DSL Extension...\n');

const checks = [
  {
    name: 'Extension entry point exists',
    check: () => fs.existsSync(path.join(__dirname, 'out', 'extension.js'))
  },
  {
    name: 'TypeScript compiled successfully',
    check: () => fs.existsSync(path.join(__dirname, 'out', 'extension.d.ts'))
  },
  {
    name: 'Language configuration exists',
    check: () => fs.existsSync(path.join(__dirname, 'language-configuration.json'))
  },
  {
    name: 'TextMate grammar exists',
    check: () => fs.existsSync(path.join(__dirname, 'syntaxes', 'junita.tmLanguage.json'))
  },
  {
    name: 'File icon theme exists',
    check: () => fs.existsSync(path.join(__dirname, 'fileicons', 'junita-icons.json'))
  },
  {
    name: 'Package manifest is valid JSON',
    check: () => {
      try {
        const pkg = JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8'));
        return pkg.name === 'junita-dsl' && pkg.main === './out/extension.js';
      } catch {
        return false;
      }
    }
  },
  {
    name: 'Extension entry point can be loaded',
    check: () => {
      try {
        // Try to require the extension module
        require(path.join(__dirname, 'out', 'extension.js'));
        return true;
      } catch (e) {
        console.error('  Error loading extension:', e.message);
        return false;
      }
    }
  },
  {
    name: 'Grammar file is valid JSON',
    check: () => {
      try {
        JSON.parse(fs.readFileSync(path.join(__dirname, 'syntaxes', 'junita.tmLanguage.json'), 'utf8'));
        return true;
      } catch {
        return false;
      }
    }
  },
  {
    name: 'Icon theme file is valid JSON',
    check: () => {
      try {
        JSON.parse(fs.readFileSync(path.join(__dirname, 'fileicons', 'junita-icons.json'), 'utf8'));
        return true;
      } catch {
        return false;
      }
    }
  }
];

let passed = 0;
let failed = 0;

for (const test of checks) {
  try {
    const result = test.check();
    if (result) {
      console.log(`✅ ${test.name}`);
      passed++;
    } else {
      console.log(`❌ ${test.name}`);
      failed++;
    }
  } catch (e) {
    console.log(`❌ ${test.name} - ${e.message}`);
    failed++;
  }
}

console.log(`\n📊 Results: ${passed} passed, ${failed} failed\n`);

if (failed === 0) {
  console.log('✨ Extension is ready for installation!\n');
  console.log('Installation options:');
  console.log('1. Package as VSIX: npm run vscode:prepublish && npx vsce package');
  console.log('2. Debug mode: Open in VS Code and press F5');
  console.log('3. Install from VSIX: Code → Extensions → Install from VSIX');
  process.exit(0);
} else {
  console.log('⚠️  Fix the errors above before testing in VS Code');
  process.exit(1);
}
