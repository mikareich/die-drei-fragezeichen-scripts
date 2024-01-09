import { Command } from "commander";
import ScriptHandler from "./ScriptHandler";
import { version } from "../package.json";

const program = new Command();

program
  .version(version)
  .description(
    "A CLI tool to extract and analyze the scripts of the audio play 'Die drei ???' by tokens used for the GPT-3 model."
  )
  .option("--episode-count <number>", "The number of episodes to process")
  .option("--save-scripts", "Save the scripts as .txt files")
  .option("--save-csv", "Save the summary as a .csv file")
  .parse(process.argv);

const options = program.opts();

const scriptHandler = new ScriptHandler({
  saveCSV: options.saveCsv,
  saveScripts: options.saveScripts,
  numberOfEpisodes: Number(options.episodeCount),
});

scriptHandler.processScripts().finally(() => {
  process.stdout.write(`\rFinished processing scripts 🎉🎉`);
});
