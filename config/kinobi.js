const {createFromIdls, renderJavaScriptVisitor} = require("@metaplex-foundation/kinobi");
const {join} = require("path");

// Instantiate Kinobi.

const kinobi = createFromIdls([
  join(__dirname, "..", "target", "idl","solana.json"),
]);

// Update the Kinobi tree using visitors...

// Render JavaScript.
const jsDir = join(__dirname, "clients", "js", "src", "generated");
kinobi.accept(renderJavaScriptVisitor(jsDir));
