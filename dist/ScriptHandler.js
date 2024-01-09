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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const path_1 = __importDefault(require("path"));
const promises_1 = __importDefault(require("fs/promises"));
const processScript_1 = __importDefault(require("./processScript"));
class ScriptHandler {
    get progress() {
        return this.scripts.length / this.numberOfEpisodes;
    }
    constructor({ saveCSV = false, saveScripts = false, numberOfEpisodes = 230, }) {
        this.scripts = [];
        this.numberOfEpisodes = 0;
        this.isProcessing = false;
        this.saveCSV = saveCSV;
        this.saveScripts = saveScripts;
        this.numberOfEpisodes = numberOfEpisodes;
    }
    logProgress() {
        const lastScript = this.scripts[this.scripts.length - 1];
        const statusText = lastScript.failed
            ? `❌ Episode ${lastScript.episodeFormatted}/${this.numberOfEpisodes} failed to process`
            : `✅ Episode ${lastScript.episodeFormatted}/${this.numberOfEpisodes} successfully processed`;
        process.stdout.clearLine(0);
        process.stdout.write(`\r${statusText} [${Math.round(this.progress * 100)}%]`);
    }
    processScripts() {
        return __awaiter(this, void 0, void 0, function* () {
            if (this.isProcessing)
                return;
            this.isProcessing = true;
            for (let episode = 1; episode <= this.numberOfEpisodes; episode++) {
                const episodeFormatted = episode.toString().padStart(3, "0");
                let processed = {
                    failed: true,
                    episode,
                    episodeFormatted,
                    numberOfTokens: null,
                    content: null,
                };
                try {
                    const data = yield (0, processScript_1.default)(episodeFormatted);
                    processed = Object.assign(Object.assign(Object.assign({}, processed), data), { failed: false });
                }
                catch (error) { }
                this.scripts.push(processed);
                this.logProgress();
            }
            if (this.saveCSV) {
                yield this.saveCSVFile();
            }
            if (this.saveScripts) {
                yield this.saveAllScripts();
            }
        });
    }
    prepareScriptFolder() {
        return __awaiter(this, void 0, void 0, function* () {
            yield promises_1.default.mkdir(ScriptHandler.SCRIPT_FOLDER_PATH, { recursive: true });
        });
    }
    saveAllScripts() {
        return __awaiter(this, void 0, void 0, function* () {
            process.stdout.write("\rSaving scripts");
            yield this.prepareScriptFolder();
            for (const script of this.scripts.filter((s) => !s.failed)) {
                yield promises_1.default.writeFile(path_1.default.join(ScriptHandler.SCRIPT_FOLDER_PATH, `${script.episodeFormatted}.txt`), script.content, "utf8");
            }
        });
    }
    saveCSVFile() {
        return __awaiter(this, void 0, void 0, function* () {
            process.stdout.write("\rSaving CSV file");
            yield this.prepareScriptFolder();
            const headerMap = {
                episode: "Episode",
                numberOfTokens: "Number of Tokens",
            };
            const rows = this.scripts
                .sort((a, b) => a.episode - b.episode)
                .map((script) => Object.keys(headerMap)
                .map((key) => { var _a; return (_a = script[key]) !== null && _a !== void 0 ? _a : ""; })
                .join(","));
            const headerAsString = Object.values(headerMap).join(",") + "\n";
            yield promises_1.default.writeFile(ScriptHandler.CSV_FILE_PATH, headerAsString + rows.join("\n"), "utf8");
        });
    }
}
ScriptHandler.SCRIPT_FOLDER_PATH = path_1.default.join(__dirname, "..", "scripts");
ScriptHandler.CSV_FILE_PATH = path_1.default.join(ScriptHandler.SCRIPT_FOLDER_PATH, "data.csv");
exports.default = ScriptHandler;
//# sourceMappingURL=ScriptHandler.js.map