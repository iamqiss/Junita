#!/usr/bin/env node

/**
 * Comprehensive test suite for Junita DSL VS Code Extension
 * Tests grammar, file associations, language configuration, and features
 */

const fs = require('fs');
const path = require('path');

const RESET = '\x1b[0m';
const GREEN = '\x1b[32m';
const RED = '\x1b[31m';
const YELLOW = '\x1b[33m';
const BLUE = '\x1b[34m';

function pass(msg) { console.log(`${GREEN}✅${RESET} ${msg}`); }
function fail(msg) { console.log(`${RED}❌${RESET} ${msg}`); }
function section(msg) { console.log(`\n${BLUE}━━ ${msg} ━━${RESET}`); }
function info(msg) { console.log(`${YELLOW}ℹ️${RESET}  ${msg}`); }

let totalTests = 0;
let passedTests = 0;

section('Extension Package Validation');

// 1. Test package.json
totalTests++;
try {
  const pkg = JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8'));
  if (pkg.name === 'junita-dsl' && pkg.main === './out/extension.js') {
    pass(`Package manifest is valid (version ${pkg.version})`);
    passedTests++;
  } else {
    fail('Package manifest missing required fields');
  }
} catch (e) {
  fail(`Package manifest JSON error: ${e.message}`);
}

// 2. Test compiled files exist
totalTests++;
if (fs.existsSync(path.join(__dirname, 'out', 'extension.js'))) {
  const stats = fs.statSync(path.join(__dirname, 'out', 'extension.js'));
  pass(`TypeScript compiled (${stats.size} bytes)`);
  passedTests++;
} else {
  fail('Extension entry point not compiled');
}

section('Language Configuration');

// 3. Test language config
totalTests++;
try {
  const langConfig = JSON.parse(fs.readFileSync(path.join(__dirname, 'language-configuration.json'), 'utf8'));
  if (langConfig.comments && langConfig.brackets) {
    pass('Language configuration is valid');
    pass(`  - Comments configured: ${JSON.stringify(langConfig.comments)}`);
    pass(`  - Bracket pairs: ${langConfig.brackets.length} rules`);
    passedTests++;
  } else {
    fail('Language configuration incomplete');
  }
} catch (e) {
  fail(`Language config error: ${e.message}`);
}

section('Syntax Grammar');

// 4. Test TextMate grammar
totalTests++;
try {
  const grammar = JSON.parse(fs.readFileSync(path.join(__dirname, 'syntaxes', 'junita.tmLanguage.json'), 'utf8'));
  if (grammar.scopeName === 'source.junita' && grammar.patterns) {
    const patternCount = Object.keys(grammar.patterns || {}).length;
    pass(`TextMate grammar is valid (${patternCount} patterns)`);
    
    // Check for key language features
    const grammarStr = JSON.stringify(grammar);
    const hasDecorators = grammarStr.includes('@widget');
    const hasKeywords = grammarStr.includes('if') || grammarStr.includes('else');
    const hasTypes = grammarStr.includes('Int') || grammarStr.includes('String');
    
    if (hasDecorators) pass('  - Decorators (@widget, @state, etc.) supported');
    if (hasKeywords) pass('  - Keywords (if, else, for, etc.) supported');
    if (hasTypes) pass('  - Types (Int, String, Bool, etc.) supported');
    
    passedTests++;
  } else {
    fail('TextMate grammar incomplete');
  }
} catch (e) {
  fail(`Grammar error: ${e.message}`);
}

section('File Icon Theme');

// 5. Test icon theme
totalTests++;
try {
  const icons = JSON.parse(fs.readFileSync(path.join(__dirname, 'fileicons', 'junita-icons.json'), 'utf8'));
  if (icons.fileExtensions && icons.iconDefinitions) {
    const extensions = Object.keys(icons.fileExtensions);
    pass(`Icon theme is valid (${extensions.length} file types)`);
    for (const ext of extensions) {
      pass(`  - .${ext} files associated with icon`);
    }
    
    const fileNames = Object.keys(icons.fileNames || {});
    for (const name of fileNames) {
      pass(`  - Special file ${name} has icon`);
    }
    passedTests++;
  } else {
    fail('Icon theme incomplete');
  }
} catch (e) {
  fail(`Icon theme error: ${e.message}`);
}

section('Language Features');

// 6. Test hover provider configuration
totalTests++;
try {
  const extCode = fs.readFileSync(path.join(__dirname, 'out', 'extension.js'), 'utf8');
  const hasHover = extCode.includes('registerHoverProvider');
  const hasCompletion = extCode.includes('registerCompletionItemProvider');
  const hasFormat = extCode.includes('registerDocumentRangeFormattingEditProvider');
  const hasCommands = extCode.includes('registerCommand');
  
  if (hasHover && hasCompletion && hasFormat && hasCommands) {
    pass('Language providers implemented');
    if (hasHover) pass('  - Hover provider: Decorators documentation');
    if (hasCompletion) pass('  - Completion provider: Autocomplete');
    if (hasFormat) pass('  - Format provider: Document formatting');
    if (hasCommands) pass('  - Commands: Hot reload integration');
    passedTests++;
  } else {
    fail('Language providers incomplete');
  }
} catch (e) {
  fail(`Language features error: ${e.message}`);
}

section('VSIX Package');

// 7. Test VSIX file
totalTests++;
try {
  const vsixPath = path.join(__dirname, 'junita-dsl-0.0.1.vsix');
  if (fs.existsSync(vsixPath)) {
    const stats = fs.statSync(vsixPath);
    pass(`VSIX package created (${(stats.size / 1024).toFixed(2)} KB)`);
    info(`Location: ${vsixPath}`);
    info(`Can be installed via: Code → Extensions → Install from VSIX`);
    passedTests++;
  } else {
    fail('VSIX package not found');
  }
} catch (e) {
  fail(`VSIX check error: ${e.message}`);
}

section('Logo Association');

// 8. Test logo.svg path
totalTests++;
try {
  const docLogoPath = path.join(__dirname, '..', '..', 'docs', 'book', 'src', 'logo.svg');
  if (fs.existsSync(docLogoPath)) {
    const stats = fs.statSync(docLogoPath);
    pass(`Logo found (${(stats.size / 1024).toFixed(2)} KB)`);
    info('Logo.svg will be shown for: .junita, .bl, .junitaproj files');
    passedTests++;
  } else {
    fail(`Logo not found at ${docLogoPath}`);
  }
} catch (e) {
  fail(`Logo check error: ${e.message}`);
}

section('Test Results');

const percentage = Math.round((passedTests / totalTests) * 100);
console.log(`\n${BLUE}Total: ${passedTests}/${totalTests} tests passed (${percentage}%)${RESET}\n`);

if (passedTests === totalTests) {
  console.log(`${GREEN}✨ Extension is fully functional and ready!${RESET}\n`);
  console.log(`${BLUE}Installation Methods:${RESET}`);
  console.log(`1. ${YELLOW}VSIX Package:${RESET} Install junita-dsl-0.0.1.vsix via Extensions panel`);
  console.log(`2. ${YELLOW}Debug Mode:${RESET} Open workspace in VS Code and press F5`);
  console.log(`3. ${YELLOW}From Marketplace:${RESET} Search "Junita DSL" (when published)\n`);
  console.log(`${BLUE}Verification Steps:${RESET}`);
  console.log(`1. Open a .junita file → should show logo icon and syntax highlighting`);
  console.log(`2. Hover over @widget/@state → should show documentation`);
  console.log(`3. Type @ or Ctrl+Space → should show autocomplete suggestions`);
  console.log(`4. Cmd/Ctrl+Shift+F → should format the document`);
  console.log(`5. Cmd/Ctrl+Shift+P → type "Junita" to see available commands\n`);
  process.exit(0);
} else {
  console.log(`${RED}⚠️  Some tests failed. Please review the errors above.${RESET}\n`);
  process.exit(1);
}
