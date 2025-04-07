# ddf dataset generation

binary that runs on nas at processes ddf mp3s.

- context: mp3 files, ep id
- execution order:
    (# preparation)
    1. fetch script metadata
    2. read files and order them
    (# transciption)
    3. upload intro and mp3
    4. execute prompt
    (# post-processing)
    5. revise script
    6. split script into chunks
    (# publishing)
    7. upload to s3 as chunked
    8. generate vector and upload it

write a readme for this project

Here's a README.md for the Die Drei Fragezeichen Scripts project:

```markdown
# Die Drei Fragezeichen Scripts Generator

A Rust application that generates transcripts from Die Drei Fragezeichen (The Three Investigators) audio plays using Google's Gemini AI.

## Overview

This project automatically transcribes audio files from Die Drei Fragezeichen episodes into text scripts, maintaining character dialogues and ambient sounds. It uses Gemini's multimodal capabilities to analyze audio and generate accurate transcriptions while preserving the narrative structure.

## Features

- Audio file processing and ordering
- Automated transcription using Gemini AI
- Metadata fetching from dreimetadaten.de
- S3 storage integration for generated scripts
- Proper handling of character attribution and ambient sounds
- Multi-part episode support

## Prerequisites

- Rust (latest stable version)
- An API key for Google's Gemini AI
- AWS credentials (for S3 storage)
- Audio files from Die Drei Fragezeichen episodes

## Setup

1. Clone the repository
2. Create a `.env` file with the following variables:
```env
GEMINI_API_KEY=your_gemini_api_key
AWS_ACCESS_KEY_ID=your_aws_access_key
AWS_SECRET_ACCESS_KEY=your_aws_secret_key
AWS_REGION=your_aws_region
```

3. Install dependencies:
```bash
cargo build
```

## Usage

Run the application with an episode ID and the path to the audio files:

```bash
cargo run -- <episode_id> <path_to_audio_files>
```

Example:
```bash
cargo run -- 123 ./assets/audio/123/
```

## Output Format

The generated transcripts follow this format:

```
[character:NAME]: Dialog text

[ambient:SOUND_DESCRIPTION]
```

Example:
```
[character:Justus Jonas]: Kollegen, ich glaube ich habe eine Idee!

[ambient:Thunder and rain]

[character:Peter Shaw]: Hoffentlich eine gute...
```

## Project Structure

- `src/gemini/` - Gemini AI integration and file handling
- `src/metadata/` - Episode metadata fetching and processing
- `src/s3/` - AWS S3 storage integration
- `src/tracks/` - Audio file analysis and ordering
- `src/transcript/` - Transcript generation logic

## License

MIT

## Acknowledgments

- [dreimetadaten.de](https://dreimetadaten.de) for providing episode metadata
- Google's Gemini AI for transcription capabilities
- The Three Investigators (Die Drei Fragezeichen) series
```

This README provides a comprehensive overview of the project, its features, setup instructions, and usage guidelines. You may want to adjust specific details or add more sections based on your project's needs
