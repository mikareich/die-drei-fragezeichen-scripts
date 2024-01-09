import { PDFExtract, PDFExtractResult } from "pdf.js-extract";
import { encoding_for_model } from "tiktoken";

const pdfExtract = new PDFExtract();
const tokenEncoder = encoding_for_model("gpt-3.5-turbo");

const SKRIPT_URL = "https://www.rocky-beach.com/hoerspiel/skript/"; // +skript_xxx.pdf

export default async function processScript(episodeFormatted: string) {
  let data: PDFExtractResult;

  try {
    // fetch skript
    const res = await fetch(`${SKRIPT_URL}skript_${episodeFormatted}.pdf`);
    const buffer = (await res.arrayBuffer()) as Buffer;

    // extract pdf
    data = await pdfExtract.extractBuffer(buffer, {
      normalizeWhitespace: true,
      disableCombineTextItems: false,
    });
  } catch (error) {
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
}
