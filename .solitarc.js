const path = require('path');
const programDir = path.join(__dirname, 'programs', 'echo');
const idlDir = path.join(__dirname, 'target', 'idl');
const sdkDir = path.join(__dirname, 'target', 'sdk');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'echo',
  programId: 'GJtqQUC2mbdifTW2K9yXM9AmywCUST2Ah367Xdm3BfZu',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
