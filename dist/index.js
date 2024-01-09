"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const commander_1 = require("commander");
const ScriptHandler_1 = __importDefault(require("./ScriptHandler"));
const program = new commander_1.Command();
program
    .version("0.0.1")
    .description("A CLI tool to analyze the number of tokens used in the Die Drei Fragezeichen Hörspiel-Skript's")
    .option("--episode-count <number>", "The number of episodes to process")
    .option("--save-scripts", "Save the scripts as txt files")
    .option("--save-csv", "Save the summary as a csv file")
    .parse(process.argv);
const options = program.opts();
const scriptHandler = new ScriptHandler_1.default({
    saveCSV: options.saveCsv,
    saveScripts: options.saveScripts,
    numberOfEpisodes: Number(options.episodeCount),
});
scriptHandler.processScripts().finally(() => {
    process.stdout.clearLine(0);
    process.stdout.write(`\rFinished processing scripts 🎉🎉`);
});
//# sourceMappingURL=index.js.map