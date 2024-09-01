# Docy

Simple CLI tool to extract JSDoc from a project and store it. Then inject back from the store.

### Why?

You're addicted to types and need auto-complete because you can't spell, but you're working on a pure JS repo and you can't commit JSDoc.

Or whatever your use case might be...

### Usage

`docy ex`:
Extract all JSDoc from project and store in a `.docy.json` store file.

`docy in`
Inject all JSDoc back into where it came from using the `docy.json` store file.

To avoid data loss or corruption, `docy` cannot run the same action back to back.
An extract must be followed by an inject!

**Note:** I recommend you run just `docy` or `docy in` for the fist time to generate a basic config file.

### Config

Look t the `docy.json` file, the fields should be self explanatory.

You can use this project's `docy.json` as an example.
