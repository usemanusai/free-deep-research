module.exports = {
  // Basic formatting
  semi: true,
  trailingComma: 'es5',
  singleQuote: true,
  doubleQuote: false,
  
  // Indentation
  tabWidth: 2,
  useTabs: false,
  
  // Line length
  printWidth: 80,
  
  // Bracket spacing
  bracketSpacing: true,
  bracketSameLine: false,
  
  // Arrow functions
  arrowParens: 'avoid',
  
  // JSX
  jsxSingleQuote: true,
  jsxBracketSameLine: false,
  
  // HTML
  htmlWhitespaceSensitivity: 'css',
  
  // Vue
  vueIndentScriptAndStyle: false,
  
  // End of line
  endOfLine: 'lf',
  
  // Embedded language formatting
  embeddedLanguageFormatting: 'auto',
  
  // Quote props
  quoteProps: 'as-needed',
  
  // Range formatting
  rangeStart: 0,
  rangeEnd: Infinity,
  
  // Parser
  requirePragma: false,
  insertPragma: false,
  
  // Prose wrap
  proseWrap: 'preserve',
  
  // Override for specific file types
  overrides: [
    {
      files: '*.json',
      options: {
        printWidth: 120,
        tabWidth: 2,
      },
    },
    {
      files: '*.md',
      options: {
        printWidth: 100,
        proseWrap: 'always',
        tabWidth: 2,
      },
    },
    {
      files: '*.yml',
      options: {
        tabWidth: 2,
        singleQuote: false,
      },
    },
    {
      files: '*.yaml',
      options: {
        tabWidth: 2,
        singleQuote: false,
      },
    },
    {
      files: '*.html',
      options: {
        printWidth: 120,
        tabWidth: 2,
        htmlWhitespaceSensitivity: 'ignore',
      },
    },
    {
      files: '*.css',
      options: {
        printWidth: 120,
        tabWidth: 2,
      },
    },
    {
      files: '*.scss',
      options: {
        printWidth: 120,
        tabWidth: 2,
      },
    },
    {
      files: '*.less',
      options: {
        printWidth: 120,
        tabWidth: 2,
      },
    },
    {
      files: '*.tsx',
      options: {
        jsxSingleQuote: true,
        jsxBracketSameLine: false,
      },
    },
    {
      files: '*.jsx',
      options: {
        jsxSingleQuote: true,
        jsxBracketSameLine: false,
      },
    },
    {
      files: 'package.json',
      options: {
        printWidth: 120,
        tabWidth: 2,
      },
    },
    {
      files: '*.toml',
      options: {
        tabWidth: 2,
      },
    },
    {
      files: '*.rs',
      options: {
        tabWidth: 4,
        printWidth: 100,
      },
    },
  ],
};
