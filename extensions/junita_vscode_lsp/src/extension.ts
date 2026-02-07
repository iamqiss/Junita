import * as path from 'path';
import * as os from 'os';
import { workspace, ExtensionContext, window } from 'vscode';
import {
LanguageClient,
LanguageClientOptions,
ServerOptions,
TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

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
} catch (error) {
window.showErrorMessage(`Failed to start Junita LSP: ${error}`);
console.error('Failed to start Junita LSP client:', error);
}

context.subscriptions.push(client);
}

export function deactivate(): Thenable<void> | undefined {
if (!client) {
return undefined;
}
return client.stop();
}
