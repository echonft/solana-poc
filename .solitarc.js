const path = require('path');
const programDir = path.join(__dirname, 'programs', 'echo');
const idlDir = path.join(__dirname, 'target', 'idl');
const sdkDir = path.join(__dirname, 'target', 'sdk');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'echo',
  programId: '3xnJZhQ8U7whiBToRmQ3H4UWHmRWkJJEGZ2fLY2SmZ95',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
