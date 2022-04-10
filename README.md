# **PLEASE IGNORE** Solidity files analyzer

This is an N-API lib that exposes a single library, which takes a solidity source file's content, and returns its version pragmas and imports.

## API

```ts
export interface AnalysisResult {
  versionPragmas: Array<string>;
  imports: Array<string>;
}

export function analyze(input: string): AnalysisResult;
```

## TODO

1. Add tests for malformed version import statements where it still works
2. Add tests for malformed version pragma statements where it still works
3. Add tests for error recovery
