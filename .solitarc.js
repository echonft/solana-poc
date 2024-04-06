const path = require('path');
const programDir = path.join(__dirname, 'programs', 'solana');
const idlDir = path.join(__dirname, 'target', 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'solana',
  programId: '251VpQ2e7acPSqM4m7DRoUMpfX9mEtFXHjbYRx2C5JGX',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
