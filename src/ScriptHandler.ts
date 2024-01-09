import path from "path";
import fs from "fs/promises";
import processScript from "./processScript";

type Script = {
  failed: boolean;
  episode: number;
  episodeFormatted: string;
  numberOfTokens: number;
  content: string;
};

class ScriptHandler {
  private static SCRIPT_FOLDER_PATH = path.join(__dirname, "..", "scripts");
  private static CSV_FILE_PATH = path.join(
    ScriptHandler.SCRIPT_FOLDER_PATH,
    "data.csv"
  );

  private readonly scripts: Script[] = [];
  private readonly saveCSV: boolean;
  private readonly saveScripts: boolean;
  private readonly numberOfEpisodes: number = 0;
  private isProcessing: boolean = false;

  get progress() {
    return this.scripts.length / this.numberOfEpisodes;
  }

  constructor({
    saveCSV = false,
    saveScripts = false,
    numberOfEpisodes = 230,
  }) {
    this.saveCSV = saveCSV;
    this.saveScripts = saveScripts;
    this.numberOfEpisodes = numberOfEpisodes;
  }

  private logProgress() {
    const lastScript = this.scripts[this.scripts.length - 1];
    const statusText = lastScript.failed
      ? `❌ Episode ${lastScript.episodeFormatted}/${this.numberOfEpisodes} failed to process`
      : `✅ Episode ${lastScript.episodeFormatted}/${this.numberOfEpisodes} successfully processed`;

    process.stdout.clearLine(0);
    process.stdout.write(
      `\r${statusText} [${Math.round(this.progress * 100)}%]`
    );
  }

  public async processScripts() {
    if (this.isProcessing) return;
    this.isProcessing = true;

    for (let episode = 1; episode <= this.numberOfEpisodes; episode++) {
      const episodeFormatted = episode.toString().padStart(3, "0");

      let processed: Script = {
        failed: true,
        episode,
        episodeFormatted,
        numberOfTokens: null,
        content: null,
      };

      try {
        const data = await processScript(episodeFormatted);
        processed = {
          ...processed,
          ...data,
          failed: false,
        };
      } catch (error) {}

      this.scripts.push(processed);
      this.logProgress();
    }

    if (this.saveCSV) {
      await this.saveCSVFile();
    }
    if (this.saveScripts) {
      await this.saveAllScripts();
    }
  }

  private async prepareScriptFolder() {
    await fs.mkdir(ScriptHandler.SCRIPT_FOLDER_PATH, { recursive: true });
  }

  private async saveAllScripts() {
    process.stdout.write("\rSaving scripts");
    await this.prepareScriptFolder();

    for (const script of this.scripts.filter((s) => !s.failed)) {
      await fs.writeFile(
        path.join(
          ScriptHandler.SCRIPT_FOLDER_PATH,
          `${script.episodeFormatted}.txt`
        ),
        script.content,
        "utf8"
      );
    }
  }

  private async saveCSVFile() {
    process.stdout.write("\rSaving CSV file");
    await this.prepareScriptFolder();

    const headerMap = {
      episode: "Episode",
      numberOfTokens: "Number of Tokens",
    };

    const rows = this.scripts
      .sort((a, b) => a.episode - b.episode)
      .map((script) =>
        Object.keys(headerMap)
          .map((key) => script[key] ?? "")
          .join(",")
      );

    const headerAsString = Object.values(headerMap).join(",") + "\n";

    await fs.writeFile(
      ScriptHandler.CSV_FILE_PATH,
      headerAsString + rows.join("\n"),
      "utf8"
    );
  }
}

export default ScriptHandler;
