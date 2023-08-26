import type {Config} from 'jest';

const config:Config = {
  modulePaths: ["<rootDir>", "src/"], // buscar modulos con relative path en src
  preset: "ts-jest", // configuración base 
  transform: {
    'node_modules/variables/.+\\.(j|t)sx?$': 'ts-jest', // permite imports en ts files
  },
};

module.exports = config;
