use serde::Serialize;

// Everything in here is taken from https://microsoft.github.io/language-server-protocol/specification
// and edited for rust.

#[derive(Serialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(u64),
    String(String),
}

#[derive(Serialize)]
pub struct Request<Params> {
    pub id: RequestId,
    pub method: String,
    pub params: Option<Params>,
}

type DocumentUri = String;

#[derive(Default, Serialize)]
pub struct ClientInfo {
    /**
    	* The name of the client as defined by the client.
    	*/
    pub name: String,

    /**
    	* The client's version as defined by the client.
    	*/
    pub version: Option<String>,
}

// does not extend WorkDoneProgressParams
#[derive(Default, Serialize)]
pub struct InitializeParams {
    /**
     * The process Id of the parent process that started the server. Is null if
     * the process has not been started by another process. If the parent
     * process is not alive then the server should exit (see exit notification)
     * its process.
     */
    pub processId: Option<i64>,

    /**
     * Information about the client
     *
     * @since 3.15.0
     */
    pub clientInfo: Option<ClientInfo>,

    /**
     * The locale the client is currently showing the user interface
     * in. This must not necessarily be the locale of the operating
     * system.
     *
     * Uses IETF language tags as the value's syntax
     * (See https://en.wikipedia.org/wiki/IETF_language_tag)
     *
     * @since 3.16.0
     */
    pub locale: Option<String>,

    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     *
     * @deprecated in favour of `rootUri`.
     */
    pub rootPath: Option<String>,

    /**
     * The rootUri of the workspace. Is null if no
     * folder is open. If both `rootPath` and `rootUri` are set
     * `rootUri` wins.
     *
     * @deprecated in favour of `workspaceFolders`
     */
    pub rootUri: Option<DocumentUri>,

    // /**
    //  * User provided initialization options.
    //  */
    // initializationOptions?: any;
    /**
     * The capabilities provided by the client (editor or tool)
     */
    pub capabilities: ClientCapabilities,
    // /**
    //  * The initial trace setting. If omitted trace is disabled ('off').
    //  */
    // trace: Option<TraceValue>;

    // /**
    //  * The workspace folders configured in the client when the server starts.
    //  * This property is only available if the client supports workspace folders.
    //  * It can be `null` if the client supports workspace folders but none are
    //  * configured.
    //  *
    //  * @since 3.6.0
    //  */
    // workspaceFolders: Option<WorkspaceFolder>;
}

/**
 * Text document specific client capabilities.
 */
#[derive(Default, Serialize)]
pub struct TextDocumentClientCapabilities {
    // synchronization: Option<TextDocumentSyncClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/completion` request.
//  */
// completion: Option<CompletionClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/hover` request.
//  */
// hover: Option<HoverClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/signatureHelp` request.
//  */
// signatureHelp: Option<SignatureHelpClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/declaration` request.
//  *
//  * @since 3.14.0
//  */
// declaration: Option<DeclarationClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/definition` request.
//  */
// definition: Option<DefinitionClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/typeDefinition` request.
//  *
//  * @since 3.6.0
//  */
// typeDefinition: Option<TypeDefinitionClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/implementation` request.
//  *
//  * @since 3.6.0
//  */
// implementation: Option<ImplementationClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/references` request.
//  */
// references: Option<ReferenceClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/documentHighlight` request.
//  */
// documentHighlight: Option<DocumentHighlightClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/documentSymbol` request.
//  */
// documentSymbol: Option<DocumentSymbolClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/codeAction` request.
//  */
// codeAction: Option<CodeActionClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/codeLens` request.
//  */
// codeLens: Option<CodeLensClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/documentLink` request.
//  */
// documentLink: Option<DocumentLinkClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/documentColor` and the
//  * `textDocument/colorPresentation` request.
//  *
//  * @since 3.6.0
//  */
// colorProvider: Option<DocumentColorClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/formatting` request.
//  */
// formatting: Option<DocumentFormattingClientCapabilities>

// /**
//  * Capabilities specific to the `textDocument/rangeFormatting` request.
//  */
// rangeFormatting: Option<DocumentRangeFormattingClientCapabilities>;

// /** request.
//  * Capabilities specific to the `textDocument/onTypeFormatting` request.
//  */
// onTypeFormatting: Option<DocumentOnTypeFormattingClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/rename` request.
//  */
// rename: Option<RenameClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/publishDiagnostics`
//  * notification.
//  */
// publishDiagnostics: Option<PublishDiagnosticsClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/foldingRange` request.
//  *
//  * @since 3.10.0
//  */
// foldingRange: Option<FoldingRangeClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/selectionRange` request.
//  *
//  * @since 3.15.0
//  */
// selectionRange: Option<SelectionRangeClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/linkedEditingRange` request.
//  *
//  * @since 3.16.0
//  */
// linkedEditingRange: Option<LinkedEditingRangeClientCapabilities>;

// /**
//  * Capabilities specific to the various call hierarchy requests.
//  *
//  * @since 3.16.0
//  */
// callHierarchy: Option<CallHierarchyClientCapabilities>;

// /**
//  * Capabilities specific to the various semantic token requests.
//  *
//  * @since 3.16.0
//  */
// semanticTokens: Option<SemanticTokensClientCapabilities>;

// /**
//  * Capabilities specific to the `textDocument/moniker` request.
//  *
//  * @since 3.16.0
//  */
// moniker: Option<MonikerClientCapabilities>;
}

#[derive(Default, Serialize)]
pub struct ClientCapabilities {
    // /**
//  * Workspace specific client capabilities.
//  */
// // workspace?: {
// 	/**
// 	 * The client supports applying batch edits
// 	 * to the workspace by supporting the request
// 	 * 'workspace/applyEdit'
// 	 */
// 	// applyEdit: Option<boolean>;

// 	/**
// 	 * Capabilities specific to `WorkspaceEdit`s
// 	 */
// 	// workspaceEdit: Option<WorkspaceEditClientCapabilities>;

// 	/**
// 	 * Capabilities specific to the `workspace/didChangeConfiguration`
// 	 * notification.
// 	 */
// 	// didChangeConfiguration: Option<DidChangeConfigurationClientCapabilities>;

// 	/**
// 	 * Capabilities specific to the `workspace/didChangeWatchedFiles`
// 	 * notification.
// 	 */
// 	// didChangeWatchedFiles: Option<DidChangeWatchedFilesClientCapabilities>;

// 	/**
// 	 * Capabilities specific to the `workspace/symbol` request.
// 	 */
// 	// symbol: Option<WorkspaceSymbolClientCapabilities>;

// 	/**
// 	 * Capabilities specific to the `workspace/executeCommand` request.
// 	 */
// 	// executeCommand: Option<ExecuteCommandClientCapabilities>;

// 	/**
// 	 * The client has support for workspace folders.
// 	 *
// 	 * @since 3.6.0
// 	 */
// 	// workspaceFolders: Option<boolean>;

// 	/**
// 	 * The client supports `workspace/configuration` requests.
// 	 *
// 	 * @since 3.6.0
// 	 */
// 	// configuration: Option<boolean>;

// 	/**
// 	 * Capabilities specific to the semantic token requests scoped to the
// 	 * workspace.
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	//  semanticTokens: Option<SemanticTokensWorkspaceClientCapabilities>;

// 	/**
// 	 * Capabilities specific to the code lens requests scoped to the
// 	 * workspace.
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	// codeLens: Option<CodeLensWorkspaceClientCapabilities>;

// 	/**
// 	 * The client has support for file requests/notifications.
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	// fileOperations: Option<{>
// 		/**
// 		 * Whether the client supports dynamic registration for file
// 		 * requests/notifications.
// 		 */
// 		// dynamicRegistration: Option<boolean>;

// 		/**
// 		 * The client has support for sending didCreateFiles notifications.
// 		 */
// 		// didCreate: Option<boolean>;

// 		/**
// 		 * The client has support for sending willCreateFiles requests.
// 		 */
// 		// willCreate: Option<boolean>;

// 		/**
// 		 * The client has support for sending didRenameFiles notifications.
// 		 */
// 		// didRename: Option<boolean>;

// 		/**
// 		 * The client has support for sending willRenameFiles requests.
// 		 */
// 		// willRename: Option<boolean>;

// 		/**
// 		 * The client has support for sending didDeleteFiles notifications.
// 		 */
// 		// didDelete: Option<boolean>;

// 		/**
// 		 * The client has support for sending willDeleteFiles requests.
// 		 */
// 		// willDelete: Option<boolean>;
// 	// }
// // };

// /**
//  * Text document specific client capabilities.
//  */
// // textDocument: Option<TextDocumentClientCapabilities>;

// /**
//  * Window specific client capabilities.
//  */
// // window: Option<{
// 	/**
// 	 * Whether client supports handling progress notifications. If set
// 	 * servers are allowed to report in `workDoneProgress` property in the
// 	 * request specific server capabilities.
// 	 *
// 	 * @since 3.15.0
// 	 */
// 	// workDoneProgress: Option<boolean>;

// 	/**
// 	 * Capabilities specific to the showMessage request
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	// showMessage: Option<ShowMessageRequestClientCapabilities>;

// 	/**
// 	 * Client capabilities for the show document request.
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	// showDocument: Option<ShowDocumentClientCapabilities>;
// // }>

// /**
//  * General client capabilities.
//  *
//  * @since 3.16.0
//  */
// // general: Option<{
// 	/**
// 	 * Client capabilities specific to regular expressions.
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	// regularExpressions: Option<RegularExpressionsClientCapabilities>;

// 	/**
// 	 * Client capabilities specific to the client's markdown parser.
// 	 *
// 	 * @since 3.16.0
// 	 */
// 	// markdown: Option<MarkdownClientCapabilities>;
// // }>

// /**
//  * Experimental client capabilities.
//  */
// // experimental: Option<any>;
}
