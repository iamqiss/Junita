import * as path from 'path';
import * as os from 'os';
import { workspace, ExtensionContext, window, commands, StatusBarAlignment } from 'vscode';
import {
LanguageClient,
LanguageClientOptions,
ServerOptions,
TransportKind,
NotificationType
} from 'vscode-languageclient/node';

let client: LanguageClient;
let hotReloadEnabled = false;
let statusBarItem: any;
let outputChannel: any;

export async function activate(context: ExtensionContext) {
console.log('🎉 Junita DSL Extension activated with LSP support');

// Determine the LSP server path
const platform = os.platform();
const isWindows = platform === 'win32';

// Try to find the compiled LSP binary
let serverPath = path.join(
context.extensionPath,
'..',
'..',
'junita-analyzer',
'target',
'release',
isWindows ? 'junita-lsp.exe' : 'junita-lsp'
);

// Server options
const serverOptions: ServerOptions = {
run: {
command: serverPath,
transport: TransportKind.stdio
},
debug: {
command: serverPath,
transport: TransportKind.stdio
}
};

// Client options
const clientOptions: LanguageClientOptions = {
documentSelector: [
{ scheme: 'file', language: 'junita' },
{ scheme: 'file', pattern: '**/*.junita' },
{ scheme: 'file', pattern: '**/*.bl' }
],
synchronize: {
fileEvents: workspace.createFileSystemWatcher('**/*.{junita,bl}')
},
progressOnInitialization: true
};

// Create the language client
client = new LanguageClient(
'junitaLsp',
'Junita Language Server',
serverOptions,
clientOptions
);

try {
console.log('🚀 Starting Junita LSP client...');
await client.start();
window.showInformationMessage('✅ Junita LSP server connected! Enjoy coding with Junita!');
console.log('✅ Junita LSP client started successfully');

// Initialize output channel for hot reload feedback
outputChannel = window.createOutputChannel('Junita Hot Reload');
outputChannel.appendLine('Hot reload channel ready. Use Ctrl+Shift+P and search "Junita Hot Reload" to start.');

// Create status bar item
statusBarItem = window.createStatusBarItem(StatusBarAlignment.Right, 100);
statusBarItem.command = 'junita.toggleHotReload';
updateStatusBar();
statusBarItem.show();

// Register hot reload commands
commands.registerCommand('junita.startHotReload', async () => {
  try {
    const result = await client.sendRequest('junita/startHotReload', {});
    hotReloadEnabled = true;
    updateStatusBar();
    outputChannel.appendLine('✅ Hot reload started');
    window.showInformationMessage('🔴 Hot reload enabled - Changes will auto-compile!');
  } catch (error) {
    window.showErrorMessage(`Failed to start hot reload: ${error}`);
    outputChannel.appendLine(`❌ Error: ${error}`);
  }
});

commands.registerCommand('junita.stopHotReload', async () => {
  try {
    const result = await client.sendRequest('junita/stopHotReload', {});
    hotReloadEnabled = false;
    updateStatusBar();
    outputChannel.appendLine('⏹️ Hot reload stopped');
    window.showInformationMessage('⚪ Hot reload disabled');
  } catch (error) {
    window.showErrorMessage(`Failed to stop hot reload: ${error}`);
    outputChannel.appendLine(`❌ Error: ${error}`);
  }
});

commands.registerCommand('junita.toggleHotReload', async () => {
  if (hotReloadEnabled) {
    commands.executeCommand('junita.stopHotReload');
  } else {
    commands.executeCommand('junita.startHotReload');
  }
});

// Listen for hot reload update notifications from the server
client.onNotification('junita/hotReloadUpdate', (data: any) => {
  outputChannel.show();
  outputChannel.appendLine(`\n[${new Date().toLocaleTimeString()}] ${data.message}`);
  outputChannel.appendLine(`  📦 Widgets: ${data.widget_count} | Machines: ${data.machine_count} | Animations: ${data.animation_count}`);
  
  if (data.action === 'compiled') {
    window.showInformationMessage('✅ Hot reload compiled successfully', { modal: false });
  } else if (data.action === 'compile_error') {
    window.showErrorMessage('❌ Hot reload compilation failed - check output');
  }
});

} catch (error) {
window.showErrorMessage(`Failed to start Junita LSP: ${error}`);
console.error('Failed to start Junita LSP client:', error);
}

context.subscriptions.push(client);
context.subscriptions.push(statusBarItem);
context.subscriptions.push(outputChannel);
}

function updateStatusBar() {
  if (hotReloadEnabled) {
    statusBarItem.text = '🔴 Hot Reload ON';
    statusBarItem.backgroundColor = '';
  } else {
    statusBarItem.text = '⚪ Hot Reload OFF';
    statusBarItem.backgroundColor = '';
  }
}

export function deactivate(): Thenable<void> | undefined {
if (!client) {
return undefined;
}
return client.stop();
}
