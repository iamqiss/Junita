"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = __importStar(require("path"));
const os = __importStar(require("os"));
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
async function activate(context) {
    console.log('🎉 Junita DSL Extension activated with LSP support');
    // Determine the LSP server path
    const platform = os.platform();
    const isWindows = platform === 'win32';
    // Try to find the compiled LSP binary
    let serverPath = path.join(context.extensionPath, '..', '..', 'junita-analyzer', 'target', 'release', isWindows ? 'junita-lsp.exe' : 'junita-lsp');
    // Server options
    const serverOptions = {
        run: {
            command: serverPath,
            transport: node_1.TransportKind.stdio
        },
        debug: {
            command: serverPath,
            transport: node_1.TransportKind.stdio
        }
    };
    // Client options
    const clientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'junita' },
            { scheme: 'file', pattern: '**/*.junita' },
            { scheme: 'file', pattern: '**/*.bl' }
        ],
        synchronize: {
            fileEvents: vscode_1.workspace.createFileSystemWatcher('**/*.{junita,bl}')
        },
        progressOnInitialization: true
    };
    // Create the language client
    client = new node_1.LanguageClient('junitaLsp', 'Junita Language Server', serverOptions, clientOptions);
    try {
        console.log('🚀 Starting Junita LSP client...');
        await client.start();
        vscode_1.window.showInformationMessage('✅ Junita LSP server connected! Enjoy coding with Junita!');
        console.log('✅ Junita LSP client started successfully');
    }
    catch (error) {
        vscode_1.window.showErrorMessage(`Failed to start Junita LSP: ${error}`);
        console.error('Failed to start Junita LSP client:', error);
    }
    context.subscriptions.push(client);
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map