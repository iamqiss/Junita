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
const vscode = __importStar(require("vscode"));
function activate(context) {
    console.log('Junita DSL Extension activated');
    // Command: Connect to hot reload server
    let connectCommand = vscode.commands.registerCommand('junita.connectHotReload', async () => {
        const port = await vscode.window.showInputBox({
            prompt: 'Enter hot reload server port',
            value: '8080',
            placeHolder: '8080'
        });
        if (port) {
            vscode.window.showInformationMessage(`Connecting to hot reload server on port ${port}...`);
            // TODO: Implement WebSocket connection to hot reload server
        }
    });
    context.subscriptions.push(connectCommand);
    // Command: Format document
    let formatCommand = vscode.commands.registerCommand('junita.formatDocument', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor)
            return;
        const doc = editor.document;
        const text = doc.getText();
        // Simple formatting: sort declarations
        const formatted = formatJunitaCode(text);
        const fullRange = new vscode.Range(doc.lineAt(0).range.start, doc.lineAt(doc.lineCount - 1).range.end);
        await editor.edit(editBuilder => {
            editBuilder.replace(fullRange, formatted);
        });
        vscode.window.showInformationMessage('Junita document formatted');
    });
    context.subscriptions.push(formatCommand);
    // Hover provider for documentation
    let hoverProvider = vscode.languages.registerHoverProvider('junita', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            if (!range)
                return null;
            const word = document.getText(range);
            // Provide hover information for decorators
            const decorators = {
                '@widget': 'Defines a reusable widget component',
                '@state': 'Declares a reactive state variable',
                '@prop': 'Declares a widget property',
                '@derived': 'Declares a derived/computed value',
                '@machine': 'Declares a state machine (FSM)',
                '@animation': 'Declares a keyframe animation',
                '@spring': 'Declares a spring physics animation',
                '@render': 'Defines the render body for layout',
                '@paint': 'Defines custom painting code'
            };
            if (word in decorators) {
                return new vscode.Hover(new vscode.MarkdownString(`**${word}**\n\n${decorators[word]}`));
            }
            return null;
        }
    });
    context.subscriptions.push(hoverProvider);
    // Complettion provider for autocomplete
    let completionProvider = vscode.languages.registerCompletionItemProvider('junita', {
        provideCompletionItems(document, position, token) {
            const completions = [];
            // Decorators
            const decorators = [
                { label: '@widget', detail: 'Define a widget' },
                { label: '@state', detail: 'Declare state variable' },
                { label: '@prop', detail: 'Declare property' },
                { label: '@derived', detail: 'Declare derived value' },
                { label: '@machine', detail: 'Declare state machine' },
                { label: '@animation', detail: 'Declare animation' },
                { label: '@spring', detail: 'Declare spring' },
                { label: '@render', detail: 'Define render body' },
                { label: '@paint', detail: 'Define paint body' }
            ];
            for (const decorator of decorators) {
                const item = new vscode.CompletionItem(decorator.label, vscode.CompletionItemKind.Keyword);
                item.detail = decorator.detail;
                completions.push(item);
            }
            // Types
            const types = [
                'Int', 'Float', 'Bool', 'String', 'color',
                'Vec', 'HashMap', 'Option', 'Result'
            ];
            for (const type of types) {
                const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.Class);
                completions.push(item);
            }
            return completions;
        }
    });
    context.subscriptions.push(completionProvider);
    // Simple formatter
    vscode.languages.registerDocumentRangeFormattingEditProvider('junita', {
        provideDocumentRangeFormattingEdits(document, range, options, token) {
            const edits = [];
            // Indent @decorators properly
            for (let i = range.start.line; i <= range.end.line; i++) {
                const line = document.lineAt(i);
                const text = line.text;
                if (text.trim().startsWith('@')) {
                    // @decorators should be at indent level 2 spaces
                    const match = text.match(/^(\s*)@/);
                    if (match && match[1].length !== 2) {
                        edits.push(vscode.TextEdit.replace(new vscode.Range(i, 0, i, match[1].length), '  '));
                    }
                }
            }
            return edits;
        }
    });
    vscode.window.showInformationMessage('Junita DSL support activated! Use Ctrl+Shift+P to format documents.');
}
function formatJunitaCode(code) {
    const lines = code.split('\n');
    const sorted = [];
    let currentWidget = '';
    let currentBody = [];
    let inBody = false;
    let braceDepth = 0;
    for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith('@widget')) {
            if (currentWidget) {
                sorted.push(currentWidget);
                sorted.push(...currentBody);
            }
            currentWidget = line;
            inBody = true;
            braceDepth = 0;
        }
        else if (inBody) {
            currentBody.push(line);
            braceDepth += (line.match(/{/g) || []).length;
            braceDepth -= (line.match(/}/g) || []).length;
            if (braceDepth === 0 && trimmed === '}') {
                inBody = false;
            }
        }
        else {
            sorted.push(line);
        }
    }
    if (currentWidget) {
        sorted.push(currentWidget);
        sorted.push(...currentBody);
    }
    return sorted.join('\n');
}
function deactivate() {
    console.log('Junita LSP Extension deactivated');
}
//# sourceMappingURL=extension.js.map