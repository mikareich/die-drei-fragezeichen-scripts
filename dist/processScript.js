"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const pdf_js_extract_1 = require("pdf.js-extract");
const tiktoken_1 = require("tiktoken");
const pdfExtract = new pdf_js_extract_1.PDFExtract();
const tokenEncoder = (0, tiktoken_1.encoding_for_model)("gpt-3.5-turbo");
const SKRIPT_URL = "https://www.rocky-beach.com/hoerspiel/skript/"; // +skript_xxx.pdf
function processScript(episodeFormatted) {
    return __awaiter(this, void 0, void 0, function* () {
        let data;
        try {
            // fetch skript
            const res = yield fetch(`${SKRIPT_URL}skript_${episodeFormatted}.pdf`);
            const buffer = (yield res.arrayBuffer());
            // extract pdf
            data = yield pdfExtract.extractBuffer(buffer, {
                normalizeWhitespace: true,
                disableCombineTextItems: false,
            });
        }
        catch (error) {
            throw new Error(`Error while processing episode ${episodeFormatted}`);
        }
        // extract content from pdf
        const content = data.pages
            .map((page) => page.content.map((c) => c.str))
            .join(" ");
        // encode content
        const numberOfTokens = tokenEncoder.encode(content).length;
        return {
            episodeFormatted,
            numberOfTokens,
            content,
        };
    });
}
exports.default = processScript;
//# sourceMappingURL=processScript.js.map